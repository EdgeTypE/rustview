# Getting Started

## Installation

Add RustView to your `Cargo.toml`:

```toml
[dependencies]
rustview = { git = "https://github.com/EdgeTypE/rustview" }
```

**Minimum Supported Rust Version (MSRV):** 1.75

## Your First App

Create `src/main.rs`:

```rust
use rustview::prelude::*;

fn app(ui: &mut Ui) {
    ui.heading("My First App");
    let name = ui.text_input("Your name", "World");
    let count = ui.int_slider("Repeat", 1..=10);
    for _ in 0..count {
        ui.write(format!("Hello, {}!", name));
    }
}

fn main() {
    rustview::run(app);
}
```

Run with `cargo run`, and your browser will open at `http://127.0.0.1:8501`.

## How It Works

1. RustView calls your `app` function once per interaction.
2. Each widget call (e.g., `text_input`) returns the current value from the session.
3. Output widgets (e.g., `write`, `heading`) append content to the virtual DOM tree.
4. RustView diffs the new tree against the previous tree and sends only changed nodes to the browser via SSE (Server-Sent Events).
5. The browser shim patches the DOM in-place — no full page reload.

## Project Structure

A typical RustView project looks like this:

```
my-app/
├── Cargo.toml
├── src/
│   └── main.rs       # Your app function
```

No `static/`, no `templates/`, no `package.json`. Just Rust.

## Running with Custom Config

```rust
use rustview::prelude::*;

fn app(ui: &mut Ui) {
    ui.write("Running on custom port!");
}

fn main() {
    let config = RustViewConfig {
        bind: "0.0.0.0:9000".parse().unwrap(),
        title: "My Dashboard".into(),
        session_ttl_secs: 3600,
        max_upload_bytes: 10_000_000,
        ..Default::default()
    };
    rustview::run_with_config(app, config);
}
```

## Async Support

If you're already inside a Tokio runtime:

```rust
#[tokio::main]
async fn main() {
    rustview::run_async(|ui| {
        ui.write("Hello from async!");
    }).await;
}
```

## Using the `#[app]` Macro

RustView provides a proc-macro for even shorter startup:

```rust
use rustview::prelude::*;

#[rustview::app]
fn app(ui: &mut Ui) {
    ui.write("Hello!");
}
```

This generates the `main` function and `rustview::run(app)` call for you.

## Next Steps

- Browse the [Widget Reference](widgets.md) for the full API
- Learn about [Layouts](layouts.md) for multi-column designs
- See [Examples](examples.md) for complete runnable apps
