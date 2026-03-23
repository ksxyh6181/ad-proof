use salvo::conn::TcpListener;
use salvo::{Listener, Server};

mod config;
mod controller;
mod middleware;
mod router;
mod utils;

const IP: &str = "0.0.0.0";
const PORT: i32 = 8090;

#[tokio::main]
async fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    log::warn!("starting selective disclosure vc demo on {}:{}", IP, PORT);

    let _ = config::AppConfig::load("config/app_config.json").ok();

    let service = router::init_service();
    let address = format!("{}:{}", IP, PORT);
    Server::new(TcpListener::new(address).bind().await).serve(service).await;
}
