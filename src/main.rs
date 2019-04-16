use actix_web::{App, server, HttpRequest};

fn index(_req: &HttpRequest) -> &'static str {
    "Hello world!"
}

fn main() {
    server::new(|| App::new().resource("/a/pawan", |res| res.f(index)))
        .bind("127.0.0.1:8080").unwrap().run();
}
