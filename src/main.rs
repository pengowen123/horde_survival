extern crate horde_survival;
#[macro_use]
extern crate quick_error;
#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_async;
extern crate serde;
extern crate toml;
extern crate directories;

use slog::Drain;
use directories::ProjectDirs;

use std::fs;
use std::path::PathBuf;
use std::io::{self, Read, Write};

use horde_survival::config;

const CONFIG_FILE_NAME: &str = "settings.toml";

quick_error! {
    /// An error while loading or saving a `Config`
    #[derive(Debug)]
    enum ConfigError {
        Deserialize(e: toml::ser::Error) {
            display("Error deserializing `Config`: {}", e)
            from()
        }
        Serialize(e: toml::de::Error) {
            display("Error serializing `Config`: {}", e)
            from()
        }
        Io(e: (io::Error, String)) {
            display("Error loading configuration file from path `{}`: {}", e.1, e.0)
            from()
        }
        ProjectDir {
            display("Error getting project directory")
            from()
        }
    }
}

/// Initializes the logger used by horde_survival
fn init_logger() -> slog::Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    slog::Logger::root(drain, o!())
}

/// Returns `Config::default()`, and warns that the default configuration is being used
fn get_default_config(log: &slog::Logger) -> config::Config {
    warn!(log, "Default configuration will be used";);
    Default::default()
}

/// Returns the path of the configuration directory, creating it if does not exist
fn get_config_dir_path() -> Result<PathBuf, ConfigError> {
    let project_dirs = match ProjectDirs::from("", "horde_survival", "horde_survival") {
        Some(d) => d,
        None => {
            return Err(ConfigError::ProjectDir)
        }
    };
    let config_dir = project_dirs.config_dir().to_owned();

    if let Err(e) = fs::DirBuilder::new().create(&config_dir) {
        if let io::ErrorKind::AlreadyExists = e.kind() {
        } else {
            let config_dir = config_dir
                .to_str()
                .expect("Config dir path contained invalid unicode")
                .to_string();

            return Err(ConfigError::Io((e, config_dir.clone())))
         }
    }
    
    Ok(config_dir)
}

/// Loads a `Config` from the configuration file
fn load_config() -> Result<config::Config, ConfigError> {
    let config_file_path = get_config_dir_path()?.join(CONFIG_FILE_NAME);
    let config_file_path_str = config_file_path.to_str()
        .expect("Config file path contained invalid unicode");

    let mut file = fs::File::open(&config_file_path)
        .map_err(|e| ConfigError::Io((e, config_file_path_str.to_string())))?;

    let mut data = String::new();
    file.read_to_string(&mut data)
        .map_err(|e| ConfigError::Io((e, config_file_path_str.to_string())))?;

    let config = toml::from_str(&data)?;

    Ok(config)
}

/// Writes the provided `Config` to the configuration file
fn save_config(config: config::Config) -> Result<(), ConfigError> {
    let serialized = toml::to_string_pretty(&config)?;

    let config_file_path = get_config_dir_path()?.join(CONFIG_FILE_NAME);
    let config_file_path_str = config_file_path.to_str()
        .expect("Config file path contained invalid unicode");

    let mut file = fs::File::create(&config_file_path)
        .map_err(|e| ConfigError::Io((e, config_file_path_str.to_string())))?;

    file.write_all(serialized.as_bytes())
        .map_err(|e| ConfigError::Io((e, config_file_path_str.to_string())))
}

/// Attempts to load a `Config` from the configuration file, returning `Default::default()` if an
/// error occurs
fn load_config_or_default(log: &slog::Logger) -> config::Config {
    match load_config() {
        Ok(c) => c,
        Err(e) => {
            error!(log, "Error loading configuration file: {}", e;);
            get_default_config(log)
        }
    }
}

fn main() {
    let logger = init_logger();
    let config = load_config_or_default(&logger);
    let new_config = horde_survival::run(config, logger.clone());

    save_config(new_config)
        .unwrap_or_else(|e| {
            error!(logger, "Error writing to configuration file: {}", e;);
        })
}
