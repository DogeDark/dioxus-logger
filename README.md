<div align="center">
  <h1>📡 Dioxus Logger 🛰️</h1>
  <p><strong>A logging utility to provide a standard interface whether you're targetting web, desktop, or mobile with Dioxus.</strong></p>
</div>

<div align="center">
  <!-- Crates version -->
  <a href="https://crates.io/crates/dioxus-logger">
    <img src="https://img.shields.io/crates/v/dioxus-logger.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/dioxus-logger">
    <img src="https://img.shields.io/crates/d/dioxus-logger.svg?style=flat-square"
      alt="Download" />
  </a>
  <!-- docs -->
  <a href="https://docs.rs/dioxus-logger">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
</div>

-----

`dioxus-logger` is a (planned-to-be) feature-rich logger for [Dioxus](https://dioxuslabs.com/) that uses the [log](https://crates.io/crates/log) crate to provide a standard interface for all your logging needs.

**Current & Planned Features**
- [x] stdio logging (Web, Desktop) - Mobile to come
- [x] Custom log format - Basic Implementation Finished
- [x] Timestamps 
- [ ] Sending logs over HTTP to an API
- [ ] Logging to a file
- [ ] Logging to [Sentry](https://sentry.io/)?

**This library is not finished. Breaking changes may occur.**
```rust
use dioxus::prelude::*;
use log::{info, LevelFilter};

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    info!("my app component just rendered!");
    cx.render(rsx!(p { "Hello, Dioxus!" }))
}
```

## Platform Support
Dioxus logger will eventually support every target that Dioxus does. Currently only web and desktop platforms are supported.

## Installation
You can add `dioxus-logger` to your application by adding it to your dependencies.
```toml
[dependencies]
dioxus-logger = "x.x.x"
```

To use dioxus-logger as is, add this line before launching Dioxus, replacing ``LevelFilter::Info`` with your preferred log level.
```rs
dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
```

## Custom Format
Initialize dioxus-logger using the builder:
```rs
dioxus_logger::DioxusLogger::new(LevelFilter::Info)
        .use_format("[{LEVEL}] {PATH} - {ARGS}")
        .build()
        .expect("failed to init logger");
```
The available options for ``use_format`` are:
- LEVEL - The log severity.
- PATH - Which crate emitted the log.
- ARGS - The text that was logged.
- TIMESTAMP - A timestamp of when the log was emitted. Requires the ``timestamps`` feature.

Surround the option with ``{}`` in the ``use_format`` method.

## License
This project is licensed under the [MIT license].

[mit license]: ./LICENSE

Every contribution intentionally submitted
for inclusion in `dioxus-logger` by you, shall be licensed as MIT, without any additional
terms or conditions.
