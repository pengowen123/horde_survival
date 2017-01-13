//! A simple logging implementation specific to `horde_survival`

// TODO: Replace this module with slog

#[macro_use]
pub mod log_utils;

pub use self::log_utils::*;

use log::*;
use time;

use platform::misc::NEWLINE;
use consts::log_str::*;

use std::fs::File;
use std::io::Write;
use std::sync::Mutex;

/// Initializes the logger
pub fn init() {
    let file = File::create(LOG_FILE)
        .unwrap_or_else(|e| panic!("Failed to created log file: {}", e));

    // Set the logger
    let result = set_logger(|max_log_level| {
        max_log_level.set(LogLevelFilter::Trace);
        Box::new(HSLogger { file: Mutex::new(file) })
    });

    if let Err(e) = result {
        panic!("Failed to initialize logger: {}", e);
    }
}

/// A logger
pub struct HSLogger {
    file: Mutex<File>,
}

impl Log for HSLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        // Filter out messages from gfx_device_gl to avoid spamming the log
        !(metadata.target().starts_with("gfx_device_gl"))
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            let mut file = self.file.lock().expect("Failed to acquire lock on log file");
            let time = time::now();

            // Write full paths if debug assertions (usually when building in debug mode)
            if cfg!(debug_assertions) {
                    write!(file,
                           "{:02}:{:02}:{:02}.{:03} [{}] {}: {}{}",
                           time.tm_hour,
                           time.tm_min,
                           time.tm_sec,
                           time.tm_nsec / 1_000_000,
                           record.level(),
                           record.target(),
                           record.args(),
                           NEWLINE)
                } else {
                    write!(file,
                           "{:02}:{:02}:{:02}.{:03} [{}]: {}{}",
                           time.tm_hour,
                           time.tm_min,
                           time.tm_sec,
                           time.tm_nsec / 1_000_000,
                           record.level(),
                           record.args(),
                           NEWLINE)
                }
                .unwrap_or_else(|e| panic!("Failed to write to log file: {}", e));
        }
    }
}
