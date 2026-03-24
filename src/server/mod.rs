/// Axum HTTP server — serves the RustView application.
///
/// Handles:
/// - GET / — serve initial HTML (full render)
/// - GET /sse/:sid — SSE event stream for DOM patches
/// - POST /event — widget events from browser
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{
        sse::{Event, KeepAlive, Sse},
        Html, IntoResponse,
    },
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::broadcast;
use uuid::Uuid;

use crate::session::SessionStore;
use crate::ui::Ui;
use crate::vdom::{self, Patch, VNode};

/// Layout options for the application body.
///
/// Controls the maximum width and padding of the main content area.
/// All values are injected as CSS custom properties so they work with
/// the existing theming system.
///
/// When `max_width_percent` is `0` (the default), the built-in CSS
/// default of `800px` is used — so apps look good even without any
/// layout configuration.
///
/// # Example
/// ```
/// use rustview::server::Layout;
///
/// // Builder pattern — only set what you need:
/// let layout = Layout::default().with_max_width(80);
///
/// // Struct update syntax also works:
/// let layout = Layout {
///     max_width_percent: 80,
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone)]
pub struct Layout {
    /// Maximum width of the app body as a percentage of the viewport (1–100).
    /// Set to `0` to use the built-in CSS default of `800px`.
    /// Default: `0` (uses 800px).
    pub max_width_percent: u8,
    /// CSS padding for the app body.
    /// Default: `"2rem"`.
    pub padding: String,
}

impl Default for Layout {
    fn default() -> Self {
        Self {
            max_width_percent: 0,
            padding: "2rem".to_string(),
        }
    }
}

impl Layout {
    /// Set the maximum width as a percentage of the viewport (1–100).
    ///
    /// The value is clamped to 1–100. RustView converts it to CSS
    /// automatically — just pass a plain integer.
    ///
    /// # Example
    /// ```
    /// use rustview::server::Layout;
    /// let layout = Layout::default().with_max_width(80);
    /// assert_eq!(layout.max_width_percent, 80);
    /// assert_eq!(layout.padding, "2rem"); // keeps default
    /// ```
    pub fn with_max_width(mut self, percent: u8) -> Self {
        self.max_width_percent = percent.clamp(1, 100);
        self
    }

    /// Set the CSS padding for the app body.
    ///
    /// # Example
    /// ```
    /// use rustview::server::Layout;
    /// let layout = Layout::default().with_padding("3rem 1rem");
    /// assert_eq!(layout.padding, "3rem 1rem");
    /// assert_eq!(layout.max_width_percent, 0); // keeps default
    /// ```
    pub fn with_padding(mut self, padding: &str) -> Self {
        self.padding = padding.to_string();
        self
    }

    /// Generate CSS custom property declarations for this layout.
    ///
    /// When `max_width_percent` is 0, the `--rustview-max-width` variable
    /// is not emitted, letting the CSS `:root` default of `800px` apply.
    pub fn to_css_vars(&self) -> String {
        let padding_decl = format!("--rustview-padding: {};", self.padding);
        if self.max_width_percent == 0 {
            padding_decl
        } else {
            let pct = self.max_width_percent.clamp(1, 100);
            format!("--rustview-max-width: {}%;\n  {}", pct, padding_decl)
        }
    }
}

/// Theme colors for RustView.
///
/// Override any CSS custom property to customize the look of the UI.
/// Unset fields use the default dark theme values.
#[derive(Debug, Clone)]
pub struct Theme {
    /// Background color (default: `#0e1117`)
    pub background: String,
    /// Foreground text color (default: `#fafafa`)
    pub foreground: String,
    /// Primary accent color (default: `#ff4b4b`)
    pub primary: String,
    /// Secondary background color for inputs/cards (default: `#262730`)
    pub secondary_bg: String,
    /// Border color (default: `#4a4a5a`)
    pub border: String,
    /// Label/secondary text color (default: `#a3a8b8`)
    pub text_secondary: String,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            background: "#0e1117".to_string(),
            foreground: "#fafafa".to_string(),
            primary: "#ff4b4b".to_string(),
            secondary_bg: "#262730".to_string(),
            border: "#4a4a5a".to_string(),
            text_secondary: "#a3a8b8".to_string(),
        }
    }
}

impl Theme {
    /// Generate CSS custom property declarations for this theme.
    pub fn to_css_vars(&self) -> String {
        format!(
            ":root {{\n  --rustview-bg: {};\n  --rustview-fg: {};\n  --rustview-primary: {};\n  --rustview-secondary-bg: {};\n  --rustview-border: {};\n  --rustview-text-secondary: {};\n}}",
            self.background, self.foreground, self.primary,
            self.secondary_bg, self.border, self.text_secondary
        )
    }
}

/// Server configuration.
#[derive(Debug, Clone)]
pub struct RustViewConfig {
    /// Bind address (default: 127.0.0.1:8501).
    pub bind: std::net::SocketAddr,
    /// Page title shown in the browser tab. Default: "RustView App".
    pub title: String,
    /// Session TTL in seconds. Default: 86400 (24 hours).
    pub session_ttl_secs: u64,
    /// Maximum upload size in bytes. Default: 52_428_800 (50 MB).
    pub max_upload_bytes: usize,
    /// Custom theme colors. Uses the default dark theme if not set.
    pub theme: Theme,
    /// Layout options controlling max width and padding of the app body.
    pub layout: Layout,
    /// Automatically open the browser on startup. Default: false.
    pub open_browser: bool,
}

impl Default for RustViewConfig {
    fn default() -> Self {
        RustViewConfig {
            bind: "127.0.0.1:8501".parse().unwrap(),
            title: "RustView App".into(),
            session_ttl_secs: 86400,
            max_upload_bytes: 52_428_800,
            theme: Theme::default(),
            layout: Layout::default(),
            open_browser: false,
        }
    }
}

/// Shared application state.
pub struct AppState {
    pub session_store: SessionStore,
    /// The user's app function.
    pub app_fn: Box<dyn Fn(&mut Ui) + Send + Sync>,
    /// Broadcast channel for SSE events per session.
    pub sse_channels: dashmap::DashMap<Uuid, broadcast::Sender<String>>,
    /// Server configuration.
    pub config: RustViewConfig,
}

/// Widget event from the browser.
#[derive(Debug, Deserialize)]
pub struct WidgetEvent {
    pub sid: Uuid,
    pub widget_id: String,
    pub value: serde_json::Value,
}

