use tracing::{
    subscriber::{set_global_default, SetGlobalDefaultError},
    Level,
};

pub use tracing;

/// Initialize `dioxus-logger` with a specified max filter.
/// Generally it is best to initialize the logger before launching your Dioxus app.
/// Works on Web, Desktop, Fullstack, and Liveview.
///
/// # Example
///
/// ```rust,no_run
/// use dioxus::prelude::*;
/// use dioxus_logger::tracing::{Level, info};
///
/// fn main() {
///     dioxus_logger::init(Level::INFO).expect("logger failed to init");
///     launch(App);
/// }
///
/// #[component]
/// fn App() -> Element {
///     info!("App rendered");
///     rsx! {
///         p { "hi" }
///     }
/// }
/// ```
pub fn init(level: Level) -> Result<(), SetGlobalDefaultError> {
    #[cfg(target_arch = "wasm32")]
    {
        use tracing_subscriber::layer::SubscriberExt;
        use tracing_subscriber::Registry;

        let layer_config = tracing_wasm::WASMLayerConfigBuilder::new()
            .set_max_level(level)
            .build();
        let layer = tracing_wasm::WASMLayer::new(layer_config);
        let reg = Registry::default().with(layer);

        console_error_panic_hook::set_once();
        set_global_default(reg)
    }

    #[cfg(not(target_arch = "wasm32"))]
    return {
        let sub = tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(level)
            .finish();

        set_global_default(sub)
    };
}
