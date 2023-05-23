use anyhow::Result;
use spin_sdk::{
    config,
    http::{Request, Response},
    http_component,
};

/// A simple Spin HTTP component.
#[http_component]
fn hello_world(req: Request) -> Result<Response> {
    println!("{:?}", req.headers());
    let host = config::get("host").expect("Failed to acquire host");
    println!("Host is {host}");
    Ok(http::Response::builder()
        .status(200)
        .header("foo", "bar")
        .body(Some("Hello, Fermyon!\n".into()))?)
}
