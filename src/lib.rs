use log::{LevelFilter, SetLoggerError};
//use std::path::PathBuf;

/*pub struct HttpConfig {
    method: reqwest::Method,
    url: String,
    authorization_token: String,
}

pub enum OutputType {
    Json,
    Stdio,
}

pub struct FileSystemConfig {
    file_name: String,
    output_type: OutputType,
    output_path: PathBuf,
}*/

/// The primary logging struct that contains all runtime configuration.
pub struct DioxusLogger {
    //http_config: Option<HttpConfig>,
    //file_system_config: Option<FileSystemConfig>,
    level_filter: LevelFilter,
}

impl DioxusLogger {
    pub fn new(
        level_filter: LevelFilter,
        //file_system_config: Option<FileSystemConfig>,
        //http_config: Option<HttpConfig>,
    ) -> Self {
        Self {
            //http_config,
            //file_system_config,
            level_filter,
        }
    }
}

impl log::Log for DioxusLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.level_filter
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let formatted = format!("[{}] {} - {}", record.level(), record.module_path().unwrap_or(""), record.args());

        #[cfg(all(
            not(target_family = "wasm"),
            not(target_family = "android"),
            not(target_family = "ios")
        ))]
        println!("{formatted}");

        #[cfg(target_family = "wasm")]
        web_sys::console::log_1(&formatted.into());
    }

    fn flush(&self) {}
}

/// Initialize `log` and `dioxus-logger` with a specified filter.
pub fn init(level_filter: LevelFilter) -> Result<(), SetLoggerError> {
    let logger = DioxusLogger::new(level_filter);
    log::set_boxed_logger(Box::new(logger)).map(|()| log::set_max_level(level_filter))
}
