use proc_macro;
use proc_macro2::TokenStream;
use quote::quote;
use serde::Deserialize;
use syn::ItemFn;

#[derive(Deserialize, Debug)]
enum MethodType {
    GET,
    POST,
}

#[derive(Deserialize, Debug)]
struct Metadata {
    method: MethodType,
    path: String,
}

#[proc_macro_attribute]
pub fn endpoint(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    do_endpoint(attr.into(), item.into()).into()
}

fn do_endpoint(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Deserialize metadata token stream and log it
    let metadata = serde_tokenstream::from_tokenstream::<Metadata>(&attr).unwrap();
    eprintln!("{:?}", metadata);

    let ast: ItemFn = syn::parse2(item.clone()).unwrap();
    let args = ast.sig.inputs.iter().map(|arg| match arg {
        syn::FnArg::Typed(pat) => Some(pat.ty.as_ref()),
        _ => None,
    });
    for a in args {
        if let Some(t) = a {
            eprintln!("{:?}", t);
        }
    }

    let ts = quote! {
        #item
    };
    ts.into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;
    use syn::{ItemFn, Signature, Visibility};

    #[test]
    fn test_busted_function() {
        let f = quote! {
            #[lol]
            fn foo<'a, T>(parameter: &'a T) -> &'a T {
                return parameter;
            }
        };
        let ast: ItemFn = syn::parse2(f).unwrap();

        println!("{:#?}", ast);

        assert!(ast.attrs.is_empty());
        assert_eq!(ast.vis, Visibility::Inherited);
    }
}
