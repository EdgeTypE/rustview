# Contributing to RustView

Thank you for considering a contribution to RustView. This document explains how
to set up the project, run tests, and submit changes.

## Prerequisites

- Rust 1.75 or newer
- Git

## Getting Started

Clone the repository and make sure everything builds:

```bash
git clone https://github.com/EdgeTypE/rustview.git
cd rustview
cargo build
cargo test
```

The workspace has two crates:

- `rustview` (root) -- the main library and server
- `rustview-macros` -- procedural macros (`#[cached]`)

## Running the Examples

Four example applications are included under `examples/`:

```bash
cargo run --example hello       # minimal app
cargo run --example counter     # session state
cargo run --example dashboard   # multi-column layout
cargo run --example showcase    # every widget in one page
```

Each example starts an HTTP server on `http://127.0.0.1:8501`.

## Project Structure

```
src/
  lib.rs          -- public API and prelude
  ui.rs           -- Ui struct, all widget methods
  widgets/mod.rs  -- HTML render functions for each widget
  vdom/mod.rs     -- VNode tree and diffing
  server/mod.rs   -- Axum routes, SSE, JS shim, CSS
  session/mod.rs  -- per-tab session store
  cache/mod.rs    -- computation cache
  testing/mod.rs  -- TestUi harness
  interface.rs    -- Gradio-style Interface API
docs/             -- user-facing documentation
examples/         -- runnable example apps
tests/            -- integration tests
```

## Making Changes

1. Create a feature branch from `main`.
2. Keep commits focused -- one logical change per commit.
3. Add or update tests for any new behavior. The test suite currently has
   over 220 unit tests and 3 integration tests.
4. Run `cargo test` and `cargo clippy` before pushing.
5. If your change affects the public API, update the relevant file in `docs/`.

## Testing

Run the full suite:

```bash
cargo test
```

For a specific module:

```bash
cargo test --lib widgets    # widget render tests
cargo test --lib ui         # Ui method tests
cargo test --test '*'       # integration tests only
```

The `TestUi` harness in `src/testing/mod.rs` lets you unit-test apps without
starting a server. See `docs/testing.md` for patterns and examples.

## Code Style

- Follow standard `rustfmt` formatting.
- Prefer returning values from widgets rather than using callbacks.
- Keep the JavaScript shim in `server/mod.rs` minimal. It currently weighs
  around 4 KB with no external dependencies.
- CSS class names use the `rustview-` prefix. CSS custom properties use
  `--rustview-`.

## Documentation

User-facing docs live in the `docs/` directory. If you add a new widget or
change an existing API, update `docs/widgets.md` or the relevant guide. The
`docs/README.md` table of contents should stay in sync with the file list.

## Submitting a Pull Request

1. Push your branch and open a pull request against `main`.
2. Describe what the change does and why.
3. Make sure CI passes (build + tests).
4. A maintainer will review and may request changes.

## Reporting Issues

Open an issue on GitHub. Include:

- What you expected to happen
- What actually happened
- Steps to reproduce (ideally a small code snippet)
- Rust version (`rustc --version`)
