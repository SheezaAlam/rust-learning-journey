use tracing::info;
use tracing_subscriber;

pub fn run() {
    // init subscriber (controls formatting + filtering)
    tracing_subscriber::fmt::init();

    println!("(println) Server started on 127.0.0.1:8080");
    info!("(tracing) Server started on 127.0.0.1:8080");
}
