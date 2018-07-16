extern crate horde_survival;
#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_async;

use slog::Drain;

/// Initializes the logger used by horde_survival
pub fn init_logger() -> slog::Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    slog::Logger::root(drain, o!())
}

fn main() {
    let logger = init_logger();
    horde_survival::run(logger);
}
