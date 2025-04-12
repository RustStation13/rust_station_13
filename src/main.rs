use std::{thread::sleep, time::Duration};

use args::Arguments;
use clap::Parser;
use tracing::{debug, Level};

mod bevy_main;
mod args;

fn main() {
    let args = Arguments::parse();
    
    // Configure logger.
    tracing_subscriber::fmt()
        .with_ansi(true)
        .with_thread_ids(true)
        .with_max_level(args.log_level)
        .init();

    // Provide a debug warning.
    debug!("Alert! Debug level is enabled. Log files may be excessively large. If this was unintended please ctrl+c right now before its too late, proceeding in 5s...");
    if args.log_level == Level::DEBUG {
        sleep(Duration::from_secs(5));
    }
    debug!("Proceeding despite warning, prepare for large log files.");
}
