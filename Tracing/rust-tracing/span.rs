use tracing::{info, info_span};
use tracing_subscriber;

pub fn run() {
    tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:8080";

    let span = info_span!("connection", peer = %addr);
    let _enter = span.enter();

    info!("handshake started");
    info!("handshake success");
}
