#[macro_use]
pub mod log_utils;

pub use self::log_utils::*;

use log::*;
use time;

use platform::NEWLINE;
use consts::log_str::*;

use std::fs::File;
use std::io::Write;
use std::sync::Mutex;

pub fn init() {
    let file = match File::create(LOG_FILE) {
        Ok(f) => f,
        Err(e) => panic!("Failed to created log file: {}", e),
    };

    let result = set_logger(|max_log_level| {
        max_log_level.set(LogLevelFilter::Trace);
        Box::new(HSLogger { file: Mutex::new(file) })
    });

    if let Err(e) = result {
        panic!("Failed to initialize logger: {}", e);
    }
}

pub struct HSLogger {
    file: Mutex<File>,
}

impl Log for HSLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        !(metadata.target().starts_with("gfx_device_gl"))
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            let mut file = self.file.lock().expect("Failed to acquire lock on log file");
            let time = time::now();

            let result = write!(file, "{:02}:{:02}:{:02} [{}] {}: {}{}",
                                time.tm_hour,
                                time.tm_min,
                                time.tm_sec,
                                record.level(),
                                record.target(),
                                record.args(),
                                NEWLINE);

            if let Err(e) = result {
                panic!("Failed to write to log file: {}", e);
            }
        }
    }
}
