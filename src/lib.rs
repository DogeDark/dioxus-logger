use log::{LevelFilter, SetLoggerError};

/// The log level. E.g. ``INFO``
const LEVEL: &str = "{LEVEL}";
/// The file's path or crate name. E.g. ``dioxus-testing``
const PATH: &str = "{PATH}";
/// The arguments passed through the log macro.
/// E.g. the args from ``info!("hello")`` will be ``"hello"``
const ARGS: &str = "{ARGS}";
/// A timestamp of when the log was generated.
const TIMESTAMP: &str = "{TIMESTAMP}";

/// The primary logging struct that contains all runtime configuration.
pub struct DioxusLogger {
    level_filter: LevelFilter,
    format: &'static str,
}

impl DioxusLogger {
    /// Create a new [`DioxusLogger`] struct to configure and build.
    pub fn new(level_filter: LevelFilter) -> Self {
        let format = "[{LEVEL}] {PATH} - {ARGS}";

        #[cfg(feature = "timestamps")]
        let format = "[{TIMESTAMP} | {LEVEL}] {PATH} - {ARGS}";

        Self {
            level_filter,
            format,
        }
    }

    /// Builds and initializes the logger with [`log`]
    pub fn build(self) -> Result<(), SetLoggerError> {
        let level = self.level_filter.clone();
        log::set_boxed_logger(Box::new(self)).map(|()| log::set_max_level(level))
    }

    /// Allows you to define a custom format.
    ///
    /// The available options are `{LEVEL}`, `{PATH}`, `{ARGS}` and `{TIMESTAMP}`
    ///
    /// Providing the format of `[{LEVEL}] {PATH} - {ARGS}]` will return something like `[INFO] dioxus_testing - this is my log message`
    pub fn use_format(self, format: &'static str) -> Self {
        Self {
            level_filter: self.level_filter,
            format
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

        // This is cursed
        let formatted = self
            .format
            // LEVEL
            .replace(LEVEL, record.level().as_str())
            .to_owned()
            // PATH
            .replace(PATH, record.module_path().unwrap_or(""))
            .to_owned()
            // ARGS
            .replace(ARGS, record.args().to_string().as_str());

        #[cfg(feature = "timestamps")]
        let formatted = format_timestamp(formatted);

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

#[cfg(feature = "timestamps")]
fn format_timestamp(formatted: String) -> String {
    let timestamp = time::OffsetDateTime::now_utc();
    formatted
        .to_owned()
        .replace(TIMESTAMP, timestamp.to_string().as_str())
}

/// Initialize `log` and `dioxus-logger` with a specified filter.
/// For more advanced configuration, build the logger through the [`DioxusLogger`] struct.
pub fn init(level_filter: LevelFilter) -> Result<(), SetLoggerError> {
    DioxusLogger::new(level_filter).build()
}
