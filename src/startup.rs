use actix_web::{dev::Server, web, App, HttpServer};
use std::net::TcpListener;

use crate::routes;

// Notice the different signature!
// We return `Server` on the happy path and we dropped the `async` keyword
// We have no .await call, so it is not needed anymore.
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new() // .route("/", web::get().to(greet))
            // .route("/{name}", web::get().to(greet))
            .route("/health_check", web::get().to(routes::health_check))
            .route("/subscribe", web::post().to(routes::subscribe))
    })
    .listen(listener)?
    .run();

    // No await here
    Ok(server)
}
