use actix_web::{server, App, HttpRequest, Result, actix};
use actix_web::middleware::session::{RequestSession, SessionStorage, CookieSessionBackend};
use std::env;

fn index(req: &HttpRequest) -> Result<&'static str> {

    if let Some(count) = req.session().get::<i32>("counter")? {
        println!("SESSION value: {}", count);
        req.session().set("counter", count+1)?;
    } else {
        req.session().set("counter", 1)?;
    }

    Ok("Welcome!")

}

fn main() {
    let sys = actix::System::new("Session-management");
    server::new(
        || App::new().middleware(
            SessionStorage::new(
                CookieSessionBackend::signed(&[0; 32])
                    .secure(false)
            )))
        .bind("127.0.0.1:59881").unwrap()
        .start();
    let _ = sys.run();
}