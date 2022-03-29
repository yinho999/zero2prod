use crate::routes;
use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgConnection;
use std::net::TcpListener;

// Notice the different signature!
// We return `Server` on the happy path and we dropped the `async` keyword
// We have no .await call, so it is not needed anymore.
pub fn run(listener: TcpListener, connection: PgConnection) -> Result<Server, std::io::Error> {
    // Wrap the connection in a smart pointer
    let connection = web::Data::new(connection);

    let server = HttpServer::new(move || {
        App::new() // .route("/", web::get().to(greet))
            // .route("/{name}", web::get().to(greet))
            .route("/health_check", web::get().to(routes::health_check))
            .route("/subscribe", web::post().to(routes::subscribe))
            // Register a PgConnection as part of our application state
            // Get a pointer copy and attach it to the application stat
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();

    // No await here
    Ok(server)
}
