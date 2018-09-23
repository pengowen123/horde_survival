#[macro_use]
extern crate quick_error;
#[macro_use]
extern crate slog;
extern crate common;
extern crate directories;
extern crate horde_survival;
extern crate ron;
extern crate slog_async;
extern crate slog_term;

use directories::ProjectDirs;
use slog::Drain;

use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;

use common::config;

const CONFIG_FILE_NAME: &str = "settings.ron";

quick_error! {
    /// The error type for the `horde_survival` crate
    #[derive(Debug)]
    enum Error {
        Deserialize(e: ron::ser::Error) {
            display("Error deserializing `Config`: {}", e)
            from()
        }
        Serialize(e: ron::de::Error) {
            display("Error serializing `Config`: {}", e)
            from()
        }
        Io(e: (io::Error, String)) {
            display("Error loading file from path `{}`: {}", e.1, e.0)
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

/// Returns the path of the project directory selected by `f`, creating it if does not exist
fn get_project_dir_path<F>(f: F) -> Result<PathBuf, Error>
where
    F: FnOnce(&ProjectDirs) -> PathBuf,
{
    let project_dirs = match ProjectDirs::from("", "horde_survival", "horde_survival") {
        Some(d) => d,
        None => return Err(Error::ProjectDir),
    };
    let path = f(&project_dirs);

    if let Err(e) = fs::DirBuilder::new().recursive(true).create(&path) {
        if let io::ErrorKind::AlreadyExists = e.kind() {
        } else {
            let path = path
                .to_str()
                .expect("Project directory path contained invalid unicode")
                .to_string();

            return Err(Error::Io((e, path)));
        }
    }

    Ok(path)
}

/// Returns the path of the config directory, creating it if does not exist
fn get_config_dir_path() -> Result<PathBuf, Error> {
    get_project_dir_path(|dirs| dirs.config_dir().to_owned())
}

fn get_default_assets_path() -> Result<PathBuf, Error> {
    get_project_dir_path(|dirs| dirs.data_dir().join("assets"))
}

/// Loads a `Config` from the configuration file
fn load_config() -> Result<config::Config, Error> {
    let config_file_path = get_config_dir_path()?.join(CONFIG_FILE_NAME);
    let config_file_path_str = config_file_path
        .to_str()
        .expect("Config file path contained invalid unicode");

    let mut file = fs::File::open(&config_file_path)
        .map_err(|e| Error::Io((e, config_file_path_str.to_string())))?;

    let mut data = String::new();
    file.read_to_string(&mut data)
        .map_err(|e| Error::Io((e, config_file_path_str.to_string())))?;

    let config = ron::de::from_str(&data)?;

    Ok(config)
}

/// Writes the provided `Config` to the configuration file
fn save_config(config: config::Config) -> Result<(), Error> {
    let serialized = ron::ser::to_string_pretty(&config, ron::ser::PrettyConfig::default())?;

    let config_file_path = get_config_dir_path()?.join(CONFIG_FILE_NAME);
    let config_file_path_str = config_file_path
        .to_str()
        .expect("Config file path contained invalid unicode");

    let mut file = fs::File::create(&config_file_path)
        .map_err(|e| Error::Io((e, config_file_path_str.to_string())))?;

    file.write_all(serialized.as_bytes())
        .map_err(|e| Error::Io((e, config_file_path_str.to_string())))
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
    let cli_config =
        config::CommandLineConfig::new(get_default_assets_path().unwrap_or_else(|e| {
            error!(logger, "Error loading default assets path: {}", e;);
            panic!(common::CRASH_MSG);
        }));

    let new_config = horde_survival::run(config, cli_config, logger.clone());

    save_config(new_config).unwrap_or_else(|e| {
        error!(logger, "Error writing to configuration file: {}", e;);
    });
}
