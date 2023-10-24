use spin_sdk::http::{IntoResponse, Request};
use spin_sdk::http_component;

/// A simple Spin HTTP component.
#[http_component]
fn handle_rust(_req: Request) -> anyhow::Result<impl IntoResponse> {
    Ok(http::Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body("Hello, from a Fermyon Spin component")?)
}
