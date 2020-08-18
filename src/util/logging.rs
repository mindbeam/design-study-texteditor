use std::fs::OpenOptions;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub fn init_logging() {
    fn mkwriter() -> impl std::io::Write {
        let log_path = "./editor.log";

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_path)
            .unwrap();
        file
    }

    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        // completes the builder.
        .with_writer(mkwriter)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}