/// Run the app function for a session, producing patches.
/// If the app function panics, the error is caught and displayed as an error widget.
fn run_app_and_diff(state: &AppState, session_id: &Uuid) -> Option<(VNode, Vec<Patch>)> {
    let mut session_ref = state.session_store.get_session_mut(session_id)?;
    let old_tree = session_ref.last_tree.clone();

    let mut ui = Ui::new(&mut session_ref);

    // Catch panics in the user's app function to display them in the browser
    // instead of crashing the server.
    let panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        (state.app_fn)(&mut ui);
    }));

    if let Err(panic_info) = panic_result {
        // Extract panic message
        let message = if let Some(s) = panic_info.downcast_ref::<&str>() {
            s.to_string()
        } else if let Some(s) = panic_info.downcast_ref::<String>() {
            s.clone()
        } else {
            "App function panicked (unknown error)".to_string()
        };
        tracing::error!("App function panicked: {}", message);
        ui.error(&format!("💥 App panicked: {}", message));
    }

    let new_tree = ui.build_tree();

    let patches = match &old_tree {
        Some(old) => vdom::diff(old, &new_tree),
        None => vec![Patch::FullRender {
            root: new_tree.clone(),
        }],
    };

    session_ref.last_tree = Some(new_tree.clone());

    Some((new_tree, patches))
}

/// Render the initial HTML page.
fn render_initial_html(tree: &VNode, session_id: Uuid, title: &str, theme_css: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{title}</title>
    <style>{CSS}
{theme_css}</style>
</head>
<body>
    <div id="rustview-root" class="rustview-app">{}</div>
    <script>
        const SESSION_ID = "{}";
        {}
    </script>
</body>
</html>"#,
        vnode_to_html(tree),
        session_id,
        BROWSER_SHIM
    )
}

/// Convert a VNode tree to HTML string.
fn vnode_to_html(node: &VNode) -> String {
    let mut html = String::new();

    // Skip the root wrapper div since it's already in the HTML template
    if node.id == "rustview-root" {
        for child in &node.children {
            html.push_str(&vnode_to_inner_html(child));
        }
        return html;
    }

    vnode_to_inner_html(node)
}

/// Tags that are self-closing in HTML (no closing tag).
const SELF_CLOSING_TAGS: &[&str] = &["input", "br", "hr", "img", "source"];

fn vnode_to_inner_html(node: &VNode) -> String {
    let mut html = String::new();
    html.push('<');
    html.push_str(&node.tag);

    html.push_str(&format!(" id=\"{}\"", node.id));
    
    // Extract innerHTML before adding attributes
    let mut inner_html = None;
    
    for (key, value) in &node.attrs {
        // Special handling for data-innerHTML (chart SVG)
        if key == "data-innerHTML" {
            inner_html = Some(value.clone());
        } else {
            html.push_str(&format!(" {}=\"{}\"", key, html_escape(value)));
        }
    }
    html.push('>');

    if let Some(ref text) = node.text {
        html.push_str(&html_escape(text));
    }

    // Insert innerHTML without escaping (for SVG rendering)
    if let Some(svg_content) = inner_html {
        html.push_str(&svg_content);
    }

    for child in &node.children {
        html.push_str(&vnode_to_inner_html(child));
    }

    // Self-closing tags don't get a closing tag
    if !SELF_CLOSING_TAGS.contains(&node.tag.as_str()) {
        html.push_str(&format!("</{}>", node.tag));
    }

    html
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// GET / — serve the initial HTML page.
async fn index_handler(State(state): State<Arc<AppState>>) -> Html<String> {
    let session_id = state.session_store.create_session();

    // Create SSE channel for this session
    let (tx, _) = broadcast::channel(64);
    state.sse_channels.insert(session_id, tx);

    // Run the app to get initial tree
    let title = state.config.title.as_str();
    let theme_css = state.config.theme.to_css_vars();
    let layout_css = state.config.layout.to_css_vars();
    let custom_css = format!("{}\n:root {{\n  {}\n}}", theme_css, layout_css);
    let html = if let Some((tree, _)) = run_app_and_diff(&state, &session_id) {
        render_initial_html(&tree, session_id, title, &custom_css)
    } else {
        render_initial_html(
            &VNode::new("rustview-root", "div").with_attr("class", "rustview-app"),
            session_id,
            title,
            &custom_css,
        )
    };

    Html(html)
}

/// POST /event — handle widget events.
///
/// Returns the computed DOM patches directly in the response body so the
/// browser can apply them synchronously, eliminating the one-cycle delay
/// that occurs when patches are delivered only via the async SSE channel.
async fn event_handler(
    State(state): State<Arc<AppState>>,
    Json(event): Json<WidgetEvent>,
) -> impl IntoResponse {
    // Update widget state
    {
        if let Some(mut session) = state.session_store.get_session_mut(&event.sid) {
            session.set_widget_value(&event.widget_id, event.value);
        } else {
            return (StatusCode::NOT_FOUND, Json(serde_json::json!([])));
        }
    }

    // Re-run app and compute patches, return them in the response
    let patches_json = if let Some((_, patches)) = run_app_and_diff(&state, &event.sid) {
        serde_json::json!(patches)
    } else {
        serde_json::json!([])
    };

    (StatusCode::OK, Json(patches_json))
}

/// GET /sse/:sid — SSE event stream.
async fn sse_handler(
    State(state): State<Arc<AppState>>,
    Path(sid): Path<Uuid>,
) -> Result<Sse<impl futures_core::Stream<Item = Result<Event, Infallible>>>, StatusCode> {
    let tx = state.sse_channels.get(&sid).ok_or(StatusCode::NOT_FOUND)?;

    let mut rx = tx.subscribe();
    drop(tx);

    let stream = async_stream::stream! {
        loop {
            match rx.recv().await {
                Ok(data) => {
                    yield Ok(Event::default().data(data));
                }
                Err(broadcast::error::RecvError::Lagged(n)) => {
                    tracing::warn!("SSE client lagged by {} messages for session {}", n, sid);
                    continue;
                }
                Err(broadcast::error::RecvError::Closed) => {
                    break;
                }
            }
        }
    };

    Ok(Sse::new(stream).keep_alive(KeepAlive::default()))
}

/// Build the Axum router.
pub fn build_router(app_fn: impl Fn(&mut Ui) + Send + Sync + 'static) -> Router {
    build_router_with_state(Arc::new(AppState {
        session_store: SessionStore::new(),
        app_fn: Box::new(app_fn),
        sse_channels: dashmap::DashMap::new(),
        config: RustViewConfig::default(),
    }))
}

/// Build the Axum router with pre-constructed shared state.
fn build_router_with_state(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(index_handler))
        .route("/event", post(event_handler))
        .route("/sse/{sid}", get(sse_handler))
        .with_state(state)
}

/// Run the RustView application with default configuration.
pub async fn run(app_fn: impl Fn(&mut Ui) + Send + Sync + 'static) {
    run_with_config(app_fn, RustViewConfig::default()).await;
}

