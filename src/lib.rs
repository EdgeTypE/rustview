//! # RustView — A Streamlit/Gradio Equivalent for Pure Rust
//!
//! RustView lets any Rust developer turn a function into a live browser UI
//! with a single macro call — no HTML, no JavaScript, no build toolchain required.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use rustview::prelude::*;
//!
//! fn app(ui: &mut Ui) {
//!     let name = ui.text_input("Your name", "World");
//!     let times = ui.int_slider("Repeat", 1..=10, 3);
//!     for _ in 0..times {
//!         ui.write(format!("Hello, {}!", name));
//!     }
//! }
//!
//! fn main() {
//!     rustview::run(app);
//! }
//! ```

pub mod cache;
pub mod interface;
pub mod server;
pub mod session;
pub mod testing;
pub mod ui;
pub mod vdom;
pub mod widgets;

/// Re-export proc macros.
pub use rustview_macros::{app, cached};

/// Re-export the Interface type at crate root for convenience.
pub use interface::Interface;

/// Prelude — import everything needed to write a RustView app.
pub mod prelude {
    pub use crate::interface::{Interface, WidgetInput, WidgetOutput};
    pub use crate::server::{Layout, RustViewConfig, Theme};
    pub use crate::ui::Ui;
}

/// Run the RustView application with default configuration.
///
/// This starts an Axum server on `127.0.0.1:8501` and opens the browser.
///
/// # Example
/// ```rust,no_run
/// fn app(ui: &mut rustview::ui::Ui) {
///     ui.write("Hello, World!");
/// }
///
/// fn main() {
///     rustview::run(app);
/// }
/// ```
pub fn run(app_fn: impl Fn(&mut ui::Ui) + Send + Sync + 'static) {
    let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
    rt.block_on(server::run(app_fn));
}

/// Run the RustView application with custom configuration.
pub fn run_with_config(
    app_fn: impl Fn(&mut ui::Ui) + Send + Sync + 'static,
    config: server::RustViewConfig,
) {
    let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
    rt.block_on(server::run_with_config(app_fn, config));
}

/// Run the RustView application from within an existing async context.
///
/// Use this when your app function is already running inside a Tokio runtime
/// (e.g., from `#[tokio::main]`).
///
/// # Example
/// ```rust,no_run
/// #[tokio::main]
/// async fn main() {
///     rustview::run_async(|ui| {
///         ui.write("Hello from async!");
///     }).await;
/// }
/// ```
pub async fn run_async(app_fn: impl Fn(&mut ui::Ui) + Send + Sync + 'static) {
    server::run(app_fn).await;
}

/// Run the RustView application from within an existing async context,
/// with custom configuration.
///
/// # Example
/// ```rust,no_run
/// use rustview::server::RustViewConfig;
///
/// #[tokio::main]
/// async fn main() {
///     let config = RustViewConfig {
///         bind: "0.0.0.0:9000".parse().unwrap(),
///         title: "My App".into(),
///         session_ttl_secs: 3600,
///         max_upload_bytes: 10_000_000,
///         ..Default::default()
///     };
///     rustview::run_async_with_config(|ui| {
///         ui.write("Hello from async!");
///     }, config).await;
/// }
/// ```
pub async fn run_async_with_config(
    app_fn: impl Fn(&mut ui::Ui) + Send + Sync + 'static,
    config: server::RustViewConfig,
) {
    server::run_with_config(app_fn, config).await;
}
