pub mod bindings {
    use super::T;

    wit_bindgen::generate!({
        path: "./wit",
        world: "example:example/example",
        exports: {
            "wasi:http/incoming-handler": T,
        },
    });
}

use bindings::wasi::http::types::{IncomingRequest, ResponseOutparam};

struct T;

impl bindings::exports::wasi::http::incoming_handler::Guest for T {
    fn handle(_request: IncomingRequest, outparam: ResponseOutparam) {
        let hdrs = bindings::wasi::http::types::Headers::new(&[]);
        let resp = bindings::wasi::http::types::OutgoingResponse::new(200, &hdrs);
        let body = resp.write().expect("outgoing response");

        bindings::wasi::http::types::ResponseOutparam::set(outparam, Ok(resp));

        let out = body.write().expect("outgoing stream");
        out.blocking_write_and_flush(b"Hello from a proxy in the http-wasi world!")
            .expect("writing response");

        drop(out);
        bindings::wasi::http::types::OutgoingBody::finish(body, None);
    }
}