/// Run the RustView application with custom configuration.
pub async fn run_with_config(
    app_fn: impl Fn(&mut Ui) + Send + Sync + 'static,
    config: RustViewConfig,
) {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rustview=info".into()),
        )
        .init();

    let state = Arc::new(AppState {
        session_store: SessionStore::with_ttl(std::time::Duration::from_secs(
            config.session_ttl_secs,
        )),
        app_fn: Box::new(app_fn),
        sse_channels: dashmap::DashMap::new(),
        config,
    });

    let router = build_router_with_state(state.clone());

    // Spawn background session cleanup task (runs every 5 minutes)
    let cleanup_store = state.session_store.clone();
    let cleanup_channels = state.sse_channels.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(300));
        loop {
            interval.tick().await;
            let removed = cleanup_store.cleanup_expired();
            if removed > 0 {
                tracing::info!("Session cleanup: removed {} expired sessions", removed);
                // Also clean up SSE channels for removed sessions
                cleanup_channels.retain(|id, _| cleanup_store.get_session(id).is_some());
            }
        }
    });

    tracing::info!("RustView running at http://{}", state.config.bind);

    if state.config.open_browser {
        let url = format!("http://{}", state.config.bind);
        if let Err(e) = webbrowser::open(&url) {
            tracing::error!("Failed to open browser: {}", e);
        }
    }

    if !state.config.bind.ip().is_loopback() {
        tracing::warn!(
            "WARNING: RustView is exposed on the network. \
             There is no authentication. Do not expose to untrusted networks."
        );
    }

    let listener = tokio::net::TcpListener::bind(state.config.bind).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

/// CSS styles for RustView widgets.
const CSS: &str = r#"
:root {
    --rustview-bg: #0e1117;
    --rustview-fg: #fafafa;
    --rustview-primary: #ff4b4b;
    --rustview-secondary-bg: #262730;
    --rustview-border: #4a4a5a;
    --rustview-text-secondary: #a3a8b8;
    --rustview-max-width: 800px;
    --rustview-padding: 2rem;
}
* { box-sizing: border-box; margin: 0; padding: 0; }
body {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
    background: var(--rustview-bg);
    color: var(--rustview-fg);
    padding: var(--rustview-padding);
    max-width: var(--rustview-max-width);
    margin: 0 auto;
}
.rustview-app { display: flex; flex-direction: column; gap: 1rem; }
.rustview-widget { margin-bottom: 0.5rem; }

/* Text Input */
.rustview-text-input label { display: block; font-size: 0.875rem; color: var(--rustview-text-secondary); margin-bottom: 0.25rem; }
.rustview-text-input input {
    width: 100%; padding: 0.5rem 0.75rem; background: var(--rustview-secondary-bg); border: 1px solid var(--rustview-border);
    border-radius: 0.375rem; color: var(--rustview-fg); font-size: 1rem; outline: none;
}
.rustview-text-input input:focus { border-color: var(--rustview-primary); }

/* Slider */
.rustview-slider label { display: block; font-size: 0.875rem; color: var(--rustview-text-secondary); margin-bottom: 0.25rem; }
.rustview-slider input[type="range"] { width: 100%; accent-color: var(--rustview-primary); }

/* Checkbox */
.rustview-checkbox label { display: flex; align-items: center; gap: 0.5rem; cursor: pointer; }
.rustview-checkbox input[type="checkbox"] { accent-color: var(--rustview-primary); width: 1.25rem; height: 1.25rem; }

