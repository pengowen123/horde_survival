//! Logger initialization

use slog::{self, Drain};
use slog_term;
use slog_async;

/// Initializes a `slog::Logger` and returns it
pub fn init_logger() -> slog::Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    slog::Logger::root(drain, o!())
}
