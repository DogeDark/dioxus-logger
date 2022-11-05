<div align="center">
  <h1>📡 Dioxus Logger 🛰️</h1>
  <p><strong>A logging utility to provide a standard interface whether you're targetting web, desktop, or mobile.</strong></p>
</div>

<div align="center">
  <!-- Crates version -->
  <a href="https://crates.io/crates/dioxus-std">
    <img src="https://img.shields.io/crates/v/dioxus-logger.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/dioxus-std">
    <img src="https://img.shields.io/crates/d/dioxus-logger.svg?style=flat-square"
      alt="Download" />
  </a>
  <!-- docs -->
  <a href="https://docs.rs/dioxus-std">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
</div>

-----

`dioxus-logger` is a (planned-to-be) feature-rich logger that uses the `log` crate to provide a standard interface for all your logging needs.

**Current & Planned Features**
- [x] stdio logging (Web, Desktop) - Mobile to come
- [ ] Sending logs over HTTP to an API
- [ ] Logging to a file
- [ ] Timestamps 
- [ ] Custom log format
- [ ] Feature flags for faster compilation

**This library is under development. Expect breaking changes.**

```rust, ignore
use dioxus::prelude::*;
use log::{LevelFilter, info};

fn main() {
  dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
  dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
  info!("app component rendered!");
  rsx!(cx, p { "Hello, Dioxus!" })
}
```

## Platform Support
Dioxus Logger is supported on every platform that Dioxus supports! There are a few advanced features that will only work on a couple of targets. 
Specifically the HTTP logging and filesystem logging will not work on mobile targets.

## Installation
You can add `dioxus-logger` to your application by adding it to your dependencies.
```toml
[dependencies]
dioxus-logger = { version = "x.x.x", features = ["stdio"] }
```

## License
This project is licensed under the [MIT license].

[mit license]: https://github.com/DogeDark/dioxus-logger/LICENSE

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Dioxus by you, shall be licensed as MIT, without any additional
terms or conditions.