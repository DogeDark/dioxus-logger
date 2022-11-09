use log::{LevelFilter, SetLoggerError};

/// The log level. E.g. ``INFO``
const LEVEL: &str = "{LEVEL}";
/// The file's path or crate name. E.g. ``dioxus-testing``
const PATH: &str = "{PATH}";
/// The arguments passed through the log macro.
/// E.g. the args from ``info!("hello")`` will be ``"hello"``
const ARGS: &str = "{ARGS}";

/// The primary logging struct that contains all runtime configuration.
pub struct DioxusLogger {
    level_filter: LevelFilter,
    format: &'static str,
}

impl DioxusLogger {
    /// Create a new [`DioxusLogger`] struct to configure and build.
    pub fn new(level_filter: LevelFilter) -> Self {
        Self {
            level_filter,
            format: "[{LEVEL}] {PATH} - {ARGS}]",
        }
    }

    /// Builds and initializes the logger with [`log`]
    pub fn build(self) -> Result<(), SetLoggerError> {
        let level = self.level_filter.clone();
        log::set_boxed_logger(Box::new(self)).map(|()| log::set_max_level(level))
    }

    /// Allows you to define a custom format.
    /// 
    /// The available options are `{LEVEL}`, `{PATH}` and `{ARGS}`
    /// 
    /// Providing the format of `[{LEVEL}] {PATH} - {ARGS}]` will return something like `[INFO] dioxus_testing - this is my log message`
    pub fn use_format(&mut self, format: &'static str) {
        self.format = format;
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

        /*
        // The old format, not customizable
        let formatted = format!(
            "[{}] {} - {}",
            record.level(),
            record.module_path().unwrap_or(""),
            record.args()
        );
        */

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
/// For more advanced configuration, build the logger through the [`DioxusLogger`] struct.
pub fn init(level_filter: LevelFilter) -> Result<(), SetLoggerError> {
    DioxusLogger::new(level_filter).build()
}
