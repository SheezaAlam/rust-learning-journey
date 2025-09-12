use tracing::{info, warn, error, debug, trace};
use tracing_subscriber;

pub fn run() {
    tracing_subscriber::fmt::init();

    info!("app started (INFO = normal updates)");
    debug!("debugging info (DEBUG = detailed dev logs)");
    warn!("caution: using default config (WARN = yellow)");
    error!("something went wrong (ERROR = red)");
    trace!("trace event (TRACE = very low-level, like packet dumps)");
}
