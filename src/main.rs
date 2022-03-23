use std::net::TcpListener;
use zero2prod::run;
#[actix_rt::main]
pub async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Fail to bind address");
    let port = listener.local_addr().unwrap().port();
    println!("Listening on port {}", port);

    run(listener)?.await
}
