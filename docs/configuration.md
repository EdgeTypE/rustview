# Configuration & Theming

## RustViewConfig

The `RustViewConfig` struct controls server behavior:

```rust
use rustview::prelude::*;

let config = RustViewConfig {
    bind: "0.0.0.0:9000".parse().unwrap(),     // Listen address
    title: "My Dashboard".into(),               // Browser tab title
    session_ttl_secs: 3600,                     // Session timeout (1 hour)
    max_upload_bytes: 10_000_000,               // 10 MB upload limit
    theme: Theme::default(),                    // Custom theme
    layout: Layout::default(),                  // Layout options
};

rustview::run_with_config(app, config);
```

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `bind` | `SocketAddr` | `127.0.0.1:8501` | Server bind address |
| `title` | `String` | `"RustView App"` | HTML page title |
| `session_ttl_secs` | `u64` | `86400` | Session TTL in seconds (24h) |
| `max_upload_bytes` | `usize` | `52_428_800` | Max upload size (50 MB) |
| `theme` | `Theme` | Dark theme | Color theme |
| `layout` | `Layout` | Full width, 2rem padding | Body layout options |

---

## Layout

The `Layout` struct controls the maximum width and padding of the application body.
Only set the fields you need — unspecified fields keep their defaults.
When no layout is configured at all, a sensible default of `800px` max-width with `2rem` padding is applied automatically:

```rust
use rustview::prelude::*;

// Builder pattern — set only what you need:
let layout = Layout::default().with_max_width(80);

// Struct update syntax also works:
let layout = Layout {
    max_width_percent: 80,
    ..Default::default()
};

// Or set everything explicitly:
let layout = Layout::default()
    .with_max_width(80)
    .with_padding("3rem 1rem");

let config = RustViewConfig {
    layout,
    ..Default::default()
};
```

| Field | CSS Variable | Default | Description |
|-------|-------------|---------|-------------|
| `max_width_percent` | `--rustview-max-width` | `0` (uses 800px) | Maximum body width as a percentage of the viewport (1–100). Set to 0 to use the built-in 800px default. |
| `padding` | `--rustview-padding` | `"2rem"` | CSS padding for the app body |

### Builder Methods

| Method | Description |
|--------|-------------|
| `Layout::default().with_max_width(80)` | Set width as viewport percentage (1–100) |
| `Layout::default().with_padding("3rem")` | Set only padding, keep default width |
| `.with_max_width(80).with_padding(s)` | Chain both |

---

## Theming

RustView uses CSS custom properties for theming. Customize colors with the `Theme` struct:

```rust
let theme = Theme {
    background: "#1a1a2e".into(),
    foreground: "#e0e0e0".into(),
    primary: "#e94560".into(),
    secondary_bg: "#16213e".into(),
    border: "#0f3460".into(),
    text_secondary: "#8899aa".into(),
};

let config = RustViewConfig {
    theme,
    ..Default::default()
};
```

| Field | CSS Variable | Default | Description |
|-------|-------------|---------|-------------|
| `background` | `--rustview-bg` | `#0e1117` | Page background |
| `foreground` | `--rustview-fg` | `#fafafa` | Primary text color |
| `primary` | `--rustview-primary` | `#ff4b4b` | Accent color (buttons, links, charts) |
| `secondary_bg` | `--rustview-secondary-bg` | `#262730` | Input/card background |
| `border` | `--rustview-border` | `#4a4a5a` | Border color |
| `text_secondary` | `--rustview-text-secondary` | `#a3a8b8` | Labels and secondary text |

### Default Dark Theme

The default theme is a dark design inspired by Streamlit's dark mode:

- Dark navy background (`#0e1117`)
- High-contrast white text (`#fafafa`)
- Coral accent (`#ff4b4b`)

### Custom Theme Example — Ocean Theme

```rust
let theme = Theme {
    background: "#0a192f".into(),
    foreground: "#ccd6f6".into(),
    primary: "#64ffda".into(),
    secondary_bg: "#112240".into(),
    border: "#233554".into(),
    text_secondary: "#8892b0".into(),
};
```

---

## Deployment

### Running on All Interfaces

```rust
let config = RustViewConfig {
    bind: "0.0.0.0:8080".parse().unwrap(),
    ..Default::default()
};
```

### Docker

```dockerfile
FROM rust:1.75 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/my-app /usr/local/bin/
EXPOSE 8501
CMD ["my-app"]
```

### Environment Variables

RustView does not read environment variables by default. Configure everything through `RustViewConfig`. You can read env vars in your own code:

```rust
let port: u16 = std::env::var("PORT")
    .unwrap_or_else(|_| "8501".to_string())
    .parse()
    .unwrap();

let config = RustViewConfig {
    bind: format!("0.0.0.0:{port}").parse().unwrap(),
    ..Default::default()
};
```
