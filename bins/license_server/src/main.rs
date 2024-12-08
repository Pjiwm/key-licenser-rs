use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

const GIT_VERSION: &str = git_version::git_version!();
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    setup_logger();
    info!("Running license server on git hash {}", GIT_VERSION);
    info!("Running license server on application version {}", VERSION);
}

fn setup_logger() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}
