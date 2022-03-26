use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[actix_rt::main]
pub async fn main() -> std::io::Result<()> {
    // Panic if we can't read configuaration
    let configuration = get_configuration().expect("Failed to read configuration.");

    // We have removed the hard-coded `8000` - it's now coming from our settings!

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(&address).expect("Fail to bind address");

    println!("Listening on address {}", &address);

    run(listener)?.await
}
