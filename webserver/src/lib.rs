use bytes::BytesMut;
use futures::lock::Mutex;
use hyper::Body;
use hyper::Request;
use hyper::Response;
use std::collections::HashMap;
use std::convert::Infallible;
use tokio::io::ReadHalf;

use std::future::Future;
use std::pin::Pin;
use std::{net::SocketAddr, sync::Arc};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

pub use webserver_macro::endpoint;

pub enum MethodType {
    DELETE,
    GET,
    PATCH,
    POST,
    PUT,
}

pub struct Server {
    addr: SocketAddr,
    router: Arc<Mutex<Router>>,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

impl Server {
    pub fn new(addr: SocketAddr, router: Router) -> Server {
        Server {
            addr,
            router: Arc::new(Mutex::new(router)),
        }
    }

    pub async fn serve(&mut self) -> Result<()> {
        let listener = TcpListener::bind(self.addr).await?;

        loop {
            let (socket, _) = listener.accept().await?;
            let router = self.router.clone();

            tokio::spawn(async move {
                Self::process(socket, router).await?;
                Ok::<_, Error>(())
            });
        }
    }

    async fn process(stream: TcpStream, router: Arc<Mutex<Router>>) -> Result<()> {
        let (rd, mut wr) = io::split(stream);
        let mut router = router.lock().await;

        let request = Self::parse_request(rd).await?;
        let path = request.uri().to_string();

        let mut response = router.route_match(&path)(request).await?;

        tokio::spawn(async move {
            let byte_response = hyper::body::to_bytes(response.body_mut()).await.unwrap();
            wr.write_all(&byte_response[..]).await?;

            Ok::<_, io::Error>(())
        });

        Ok(())
    }

    async fn parse_request(stream: ReadHalf<TcpStream>) -> Result<Request<Body>> {
        let mut buffer = BytesMut::with_capacity(4 * 1024);
        BufReader::new(stream).read_buf(&mut buffer).await?;

        let mut headers = [httparse::EMPTY_HEADER; 64];
        let mut buf_request = httparse::Request::new(&mut headers);
        buf_request.parse(&buffer)?;

        let mut request = Request::builder()
            .method(buf_request.method.unwrap())
            .uri(buf_request.path.unwrap())
            .version(hyper::Version::HTTP_11);

        for header in buf_request.headers {
            request = request.header(header.name, header.value);
        }

        Ok(request.body(Body::empty()).unwrap())
    }
}

type Service = Box<
    dyn Fn(
            Request<Body>,
        ) -> Pin<
            Box<dyn Future<Output = std::result::Result<Response<Body>, Infallible>> + Send + Sync>,
        > + Send
        + Sync,
>;

pub struct Router {
    count: usize,
    inner: HashMap<String, Service>,
}

impl Default for Router {
    fn default() -> Self {
        Router::new()
    }
}

impl Router {
    pub fn new() -> Self {
        let map: HashMap<String, Service> = HashMap::new();
        Router {
            count: 0,
            inner: map,
        }
    }

    pub fn route<F>(
        &mut self,
        path: &str,
        service: impl Fn(Request<Body>) -> F + Send + Sync + 'static,
    ) where
        F: Future<Output = std::result::Result<Response<Body>, Infallible>> + Sync + Send + 'static,
    {
        self.inner.insert(
            path.to_string(),
            Box::new(move |req| Box::pin(service(req))),
        );
        self.count += 1;
    }

    pub fn route_match(&mut self, path: &str) -> &mut Service {
        let valid_path = self.inner.get_mut(path).is_some();

        if !valid_path {
            return self.inner.get_mut("*").unwrap();
        }

        self.inner.get_mut(path).unwrap()
    }
}
