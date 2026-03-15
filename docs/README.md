# RustView Documentation

> A Streamlit/Gradio equivalent for pure Rust — turn any Rust function into a live browser UI.

## Table of Contents

| Document | Description |
|----------|-------------|
| [Getting Started](getting-started.md) | Installation, first app, project setup |
| [Widget Reference](widgets.md) | Complete API for all 56+ widgets |
| [Layouts](layouts.md) | Columns, sidebar, tabs, containers, modals |
| [Charts](charts.md) | Line, bar, scatter, and histogram charts |
| [Session State](session-state.md) | Persistent state across re-renders |
| [Configuration & Theming](configuration.md) | Server config, custom themes, deployment |
| [Interface Mode](interface.md) | Gradio-style function→UI mapping |
| [Testing](testing.md) | Unit testing RustView apps with `TestUi` |
| [Architecture](architecture.md) | How RustView works internally |
| [Examples](examples.md) | Runnable code examples |

## Quick Start

```rust
use rustview::prelude::*;

fn app(ui: &mut Ui) {
    let name = ui.text_input("Your name", "World");
    ui.write(format!("Hello, {}!", name));
}

fn main() {
    rustview::run(app);
}
```

Run it:

```bash
cargo run
# Opens http://127.0.0.1:8501 in your browser
```

## Key Concepts

1. **Widget-as-variable**: Every widget returns its current value. A `text_input` returns the current text, a `checkbox` returns `true`/`false`.
2. **Immediate mode**: The app function re-runs on every user interaction. No callbacks, no event handlers.
3. **Server-rendered**: RustView runs an Axum server and streams DOM diffs over SSE. Zero client-side JavaScript needed from the user.
4. **Pure Rust**: No HTML templates, no JavaScript, no build toolchain. Just `cargo run`.
