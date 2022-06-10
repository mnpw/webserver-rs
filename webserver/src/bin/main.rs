use std::convert::Infallible;
use std::net::SocketAddr;

use hyper::*;
use webserver::{Result, Router, Server};

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = Router::new();
    app.route("*", echo);
    app.route("/welcome", welcome);

    let mut server = Server::new(SocketAddr::from(([127, 0, 0, 1], 3000)), app);

    server.serve().await?;

    Ok(())
}

// #[endpoint {
//     method = GET,
//     path = "/*",
// }]
async fn echo(req: Request<Body>) -> std::result::Result<Response<Body>, Infallible> {
    Ok(Response::new(req.uri().to_string().into()))
}

// #[endpoint {
//     method = GET,
//     path = "/welcome",
// }]
async fn welcome(_req: Request<Body>) -> std::result::Result<Response<Body>, Infallible> {
    Ok(Response::new("Welcome to the hacky server".into()))
}
