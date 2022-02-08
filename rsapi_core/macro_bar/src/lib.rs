use proc_macro::TokenStream;
use pretty_env_logger;



#[proc_macro_attribute]
pub fn get(args: TokenStream, input: TokenStream) -> TokenStream {
    let x = format!(r#"
       async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {{
            Ok(Response::new("Hello, World".into()))
          }}

    "#);

    x.parse().expect("Generated invalid tokens")
}