/* Button */
.rustview-button button {
    padding: 0.5rem 1.5rem; background: var(--rustview-primary); color: white; border: none;
    border-radius: 0.375rem; font-size: 0.875rem; cursor: pointer; font-weight: 500;
}
.rustview-button button:hover { background: #ff6b6b; }

/* Write */
.rustview-write p { font-size: 1rem; line-height: 1.6; }

/* Markdown */
.rustview-markdown { line-height: 1.6; }

/* Progress */
.rustview-progress-bg {
    width: 100%; height: 0.5rem; background: #262730; border-radius: 0.25rem; overflow: hidden;
}
.rustview-progress-fill { height: 100%; background: #ff4b4b; transition: width 0.3s ease; }
.rustview-progress span { font-size: 0.75rem; color: #a3a8b8; }

/* Alert */
.rustview-alert {
    padding: 0.75rem 1rem; border-radius: 0.375rem; display: flex; align-items: center; gap: 0.5rem;
}
.rustview-alert-error { background: rgba(255, 75, 75, 0.1); border: 1px solid #ff4b4b; color: #ff6b6b; }
.rustview-alert-icon { font-size: 1.25rem; }

/* Number Input */
.rustview-number-input label { display: block; font-size: 0.875rem; color: #a3a8b8; margin-bottom: 0.25rem; }
.rustview-number-input input[type="number"] {
    width: 100%; padding: 0.5rem 0.75rem; background: #262730; border: 1px solid #4a4a5a;
    border-radius: 0.375rem; color: #fafafa; font-size: 1rem; outline: none;
}
.rustview-number-input input:focus { border-color: #ff4b4b; }

/* Integer Input */
.rustview-int-input label { display: block; font-size: 0.875rem; color: #a3a8b8; margin-bottom: 0.25rem; }
.rustview-int-input input[type="number"] {
    width: 100%; padding: 0.5rem 0.75rem; background: #262730; border: 1px solid #4a4a5a;
    border-radius: 0.375rem; color: #fafafa; font-size: 1rem; outline: none;
}
.rustview-int-input input:focus { border-color: #ff4b4b; }

/* Toggle */
.rustview-toggle label { display: flex; align-items: center; gap: 0.75rem; cursor: pointer; }
.rustview-toggle input[type="checkbox"] { display: none; }
.rustview-toggle-track {
    width: 2.5rem; height: 1.25rem; background: #4a4a5a; border-radius: 0.625rem;
    position: relative; transition: background 0.2s ease;
}
.rustview-toggle-track::after {
    content: ''; position: absolute; top: 0.125rem; left: 0.125rem;
    width: 1rem; height: 1rem; background: #fafafa; border-radius: 50%;
    transition: transform 0.2s ease;
}
.rustview-toggle input:checked + .rustview-toggle-track { background: #ff4b4b; }
.rustview-toggle input:checked + .rustview-toggle-track::after { transform: translateX(1.25rem); }

/* Radio */
.rustview-radio > label { display: block; font-size: 0.875rem; color: #a3a8b8; margin-bottom: 0.5rem; }
.rustview-radio-options { display: flex; flex-direction: column; gap: 0.375rem; }
.rustview-radio-options label {
    display: flex; align-items: center; gap: 0.5rem; cursor: pointer; font-size: 0.9375rem;
}
.rustview-radio-options input[type="radio"] { accent-color: #ff4b4b; }

/* Select */
.rustview-select label { display: block; font-size: 0.875rem; color: var(--rustview-text-secondary); margin-bottom: 0.25rem; }
.rustview-select select {
    width: 100%; padding: 0.5rem 0.75rem; background: #262730; border: 1px solid #4a4a5a;
    border-radius: 0.375rem; color: #fafafa; font-size: 1rem; outline: none;
    appearance: none; -webkit-appearance: none;
    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 12 12'%3E%3Cpath fill='%23a3a8b8' d='M6 8L1 3h10z'/%3E%3C/svg%3E");
    background-repeat: no-repeat; background-position: right 0.75rem center;
}
.rustview-select select:focus { border-color: #ff4b4b; }

/* Multi-Select */
.rustview-multi-select label { display: block; font-size: 0.875rem; color: #a3a8b8; margin-bottom: 0.25rem; }
.rustview-multi-select select {
    width: 100%; padding: 0.5rem; background: #262730; border: 1px solid #4a4a5a;
    border-radius: 0.375rem; color: #fafafa; font-size: 1rem; outline: none;
    min-height: 5rem;
}
.rustview-multi-select select:focus { border-color: #ff4b4b; }
.rustview-multi-select select option { padding: 0.25rem 0.5rem; }

/* Text Area */
.rustview-text-area label { display: block; font-size: 0.875rem; color: #a3a8b8; margin-bottom: 0.25rem; }
.rustview-text-area textarea {
    width: 100%; padding: 0.5rem 0.75rem; background: #262730; border: 1px solid #4a4a5a;
    border-radius: 0.375rem; color: #fafafa; font-size: 1rem; outline: none;
    resize: vertical; font-family: inherit;
}
.rustview-text-area textarea:focus { border-color: #ff4b4b; }

/* Color Picker */
.rustview-color-picker label { display: block; font-size: 0.875rem; color: #a3a8b8; margin-bottom: 0.25rem; }
.rustview-color-picker input[type="color"] {
    width: 3rem; height: 2rem; padding: 0; border: 1px solid #4a4a5a;
    border-radius: 0.375rem; cursor: pointer; background: transparent;
}

/* Download Button */
.rustview-download-button a { text-decoration: none; }
.rustview-download-button button {
    padding: 0.5rem 1.5rem; background: #262730; color: #fafafa; border: 1px solid #4a4a5a;
    border-radius: 0.375rem; font-size: 0.875rem; cursor: pointer; font-weight: 500;
}
.rustview-download-button button:hover { background: #363740; border-color: #ff4b4b; }

/* Link */
.rustview-link a {
    color: var(--rustview-primary); text-decoration: none; font-size: 0.9375rem;
    transition: color 0.2s;
}
.rustview-link a:hover { color: #ff6b6b; text-decoration: underline; }

/* Heading */
.rustview-heading h1 { font-size: 2rem; font-weight: 700; color: #fafafa; line-height: 1.3; }

/* Subheading */
.rustview-subheading h2 { font-size: 1.5rem; font-weight: 600; color: #fafafa; line-height: 1.3; }

/* Caption */
.rustview-caption small { font-size: 0.8125rem; color: #808495; }

/* Code Block */
.rustview-code pre {
    background: #1a1b26; border: 1px solid #2d2f3a; border-radius: 0.375rem;
    padding: 1rem; overflow-x: auto; font-size: 0.875rem; line-height: 1.5;
}
.rustview-code code { color: #c0caf5; font-family: 'Fira Code', 'Cascadia Code', monospace; }

/* JSON */
.rustview-json pre {
    background: #1a1b26; border: 1px solid #2d2f3a; border-radius: 0.375rem;
    padding: 1rem; overflow-x: auto; font-size: 0.875rem; line-height: 1.5;
}
.rustview-json code { color: #c0caf5; font-family: 'Fira Code', 'Cascadia Code', monospace; }

/* Table */
.rustview-table table {
    width: 100%; border-collapse: collapse; font-size: 0.9375rem;
}
.rustview-table th {
    text-align: left; padding: 0.5rem 0.75rem; background: #1a1b26;
    border-bottom: 2px solid #4a4a5a; font-weight: 600; color: #a3a8b8;
}
.rustview-table td {
    padding: 0.5rem 0.75rem; border-bottom: 1px solid #262730;
}
.rustview-table tr:hover td { background: rgba(255, 255, 255, 0.03); }

/* Dataframe */
.rustview-dataframe-title {
    font-weight: 600; font-size: 0.875rem; margin-bottom: 0.375rem; color: var(--rustview-fg);
}
.rustview-dataframe-scroll {
    overflow-x: auto; max-height: 500px; overflow-y: auto;
    border: 1px solid var(--rustview-border); border-radius: 0.5rem;
}
.rustview-dataframe table {
    width: 100%; border-collapse: collapse; font-size: 0.8125rem;
    font-family: 'Fira Code', 'Cascadia Code', 'Consolas', monospace;
}
.rustview-dataframe th {
    position: sticky; top: 0; z-index: 1;
    text-align: left; padding: 0.375rem 0.625rem;
    background: var(--rustview-secondary-bg); border-bottom: 2px solid var(--rustview-border);
    font-weight: 600; white-space: nowrap;
}
.rustview-dataframe-col-name { display: block; color: var(--rustview-fg); }
.rustview-dataframe-col-type {
    display: block; font-size: 0.6875rem; color: var(--rustview-text-secondary);
    font-weight: 400; opacity: 0.7;
}
.rustview-dataframe td {
    padding: 0.25rem 0.625rem; border-bottom: 1px solid var(--rustview-border);
    white-space: nowrap;
}
.rustview-dataframe-idx {
    color: var(--rustview-text-secondary); text-align: right; min-width: 2.5rem;
    font-size: 0.75rem; opacity: 0.5; padding-right: 0.75rem !important;
    background: var(--rustview-secondary-bg);
}
.rustview-dataframe-num { text-align: right; }
.rustview-dataframe tr:hover td { background: rgba(255, 255, 255, 0.03); }
.rustview-dataframe-shape {
    font-size: 0.75rem; color: var(--rustview-text-secondary);
    margin-top: 0.25rem; text-align: right;
}

/* Spinner */
.rustview-spinner {
    display: flex; align-items: center; gap: 0.5rem; color: #a3a8b8;
}
.rustview-spinner-icon {
    display: inline-block; font-size: 1.25rem;
    animation: rustview-spin 1s linear infinite;
}
@keyframes rustview-spin { from { transform: rotate(0deg); } to { transform: rotate(360deg); } }

/* Metric */
.rustview-metric {
    display: flex; flex-direction: column; gap: 0.125rem;
}
.rustview-metric-label { font-size: 0.875rem; color: #a3a8b8; }
.rustview-metric-value { font-size: 2rem; font-weight: 700; color: #fafafa; }
.rustview-metric-delta { font-size: 0.875rem; font-weight: 500; }
.rustview-metric-delta-positive { color: #50fa7b; }
.rustview-metric-delta-negative { color: #ff5555; }

/* Alert variants */
.rustview-alert-success { background: rgba(80, 250, 123, 0.1); border: 1px solid #50fa7b; color: #50fa7b; }
.rustview-alert-warning { background: rgba(255, 183, 77, 0.1); border: 1px solid #ffb74d; color: #ffb74d; }
.rustview-alert-info { background: rgba(100, 181, 246, 0.1); border: 1px solid #64b5f6; color: #64b5f6; }

/* Divider */
.rustview-divider hr {
    border: none; border-top: 1px solid #2d2f3a; margin: 0.5rem 0;
}

/* Layout: Columns */
.rustview-columns { gap: 1rem; }
.rustview-column { min-width: 0; }

/* Layout: Sidebar */
.rustview-sidebar {
    position: fixed; top: 0; left: 0; width: 280px; height: 100vh;
    background: #161822; padding: 2rem 1rem; overflow-y: auto;
    border-right: 1px solid #2d2f3a; z-index: 100;
    display: flex; flex-direction: column; gap: 1rem;
}
body:has(.rustview-sidebar) { padding-left: 300px; }

/* Layout: Expander */
.rustview-expander details {
    border: 1px solid #2d2f3a; border-radius: 0.375rem; overflow: hidden;
}
.rustview-expander summary {
    padding: 0.75rem 1rem; background: #1a1b26; cursor: pointer;
    font-weight: 500; list-style: none;
}
.rustview-expander summary::before { content: '▶ '; font-size: 0.75rem; }
.rustview-expander details[open] summary::before { content: '▼ '; }
.rustview-expander-content {
    padding: 1rem; display: flex; flex-direction: column; gap: 0.5rem;
}

/* Layout: Tabs */
.rustview-tab-bar {
    display: flex; gap: 0; border-bottom: 2px solid #2d2f3a; margin-bottom: 1rem;
}
.rustview-tab-bar button {
    padding: 0.5rem 1rem; background: none; border: none;
    color: #a3a8b8; cursor: pointer; font-size: 0.9375rem;
    border-bottom: 2px solid transparent; margin-bottom: -2px;
    transition: color 0.2s, border-color 0.2s;
}
.rustview-tab-bar button:hover { color: #fafafa; }
.rustview-tab-bar button.active { color: #ff4b4b; border-bottom-color: #ff4b4b; }
.rustview-tab-content {
    display: flex; flex-direction: column; gap: 0.5rem;
}

/* Layout: Container */
.rustview-container {
    border: 1px solid #2d2f3a; border-radius: 0.375rem; padding: 1rem;
    display: flex; flex-direction: column; gap: 0.5rem;
}

/* Date Picker */
.rustview-date-picker label { display: block; font-size: 0.875rem; color: #a3a8b8; margin-bottom: 0.25rem; }
.rustview-date-picker input[type="date"] {
    width: 100%; padding: 0.5rem 0.75rem; background: #262730; border: 1px solid #4a4a5a;
    border-radius: 0.375rem; color: #fafafa; font-size: 1rem; outline: none;
    color-scheme: dark;
}
.rustview-date-picker input:focus { border-color: #ff4b4b; }

/* File Upload */
.rustview-file-upload label { display: block; font-size: 0.875rem; color: #a3a8b8; margin-bottom: 0.25rem; }
.rustview-file-upload input[type="file"] {
    width: 100%; padding: 0.5rem; background: #262730; border: 1px solid #4a4a5a;
    border-radius: 0.375rem; color: #fafafa; font-size: 0.875rem;
    cursor: pointer;
}
.rustview-file-upload input[type="file"]::file-selector-button {
    padding: 0.375rem 0.75rem; background: #ff4b4b; color: white; border: none;
    border-radius: 0.25rem; cursor: pointer; font-size: 0.8125rem; margin-right: 0.5rem;
}
.rustview-file-upload input[type="file"]::file-selector-button:hover { background: #ff6b6b; }

/* Image Upload */
.rustview-image-upload label { display: block; font-size: 0.875rem; color: #a3a8b8; margin-bottom: 0.25rem; }
.rustview-image-upload input[type="file"] {
    width: 100%; padding: 0.5rem; background: #262730; border: 1px solid #4a4a5a;
    border-radius: 0.375rem; color: #fafafa; font-size: 0.875rem;
    cursor: pointer;
}
.rustview-image-upload input[type="file"]::file-selector-button {
    padding: 0.375rem 0.75rem; background: #6c5ce7; color: white; border: none;
    border-radius: 0.25rem; cursor: pointer; font-size: 0.8125rem; margin-right: 0.5rem;
}
.rustview-image-upload input[type="file"]::file-selector-button:hover { background: #7d6ff0; }
.rustview-image-upload-preview {
    max-width: 100%; max-height: 200px; border-radius: 0.375rem;
    margin-top: 0.5rem; object-fit: contain;
}

/* Form */
.rustview-form {
    border: 1px solid #2d2f3a; border-radius: 0.5rem; padding: 1rem;
    display: flex; flex-direction: column; gap: 0.75rem;
    background: rgba(255, 255, 255, 0.02);
}
.rustview-form-submit button {
    width: 100%; padding: 0.5rem 1rem; background: #ff4b4b; color: white;
    border: none; border-radius: 0.375rem; font-size: 0.875rem; font-weight: 600;
    cursor: pointer; transition: background 0.2s;
}
.rustview-form-submit button:hover { background: #ff6b6b; }

/* Image */
.rustview-image { display: flex; flex-direction: column; gap: 0.25rem; }
.rustview-image img { max-width: 100%; height: auto; border-radius: 0.375rem; }
.rustview-image small { font-size: 0.8125rem; color: #808495; text-align: center; }

/* Audio */
.rustview-audio audio { width: 100%; }

/* Video */
.rustview-video video { width: 100%; border-radius: 0.375rem; }

/* Row layout */
.rustview-row {
    display: flex; flex-direction: row; gap: 0.75rem; align-items: flex-start;
    flex-wrap: wrap;
}

/* Empty placeholder */
.rustview-empty { min-height: 0; }

/* Modal */
.rustview-modal-overlay {
    position: fixed; top: 0; left: 0; width: 100vw; height: 100vh;
    background: rgba(0, 0, 0, 0.6); z-index: 1000;
    display: flex; align-items: center; justify-content: center;
}
.rustview-modal-dialog {
    background: #1a1b26; border: 1px solid #2d2f3a; border-radius: 0.5rem;
    width: 90%; max-width: 560px; max-height: 80vh; overflow-y: auto;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);
}
.rustview-modal-header {
    display: flex; justify-content: space-between; align-items: center;
    padding: 1rem 1.25rem; border-bottom: 1px solid #2d2f3a;
}
.rustview-modal-header h3 { font-size: 1.125rem; font-weight: 600; color: #fafafa; }
.rustview-modal-close {
    background: none; border: none; color: #a3a8b8; font-size: 1.25rem;
    cursor: pointer; padding: 0.25rem;
}
.rustview-modal-close:hover { color: #ff4b4b; }
.rustview-modal-body {
    padding: 1.25rem; display: flex; flex-direction: column; gap: 0.75rem;
}

/* Toast notifications */
.rustview-toast {
    position: fixed; right: 1rem; z-index: 2000;
    display: flex; align-items: center; gap: 0.5rem;
    padding: 0.75rem 1rem; border-radius: 0.5rem;
    font-size: 0.9375rem; min-width: 18rem; max-width: 28rem;
    animation: rustview-toast-in 0.3s ease, rustview-toast-out 0.3s ease 4.7s forwards;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
    pointer-events: auto;
}
.rustview-toast-success { background: #1a3a2a; border: 1px solid #50fa7b; color: #50fa7b; }
.rustview-toast-error { background: #3a1a1a; border: 1px solid #ff4b4b; color: #ff6b6b; }
.rustview-toast-warning { background: #3a2e1a; border: 1px solid #ffb74d; color: #ffb74d; }
.rustview-toast-info { background: #1a2a3a; border: 1px solid #64b5f6; color: #64b5f6; }
.rustview-toast-icon { font-size: 1.25rem; }
.rustview-toast-message { font-size: 0.9375rem; }
@keyframes rustview-toast-in { from { opacity: 0; transform: translateY(-1rem); } to { opacity: 1; transform: translateY(0); } }
@keyframes rustview-toast-out { from { opacity: 1; } to { opacity: 0; transform: translateY(-1rem); } }

/* Charts */
.rustview-chart { display: flex; flex-direction: column; gap: 0.5rem; }
.rustview-chart-title { font-size: 0.9375rem; font-weight: 600; color: #fafafa; }
.rustview-chart-svg svg { width: 100%; height: auto; }
"#;

/// Browser JavaScript shim (~4KB, no dependencies).
const BROWSER_SHIM: &str = r#"
(function() {
    'use strict';

    // SSE connection
    let eventSource = null;
    let reconnectAttempts = 0;
    const MAX_RECONNECT_DELAY = 30000;

    function connectSSE() {
        if (eventSource) { eventSource.close(); }
        eventSource = new EventSource('/sse/' + SESSION_ID);

        eventSource.onopen = function() {
            console.log('[RustView] SSE connected');
            reconnectAttempts = 0;
        };

        eventSource.onmessage = function(event) {
            try {
                const patches = JSON.parse(event.data);
                applyPatches(patches);
            } catch (e) {
                console.error('[RustView] Failed to parse SSE data:', e);
            }
        };

        eventSource.onerror = function() {
            eventSource.close();
            const delay = Math.min(1000 * Math.pow(2, reconnectAttempts), MAX_RECONNECT_DELAY);
            reconnectAttempts++;
            console.log('[RustView] SSE disconnected. Reconnecting in ' + delay + 'ms...');
            setTimeout(connectSSE, delay);
        };
    }

    // Apply DOM patches
    function applyPatches(patches) {
        for (const patch of patches) {
            switch (patch.op) {
                case 'full_render':
                    applyFullRender(patch.root);
                    break;
                case 'replace':
                    applyReplace(patch.id, patch.node);
                    break;
                case 'update_text':
                    applyUpdateText(patch.id, patch.text);
                    break;
                case 'update_attrs':
                    applyUpdateAttrs(patch.id, patch.attrs);
                    break;
                case 'append_child':
                    applyAppendChild(patch.parent_id, patch.node);
                    break;
                case 'remove_child':
                    applyRemoveChild(patch.id);
                    break;
                default:
                    console.warn('[RustView] Unknown patch op:', patch.op);
            }
        }
    }

    function applyFullRender(root) {
        const container = document.getElementById('rustview-root');
        if (container) {
            container.innerHTML = '';
            for (const child of (root.children || [])) {
                container.appendChild(createDOMNode(child));
            }
            attachEventListeners();
        }
    }

    function applyReplace(id, node) {
        const el = document.getElementById(id);
        if (el) {
            const newEl = createDOMNode(node);
            el.parentNode.replaceChild(newEl, el);
            attachEventListeners();
        }
    }

    function applyUpdateText(id, text) {
        const el = document.getElementById(id);
        if (el) {
            // Update only the text, preserving children
            const textNode = el.childNodes[0];
            if (textNode && textNode.nodeType === 3) {
                textNode.textContent = text;
            } else {
                el.textContent = text;
            }
        }
    }

    function applyUpdateAttrs(id, attrs) {
        const el = document.getElementById(id);
        if (el) {
            // Remove old attributes that are no longer present
            const toRemove = Array.from(el.attributes)
                .map(function(a) { return a.name; })
                .filter(function(name) { return name !== 'id' && !(name in attrs); });
            for (const name of toRemove) {
                el.removeAttribute(name);
            }
            // Set new/updated attributes
            for (const [key, value] of Object.entries(attrs)) {
                if (key === 'checked') {
                    el.checked = value === 'true';
                } else if (key === 'value') {
                    el.value = value;
                } else {
                    el.setAttribute(key, value);
                }
            }
        }
    }

    function applyAppendChild(parentId, node) {
        const parent = document.getElementById(parentId);
        if (parent) {
            parent.appendChild(createDOMNode(node));
            attachEventListeners();
        }
    }

    function applyRemoveChild(id) {
        const el = document.getElementById(id);
        if (el) { el.remove(); }
    }

    function createDOMNode(vnode) {
        const el = document.createElement(vnode.tag);
        el.id = vnode.id;
        for (const [key, value] of Object.entries(vnode.attrs || {})) {
            if (key === 'checked') {
                el.checked = value === 'true';
            } else {
                el.setAttribute(key, value);
            }
        }
        if (vnode.text) {
            el.textContent = vnode.text;
        }
        for (const child of (vnode.children || [])) {
            el.appendChild(createDOMNode(child));
        }
        return el;
    }

    // Event handling with debounce
    const DEBOUNCE_SLIDER = 50;
    const DEBOUNCE_TEXT = 200;
    let debounceTimers = {};

    function sendEvent(widgetId, value) {
        fetch('/event', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ sid: SESSION_ID, widget_id: widgetId, value: value })
        }).then(function(response) {
            return response.json();
        }).then(function(patches) {
            if (patches && patches.length > 0) {
                applyPatches(patches);
            }
        }).catch(function(err) {
            console.error('[RustView] Event send failed:', err);
        });
    }

    function debouncedSendEvent(widgetId, value, delay) {
        clearTimeout(debounceTimers[widgetId]);
        debounceTimers[widgetId] = setTimeout(function() {
            sendEvent(widgetId, value);
        }, delay);
    }

    function attachEventListeners() {
        // Text inputs
        document.querySelectorAll('[data-widget-type="text_input"]').forEach(function(el) {
            el.oninput = function() {
                debouncedSendEvent(el.getAttribute('data-widget-id'), el.value, DEBOUNCE_TEXT);
            };
        });

        // Sliders
        document.querySelectorAll('[data-widget-type="slider"]').forEach(function(el) {
            el.oninput = function() {
                debouncedSendEvent(el.getAttribute('data-widget-id'), parseInt(el.value), DEBOUNCE_SLIDER);
            };
        });

        // Checkboxes
        document.querySelectorAll('[data-widget-type="checkbox"]').forEach(function(el) {
            el.onchange = function() {
                sendEvent(el.getAttribute('data-widget-id'), el.checked);
            };
        });

        // Buttons
        document.querySelectorAll('[data-widget-type="button"]').forEach(function(el) {
            el.onclick = function() {
                sendEvent(el.getAttribute('data-widget-id'), true);
            };
        });

        // Number inputs
        document.querySelectorAll('[data-widget-type="number_input"]').forEach(function(el) {
            el.oninput = function() {
                debouncedSendEvent(el.getAttribute('data-widget-id'), parseFloat(el.value) || 0, DEBOUNCE_TEXT);
            };
        });

        // Integer inputs
        document.querySelectorAll('[data-widget-type="int_input"]').forEach(function(el) {
            el.oninput = function() {
                debouncedSendEvent(el.getAttribute('data-widget-id'), parseInt(el.value) || 0, DEBOUNCE_TEXT);
            };
        });

        // Float sliders
        document.querySelectorAll('[data-widget-type="float_slider"]').forEach(function(el) {
            el.oninput = function() {
                debouncedSendEvent(el.getAttribute('data-widget-id'), parseFloat(el.value), DEBOUNCE_SLIDER);
            };
        });

        // Toggles (same as checkbox but different type)
        document.querySelectorAll('[data-widget-type="toggle"]').forEach(function(el) {
            el.onchange = function() {
                sendEvent(el.getAttribute('data-widget-id'), el.checked);
            };
        });

        // Radio buttons
        document.querySelectorAll('[data-widget-type="radio"]').forEach(function(el) {
            el.onchange = function() {
                sendEvent(el.getAttribute('data-widget-id'), el.value);
            };
        });

        // Select dropdowns
        document.querySelectorAll('[data-widget-type="select"]').forEach(function(el) {
            el.onchange = function() {
                sendEvent(el.getAttribute('data-widget-id'), el.value);
            };
        });

        // Multi-select
        document.querySelectorAll('[data-widget-type="multi_select"]').forEach(function(el) {
            el.onchange = function() {
                var selected = Array.from(el.selectedOptions).map(function(opt) { return opt.value; });
                sendEvent(el.getAttribute('data-widget-id'), selected);
            };
        });

        // Text areas
        document.querySelectorAll('[data-widget-type="text_area"]').forEach(function(el) {
            el.oninput = function() {
                debouncedSendEvent(el.getAttribute('data-widget-id'), el.value, DEBOUNCE_TEXT);
            };
        });

        // Color pickers
        document.querySelectorAll('[data-widget-type="color_picker"]').forEach(function(el) {
            el.onchange = function() {
                sendEvent(el.getAttribute('data-widget-id'), el.value);
            };
        });

        // Date pickers
        document.querySelectorAll('[data-widget-type="date_picker"]').forEach(function(el) {
            el.onchange = function() {
                sendEvent(el.getAttribute('data-widget-id'), el.value);
            };
        });

        // File uploads
        document.querySelectorAll('[data-widget-type="file_upload"]').forEach(function(el) {
            el.onchange = function() {
                var file = el.files[0];
                if (!file) return;
                var reader = new FileReader();
                reader.onload = function(e) {
                    // Send as base64 data URI
                    sendEvent(el.getAttribute('data-widget-id'), e.target.result);
                };
                reader.readAsDataURL(file);
            };
        });

        // Image uploads
        document.querySelectorAll('[data-widget-type="image_upload"]').forEach(function(el) {
            el.onchange = function() {
                var file = el.files[0];
                if (!file) return;
                var reader = new FileReader();
                reader.onload = function(e) {
                    sendEvent(el.getAttribute('data-widget-id'), e.target.result);
                };
                reader.readAsDataURL(file);
            };
        });

        // Form submit buttons
        document.querySelectorAll('[data-widget-type="form_submit"]').forEach(function(el) {
            el.onclick = function() {
                // Find parent form container
                var form = el.closest('[data-widget-type="form"]');
                if (!form) return;
                // First, send all widget values within the form
                form.querySelectorAll('[data-widget-id]').forEach(function(w) {
                    var type = w.getAttribute('data-widget-type');
                    if (type === 'form' || type === 'form_submit') return;
                    var val;
                    if (type === 'text_input' || type === 'text_area' || type === 'number_input' || type === 'date_picker' || type === 'color_picker') {
                        val = w.value;
                    } else if (type === 'checkbox' || type === 'toggle') {
                        val = w.checked;
                    } else if (type === 'slider') {
                        val = parseFloat(w.value);
                    } else if (type === 'int_slider') {
                        val = parseInt(w.value, 10);
                    } else if (type === 'select') {
                        val = w.value;
                    }
                    if (val !== undefined) {
                        sendEvent(w.getAttribute('data-widget-id'), val);
                    }
                });
                // Then, send the form submission event
                sendEvent(form.getAttribute('data-widget-id'), true);
            };
        });

        // Expanders
        document.querySelectorAll('[data-widget-type="expander"]').forEach(function(el) {
            el.ontoggle = function() {
                sendEvent(el.getAttribute('data-widget-id'), el.open);
            };
        });

        // Tab buttons
        document.querySelectorAll('[data-widget-type="tab"]').forEach(function(el) {
            el.onclick = function() {
                sendEvent(el.getAttribute('data-widget-id'), parseInt(el.getAttribute('data-tab-index')));
            };
        });

        // Modal trigger buttons
        document.querySelectorAll('[data-widget-type="modal_trigger"]').forEach(function(el) {
            el.onclick = function() {
                sendEvent(el.getAttribute('data-widget-id'), true);
            };
        });

        // Modal close buttons
        document.querySelectorAll('[data-widget-type="modal_close"]').forEach(function(el) {
            el.onclick = function() {
                sendEvent(el.getAttribute('data-widget-id'), false);
            };
        });

        // Modal overlay click-to-close
        document.querySelectorAll('.rustview-modal-overlay').forEach(function(el) {
            el.onclick = function(e) {
                if (e.target === el) {
                    sendEvent(el.getAttribute('data-widget-id'), false);
                }
            };
        });

        // Stack toasts vertically
        var toasts = document.querySelectorAll('.rustview-toast');
        var topOffset = 16; // 1rem in px
        toasts.forEach(function(el) {
            el.style.top = topOffset + 'px';
            topOffset += el.offsetHeight + 8; // 0.5rem gap
        });

        // Toast auto-dismiss
        document.querySelectorAll('.rustview-toast').forEach(function(el) {
            setTimeout(function() {
                if (el.parentNode) { el.parentNode.removeChild(el); }
            }, 5000);
        });
    }

    // Initialize
    connectSSE();
    attachEventListeners();
})();
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = RustViewConfig::default();
        assert_eq!(config.bind.to_string(), "127.0.0.1:8501");
        assert!(config.bind.ip().is_loopback());
        assert_eq!(config.title, "RustView App");
        assert_eq!(config.session_ttl_secs, 86400);
        assert_eq!(config.max_upload_bytes, 52_428_800);
    }

    #[test]
    fn test_vnode_to_html_simple() {
        let node = VNode::new("test", "p").with_text("Hello");
        let html = vnode_to_inner_html(&node);
        assert!(html.contains("<p"));
        assert!(html.contains("Hello"));
        assert!(html.contains("</p>"));
    }

    #[test]
    fn test_html_escape() {
        assert_eq!(html_escape("<script>"), "&lt;script&gt;");
        assert_eq!(html_escape("a & b"), "a &amp; b");
        assert_eq!(html_escape("\"quoted\""), "&quot;quoted&quot;");
    }

    #[test]
    fn test_vnode_to_html_with_attrs() {
        let node = VNode::new("test", "div")
            .with_attr("class", "my-class")
            .with_text("content");
        let html = vnode_to_inner_html(&node);
        assert!(html.contains("class=\"my-class\""));
    }

    #[test]
    fn test_vnode_to_html_nested() {
        let node =
            VNode::new("parent", "div").with_child(VNode::new("child", "span").with_text("inner"));
        let html = vnode_to_inner_html(&node);
        assert!(html.contains("<span"));
        assert!(html.contains("inner"));
    }

    #[test]
    fn test_vnode_to_html_self_closing() {
        let node = VNode::new("input1", "input").with_attr("type", "text");
        let html = vnode_to_inner_html(&node);
        assert!(!html.contains("</input>"));
    }

    #[test]
    fn test_build_router() {
        let _router = build_router(|ui| {
            ui.write("hello");
        });
        // Just verify it compiles and doesn't panic
    }

    #[test]
    fn test_theme_default() {
        let theme = Theme::default();
        assert_eq!(theme.background, "#0e1117");
        assert_eq!(theme.primary, "#ff4b4b");
    }

    #[test]
    fn test_theme_to_css_vars() {
        let theme = Theme::default();
        let css = theme.to_css_vars();
        assert!(css.contains("--rustview-bg: #0e1117"));
        assert!(css.contains("--rustview-primary: #ff4b4b"));
        assert!(css.contains("--rustview-fg: #fafafa"));
    }

    #[test]
    fn test_theme_custom_colors() {
        let theme = Theme {
            background: "#ffffff".to_string(),
            foreground: "#000000".to_string(),
            primary: "#0066ff".to_string(),
            ..Theme::default()
        };
        let css = theme.to_css_vars();
        assert!(css.contains("--rustview-bg: #ffffff"));
        assert!(css.contains("--rustview-fg: #000000"));
        assert!(css.contains("--rustview-primary: #0066ff"));
    }

    #[test]
    fn test_config_default_has_theme() {
        let config = RustViewConfig::default();
        assert_eq!(config.theme.background, "#0e1117");
    }

    #[test]
    fn test_layout_default() {
        let layout = Layout::default();
        assert_eq!(layout.max_width_percent, 0);
        assert_eq!(layout.padding, "2rem");
    }

    #[test]
    fn test_layout_to_css_vars_default() {
        let layout = Layout::default();
        let css = layout.to_css_vars();
        // Default (0) should NOT emit --rustview-max-width, letting CSS 800px apply
        assert!(!css.contains("--rustview-max-width"));
        assert!(css.contains("--rustview-padding: 2rem"));
    }

    #[test]
    fn test_layout_custom() {
        let layout = Layout {
            max_width_percent: 80,
            padding: "3rem 1rem".into(),
        };
        let css = layout.to_css_vars();
        assert!(css.contains("--rustview-max-width: 80%"));
        assert!(css.contains("--rustview-padding: 3rem 1rem"));
    }

    #[test]
    fn test_layout_clamps_percentage() {
        let layout = Layout {
            max_width_percent: 0,
            ..Default::default()
        };
        // 0 means use CSS default — no max-width override
        let css = layout.to_css_vars();
        assert!(!css.contains("--rustview-max-width"));
    }

    #[test]
    fn test_layout_builder_max_width_only() {
        let layout = Layout::default().with_max_width(80);
        assert_eq!(layout.max_width_percent, 80);
        assert_eq!(layout.padding, "2rem"); // default preserved
        let css = layout.to_css_vars();
        assert!(css.contains("--rustview-max-width: 80%"));
        assert!(css.contains("--rustview-padding: 2rem"));
    }

    #[test]
    fn test_layout_builder_max_width_clamps() {
        let layout = Layout::default().with_max_width(0);
        assert_eq!(layout.max_width_percent, 1); // clamped to 1
        let layout = Layout::default().with_max_width(255);
        assert_eq!(layout.max_width_percent, 100); // clamped to 100
    }

    #[test]
    fn test_layout_builder_padding_only() {
        let layout = Layout::default().with_padding("3rem 1rem");
        assert_eq!(layout.max_width_percent, 0); // default preserved
        assert_eq!(layout.padding, "3rem 1rem");
    }

    #[test]
    fn test_layout_builder_chained() {
        let layout = Layout::default().with_max_width(60).with_padding("1rem");
        assert_eq!(layout.max_width_percent, 60);
        assert_eq!(layout.padding, "1rem");
    }

    #[test]
    fn test_layout_struct_update_partial() {
        // User can set only max_width_percent, padding gets default
        let layout = Layout {
            max_width_percent: 80,
            ..Default::default()
        };
        assert_eq!(layout.max_width_percent, 80);
        assert_eq!(layout.padding, "2rem");
    }

    #[test]
    fn test_config_default_has_layout() {
        let config = RustViewConfig::default();
        assert_eq!(config.layout.max_width_percent, 0);
        assert_eq!(config.layout.padding, "2rem");
    }

    #[test]
    fn test_css_contains_layout_vars() {
        assert!(CSS.contains("--rustview-max-width"));
        assert!(CSS.contains("--rustview-padding"));
        assert!(CSS.contains("max-width: var(--rustview-max-width)"));
        assert!(CSS.contains("padding: var(--rustview-padding)"));
    }
}
