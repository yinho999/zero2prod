use secrecy::ExposeSecret;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

use sqlx::PgPool;
use std::net::TcpListener;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Create a subscriber
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);

    // Initialize the subscriber
    init_subscriber(subscriber);

    // Panic if we can't read configuaration
    let configuration = get_configuration().expect("Failed to read configuration.");

    let connection_pool =
        PgPool::connect_lazy(configuration.database.connection_string().expose_secret())
            .expect("Failed to connect to Postgres database.");

    // We have removed the hard-coded `8000` - it's now coming from our settings!
    let address = format!("127.0.0.1:{}", configuration.application_port);

    let listener = TcpListener::bind(&address).expect("Fail to bind address");
    println!("Listening on address {}", &address);

    run(listener, connection_pool)?.await
}
