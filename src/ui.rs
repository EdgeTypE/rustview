/// The Ui context — the core API surface for RustView applications.
///
/// Users interact with RustView exclusively through `&mut Ui`. Widget calls
/// return their current values (widget-as-variable pattern). Each call
/// appends a VNode to the internal tree.
use crate::session::Session;
use crate::vdom::VNode;
use crate::widgets;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Display;
use std::mem;

/// Default number of discrete steps for float sliders.
const DEFAULT_SLIDER_STEPS: f64 = 100.0;

/// The main UI context passed to app functions.
///
/// Holds a reference to the current session and builds a VNode tree
/// as widgets are called.
pub struct Ui<'a> {
    /// The session for the current browser tab.
    session: &'a mut Session,
    /// The VNode tree being built during this re-run.
    children: Vec<VNode>,
    /// Widget counter for auto-generating unique IDs.
    widget_counter: u64,
    /// Optional key override for the next widget ID.
    key_override: Option<String>,
}

impl<'a> Ui<'a> {
    /// Create a new Ui context for a session.
    pub fn new(session: &'a mut Session) -> Self {
        Ui {
            session,
            children: Vec::new(),
            widget_counter: 0,
            key_override: None,
        }
    }

    /// Generate a unique widget ID based on the label.
    fn next_widget_id(&mut self, label: &str) -> String {
        self.widget_counter += 1;
        if let Some(key) = self.key_override.take() {
            return key;
        }
        // Sanitize label for use in IDs
        let sanitized: String = label
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '_' })
            .collect();
        format!("w-{}-{}", sanitized, self.widget_counter)
    }

    /// Set a stable key for the next widget.
    ///
    /// Use this when you have conditional widget rendering and need
    /// stable widget IDs across re-runs.
    ///
    /// # Example
    /// ```rust
    /// # use rustview::prelude::*;
    /// # let mut session = rustview::session::Session::new();
    /// # let mut ui = Ui::new(&mut session);
    /// ui.with_key("user_name");
    /// let name = ui.text_input("Name", "");
    /// ```
    pub fn with_key(&mut self, key: &str) -> &mut Self {
        self.key_override = Some(format!("k-{key}"));
        self
    }

    // ---- Input Widgets ----

    /// Create a text input widget.
    ///
    /// Returns the current text value.
    ///
    /// # Example
    /// ```rust
    /// # use rustview::prelude::*;
    /// # let mut session = rustview::session::Session::new();
    /// # let mut ui = Ui::new(&mut session);
    /// let name = ui.text_input("Your name", "World");
    /// ```
    pub fn text_input(&mut self, label: &str, default: &str) -> String {
        let widget_id = self.next_widget_id(label);
        let value = self
            .session
            .get_widget_value(&widget_id, serde_json::json!(default));
        let text = value.as_str().unwrap_or(default).to_string();
        self.children
            .push(widgets::render_text_input(&widget_id, label, &text));
        text
    }

    /// Create an integer slider widget.
    ///
    /// Returns the current slider value.
    ///
    /// # Example
    /// ```rust
    /// # use rustview::prelude::*;
    /// # let mut session = rustview::session::Session::new();
    /// # let mut ui = Ui::new(&mut session);
    /// let count = ui.int_slider("Repeat", 1..=10);
    /// ```
    pub fn int_slider(&mut self, label: &str, range: std::ops::RangeInclusive<i64>) -> i64 {
        let widget_id = self.next_widget_id(label);
        let min = *range.start();
        let max = *range.end();
        let default = min;
        let value = self
            .session
            .get_widget_value(&widget_id, serde_json::json!(default));
        let num = value.as_i64().unwrap_or(default).clamp(min, max);
        self.children
            .push(widgets::render_slider(&widget_id, label, num, min, max));
        num
    }

    /// Create a checkbox widget.
    ///
    /// Returns the current checked state.
    ///
    /// # Example
    /// ```rust
    /// # use rustview::prelude::*;
    /// # let mut session = rustview::session::Session::new();
    /// # let mut ui = Ui::new(&mut session);
    /// let enabled = ui.checkbox("Enable feature", false);
    /// ```
    pub fn checkbox(&mut self, label: &str, default: bool) -> bool {
        let widget_id = self.next_widget_id(label);
        let value = self
            .session
            .get_widget_value(&widget_id, serde_json::json!(default));
        let checked = value.as_bool().unwrap_or(default);
        self.children
            .push(widgets::render_checkbox(&widget_id, label, checked));
        checked
    }

    /// Create a button widget.
    ///
    /// Returns `true` if the button was clicked in this interaction.
    ///
    /// # Example
    /// ```rust
    /// # use rustview::prelude::*;
    /// # let mut session = rustview::session::Session::new();
    /// # let mut ui = Ui::new(&mut session);
    /// if ui.button("Submit") {
    ///     // handle click
    /// }
    /// ```
    pub fn button(&mut self, label: &str) -> bool {
        let widget_id = self.next_widget_id(label);
        let value = self
            .session
            .get_widget_value(&widget_id, serde_json::json!(false));
        let clicked = value.as_bool().unwrap_or(false);
        self.children
            .push(widgets::render_button(&widget_id, label));
        // Reset button state after reading (buttons are transient)
        self.session
            .set_widget_value(&widget_id, serde_json::json!(false));
        clicked
    }

    /// Create a float number input widget.
    ///
    /// Returns the current numeric value.
    ///
    /// # Example
    /// ```rust
    /// # use rustview::prelude::*;
    /// # let mut session = rustview::session::Session::new();
    /// # let mut ui = Ui::new(&mut session);
    /// let price = ui.number_input("Price", 9.99);
    /// ```
    pub fn number_input(&mut self, label: &str, default: f64) -> f64 {
        let widget_id = self.next_widget_id(label);
        let value = self
            .session
            .get_widget_value(&widget_id, serde_json::json!(default));
        let num = value.as_f64().unwrap_or(default);
        self.children
            .push(widgets::render_number_input(&widget_id, label, num));
        num
    }

    /// Create an integer input widget.
    ///
    /// Returns the current integer value.
    ///
    /// # Example
    /// ```rust
    /// # use rustview::prelude::*;
    /// # let mut session = rustview::session::Session::new();
    /// # let mut ui = Ui::new(&mut session);
    /// let count = ui.int_input("Count", 42);
    /// ```
    pub fn int_input(&mut self, label: &str, default: i64) -> i64 {
        let widget_id = self.next_widget_id(label);
        let value = self
            .session
            .get_widget_value(&widget_id, serde_json::json!(default));
        let num = value.as_i64().unwrap_or(default);
        self.children
            .push(widgets::render_int_input(&widget_id, label, num));
        num
    }

    /// Create a float slider widget.
    ///
    /// Returns the current slider value.
    ///
    /// # Example
    /// ```rust
    /// # use rustview::prelude::*;
    /// # let mut session = rustview::session::Session::new();
    /// # let mut ui = Ui::new(&mut session);
    /// let weight = ui.slider("Weight", 0.0..=1.0);
    /// ```
    pub fn slider(&mut self, label: &str, range: std::ops::RangeInclusive<f64>) -> f64 {
        let widget_id = self.next_widget_id(label);
        let min = *range.start();
        let max = *range.end();
        let step = (max - min) / DEFAULT_SLIDER_STEPS;
        let default = min;
        let value = self
            .session
            .get_widget_value(&widget_id, serde_json::json!(default));
        let num = value.as_f64().unwrap_or(default).clamp(min, max);
        self.children.push(widgets::render_float_slider(
            &widget_id, label, num, min, max, step,
        ));
        num
    }

    /// Create a toggle (switch) widget.
    ///
    /// Returns the current on/off state.
    ///
    /// # Example
    /// ```rust
    /// # use rustview::prelude::*;
    /// # let mut session = rustview::session::Session::new();
    /// # let mut ui = Ui::new(&mut session);
    /// let dark = ui.toggle("Dark mode", false);
    /// ```
    pub fn toggle(&mut self, label: &str, default: bool) -> bool {
        let widget_id = self.next_widget_id(label);
        let value = self
            .session
            .get_widget_value(&widget_id, serde_json::json!(default));
        let checked = value.as_bool().unwrap_or(default);
        self.children
            .push(widgets::render_toggle(&widget_id, label, checked));
        checked
    }

    /// Create a radio button group.
    ///
    /// Returns the currently selected option.
    ///
    /// # Example
    /// ```rust
    /// # use rustview::prelude::*;
    /// # let mut session = rustview::session::Session::new();
    /// # let mut ui = Ui::new(&mut session);
    /// let size = ui.radio("Size", &["S", "M", "L"]);
    /// ```
    pub fn radio(&mut self, label: &str, options: &[&str]) -> String {
        let widget_id = self.next_widget_id(label);
        let default = options.first().copied().unwrap_or("");
        let value = self
            .session
            .get_widget_value(&widget_id, serde_json::json!(default));
        let selected = value.as_str().unwrap_or(default).to_string();
        self.children
            .push(widgets::render_radio(&widget_id, label, options, &selected));
        selected
    }

    /// Create a dropdown select widget.
    ///
    /// Returns the currently selected option.
    ///
    /// # Example
    /// ```rust
    /// # use rustview::prelude::*;
    /// # let mut session = rustview::session::Session::new();
    /// # let mut ui = Ui::new(&mut session);
    /// let country = ui.select("Country", &["US", "UK", "DE"]);
    /// ```
    pub fn select(&mut self, label: &str, options: &[&str]) -> String {
        let widget_id = self.next_widget_id(label);
        let default = options.first().copied().unwrap_or("");
        let value = self
            .session
            .get_widget_value(&widget_id, serde_json::json!(default));
        let selected = value.as_str().unwrap_or(default).to_string();
        self.children.push(widgets::render_select(
            &widget_id, label, options, &selected,
        ));
        selected
    }

    /// Alias for [`select`](Self::select) — Streamlit compatibility.
    ///
    /// # Example
    /// ```rust
    /// # use rustview::prelude::*;
    /// # let mut session = rustview::session::Session::new();
    /// # let mut ui = Ui::new(&mut session);
    /// let choice = ui.selectbox("Pick one", &["A", "B", "C"]);
    /// ```
    pub fn selectbox(&mut self, label: &str, options: &[&str]) -> String {
        self.select(label, options)
    }

    /// Create a multi-select widget.
    ///
    /// Returns a list of currently selected options.
    ///
    /// # Example
    /// ```rust
    /// # use rustview::prelude::*;
    /// # let mut session = rustview::session::Session::new();
    /// # let mut ui = Ui::new(&mut session);
    /// let tags = ui.multi_select("Tags", &["rust", "python", "go"]);
    /// ```
    pub fn multi_select(&mut self, label: &str, options: &[&str]) -> Vec<String> {
        let widget_id = self.next_widget_id(label);
        let default: Vec<String> = Vec::new();
        let value = self
            .session
            .get_widget_value(&widget_id, serde_json::json!(default));
        let selected: Vec<String> = serde_json::from_value(value).unwrap_or_default();
        self.children.push(widgets::render_multi_select(
            &widget_id, label, options, &selected,
        ));
        selected
    }

    /// Create a text area widget.
    ///
    /// Returns the current text content.
    ///
    /// # Example
    /// ```rust
    /// # use rustview::prelude::*;
    /// # let mut session = rustview::session::Session::new();
    /// # let mut ui = Ui::new(&mut session);
    /// let bio = ui.text_area("Bio", "Tell us about yourself", 5);
    /// ```
    pub fn text_area(&mut self, label: &str, default: &str, rows: u32) -> String {
        let widget_id = self.next_widget_id(label);
        let value = self
            .session
            .get_widget_value(&widget_id, serde_json::json!(default));
        let text = value.as_str().unwrap_or(default).to_string();
        self.children
            .push(widgets::render_text_area(&widget_id, label, &text, rows));
        text
    }

    /// Create a color picker widget.
    ///
    /// Returns a hex color string like `"#ff0000"`.
    ///
    /// # Example
    /// ```rust
    /// # use rustview::prelude::*;
    /// # let mut session = rustview::session::Session::new();
    /// # let mut ui = Ui::new(&mut session);
    /// let color = ui.color_picker("Theme color");
    /// ```
    pub fn color_picker(&mut self, label: &str) -> String {
        let widget_id = self.next_widget_id(label);
        let default = "#000000";
        let value = self
            .session
            .get_widget_value(&widget_id, serde_json::json!(default));
        let color = value.as_str().unwrap_or(default).to_string();
        self.children
            .push(widgets::render_color_picker(&widget_id, label, &color));
        color
    }

    /// Create a date picker widget.
    ///
    /// Returns the selected date as a string in `"YYYY-MM-DD"` format.
    /// Default is an empty string (no date selected).
    ///
    /// # Example
    /// ```rust
    /// # use rustview::prelude::*;
    /// # let mut session = rustview::session::Session::new();
    /// # let mut ui = Ui::new(&mut session);
    /// let date = ui.date_picker("Start date");
    /// ```
    pub fn date_picker(&mut self, label: &str) -> String {
        let widget_id = self.next_widget_id(label);
        let default = "";
        let value = self
            .session
            .get_widget_value(&widget_id, serde_json::json!(default));
        let date = value.as_str().unwrap_or(default).to_string();
        self.children
            .push(widgets::render_date_picker(&widget_id, label, &date));
        date
    }

    /// Create a file upload widget.
    ///
    /// Returns the uploaded file contents as base64-encoded data, or an empty
    /// string if no file has been uploaded yet.
    ///
    /// # Example
    /// ```rust
    /// # use rustview::prelude::*;
    /// # let mut session = rustview::session::Session::new();
    /// # let mut ui = Ui::new(&mut session);
    /// let file_data = ui.file_upload("Upload CSV");
    /// if !file_data.is_empty() {
    ///     ui.write(format!("Received {} bytes", file_data.len()));
    /// }
    /// ```
    pub fn file_upload(&mut self, label: &str) -> String {
        let widget_id = self.next_widget_id(label);
        let default = "";
        let value = self
            .session
            .get_widget_value(&widget_id, serde_json::json!(default));
        let data = value.as_str().unwrap_or(default).to_string();
        self.children
            .push(widgets::render_file_upload(&widget_id, label));
        data
    }

    /// Create an image upload widget with preview.
    ///
    /// Returns the uploaded image as a base64 data URI, or an empty string
    /// if no image has been uploaded yet. Only accepts image files.
    ///
    /// # Example
    /// ```rust
    /// # use rustview::prelude::*;
    /// # let mut session = rustview::session::Session::new();
    /// # let mut ui = Ui::new(&mut session);
    /// let img_data = ui.image_upload("Upload photo");
    /// if !img_data.is_empty() {
    ///     ui.image(&img_data, "Uploaded photo");
    /// }
    /// ```
    pub fn image_upload(&mut self, label: &str) -> String {
        let widget_id = self.next_widget_id(label);
        let default = "";
        let value = self
            .session
            .get_widget_value(&widget_id, serde_json::json!(default));
        let data = value.as_str().unwrap_or(default).to_string();
        self.children
            .push(widgets::render_image_upload(&widget_id, label, &data));
        data
    }

    /// Display an image from a URL or base64-encoded data URI.
    ///
    /// # Example
    /// ```rust
    /// # use rustview::prelude::*;
    /// # let mut session = rustview::session::Session::new();
    /// # let mut ui = Ui::new(&mut session);
    /// ui.image("https://example.com/logo.png", "Company Logo");
    /// ```
    pub fn image(&mut self, src: &str, caption: &str) {
        let widget_id = self.next_widget_id(caption);
        self.children
            .push(widgets::render_image(&widget_id, src, caption));
    }

    /// Display an audio player from a URL or base64-encoded data URI.
    ///
    /// # Example
    /// ```rust
    /// # use rustview::prelude::*;
    /// # let mut session = rustview::session::Session::new();
    /// # let mut ui = Ui::new(&mut session);
    /// ui.audio("https://example.com/clip.mp3", "mp3");
    /// ```
    pub fn audio(&mut self, src: &str, format: &str) {
        let widget_id = self.next_widget_id("audio");
        self.children
            .push(widgets::render_audio(&widget_id, src, format));
    }

    /// Display a video player from a URL or base64-encoded data URI.
    ///
    /// Supported formats: `"mp4"`, `"webm"`, `"ogg"`.
    ///
    /// # Example
    /// ```rust
    /// # use rustview::prelude::*;
    /// # let mut session = rustview::session::Session::new();
    /// # let mut ui = Ui::new(&mut session);
    /// ui.video("https://example.com/clip.mp4", "mp4");
    /// ```
    pub fn video(&mut self, src: &str, format: &str) {
        let widget_id = self.next_widget_id("video");
        self.children
            .push(widgets::render_video(&widget_id, src, format));
    }

    /// Create a download button widget.
    ///
    /// The `data` is served as a base64-encoded data URI.
    ///
    /// # Example
    /// ```rust
    /// # use rustview::prelude::*;
    /// # let mut session = rustview::session::Session::new();
    /// # let mut ui = Ui::new(&mut session);
    /// ui.download_button("Export CSV", "a,b,c".as_bytes(), "data.csv");
    /// ```
    pub fn download_button(&mut self, label: &str, data: &[u8], filename: &str) {
        let widget_id = self.next_widget_id(label);
        use base64::Engine;
        let b64 = base64::engine::general_purpose::STANDARD.encode(data);
        let data_uri = format!("data:application/octet-stream;base64,{}", b64);
        self.children.push(widgets::render_download_button(
            &widget_id, label, &data_uri, filename,
        ));
    }

    /// Display a hyperlink.
    ///
    /// Opens in a new tab with `target="_blank"` and `rel="noopener noreferrer"`.
    ///
    /// # Example
    /// ```rust
    /// # use rustview::prelude::*;
    /// # let mut session = rustview::session::Session::new();
    /// # let mut ui = Ui::new(&mut session);
    /// ui.link("RustView Docs", "https://github.com/example/rustview");
    /// ```
    pub fn link(&mut self, text: &str, url: &str) {
        let widget_id = self.next_widget_id(text);
        self.children
            .push(widgets::render_link(&widget_id, text, url));
    }

    // ---- Output / Display Widgets ----

    /// Universal display widget — renders any `Display` type as text.
    ///
    /// # Example
    /// ```rust
    /// # use rustview::prelude::*;
    /// # let mut session = rustview::session::Session::new();
    /// # let mut ui = Ui::new(&mut session);
    /// ui.write(format!("Hello, {}!", "World"));
    /// ui.write(42);
    /// ```
    pub fn write(&mut self, value: impl Display) {
        let widget_id = self.next_widget_id("write");
        let text = value.to_string();
        self.children.push(widgets::render_write(&widget_id, &text));
    }

    /// Display markdown text.
    ///
    /// # Example
    /// ```rust
    /// # use rustview::prelude::*;
    /// # let mut session = rustview::session::Session::new();
    /// # let mut ui = Ui::new(&mut session);
    /// ui.markdown("# Hello\n\nThis is **bold**.");
    /// ```
    pub fn markdown(&mut self, text: &str) {
        let widget_id = self.next_widget_id("markdown");
        self.children
            .push(widgets::render_markdown(&widget_id, text));
    }

    /// Display a progress bar.
    ///
    /// `value` should be between 0.0 and 1.0.
    ///
    /// # Example
    /// ```rust
    /// # use rustview::prelude::*;
    /// # let mut session = rustview::session::Session::new();
    /// # let mut ui = Ui::new(&mut session);
    /// ui.progress(0.75);
    /// ```
    pub fn progress(&mut self, value: f64) {
        let widget_id = self.next_widget_id("progress");
        self.children
            .push(widgets::render_progress(&widget_id, value));
    }

    /// Display an error message.
    ///
    /// # Example
    /// ```rust
    /// # use rustview::prelude::*;
    /// # let mut session = rustview::session::Session::new();
    /// # let mut ui = Ui::new(&mut session);
    /// ui.error("Something went wrong!");
    /// ```
    pub fn error(&mut self, message: &str) {
        let widget_id = self.next_widget_id("error");
        self.children
            .push(widgets::render_error(&widget_id, message));
    }

    /// Display a heading (h1).
    ///
    /// # Example
    /// ```rust
    /// # use rustview::prelude::*;
    /// # let mut session = rustview::session::Session::new();
    /// # let mut ui = Ui::new(&mut session);
    /// ui.heading("Dashboard");
    /// ```
    pub fn heading(&mut self, text: &str) {
        let widget_id = self.next_widget_id("heading");
        self.children
            .push(widgets::render_heading(&widget_id, text));
    }

    /// Display a subheading (h2).
    ///
    /// # Example
    /// ```rust
    /// # use rustview::prelude::*;
    /// # let mut session = rustview::session::Session::new();
    /// # let mut ui = Ui::new(&mut session);
    /// ui.subheading("Details");
    /// ```
    pub fn subheading(&mut self, text: &str) {
        let widget_id = self.next_widget_id("subheading");
        self.children
            .push(widgets::render_subheading(&widget_id, text));
    }

    /// Display a caption (small text).
    ///
    /// # Example
    /// ```rust
    /// # use rustview::prelude::*;
    /// # let mut session = rustview::session::Session::new();
    /// # let mut ui = Ui::new(&mut session);
    /// ui.caption("Last updated 5 minutes ago");
    /// ```
    pub fn caption(&mut self, text: &str) {
        let widget_id = self.next_widget_id("caption");
        self.children
            .push(widgets::render_caption(&widget_id, text));
    }

    /// Display a code block with syntax highlighting hint.
    ///
    /// # Example
    /// ```rust
    /// # use rustview::prelude::*;
    /// # let mut session = rustview::session::Session::new();
    /// # let mut ui = Ui::new(&mut session);
    /// ui.code("fn main() {}", "rust");
    /// ```
    pub fn code(&mut self, source: &str, language: &str) {
        let widget_id = self.next_widget_id("code");
        self.children
            .push(widgets::render_code(&widget_id, source, language));
    }

    /// Display a JSON value in a formatted code block.
    ///
    /// # Example
    /// ```rust
    /// # use rustview::prelude::*;
    /// # let mut session = rustview::session::Session::new();
    /// # let mut ui = Ui::new(&mut session);
    /// ui.json(&serde_json::json!({"key": "value"}));
    /// ```
    pub fn json(&mut self, value: &impl Serialize) {
        let widget_id = self.next_widget_id("json");
        let json_text = serde_json::to_string_pretty(value).unwrap_or_else(|_| "{}".to_string());
        self.children
            .push(widgets::render_json(&widget_id, &json_text));
    }

    /// Display a table with headers and rows.
    ///
    /// # Example
    /// ```rust
    /// # use rustview::prelude::*;
    /// # let mut session = rustview::session::Session::new();
    /// # let mut ui = Ui::new(&mut session);
    /// ui.table(&["Name", "Age"], &[vec!["Alice".into(), "30".into()]]);
    /// ```
    pub fn table(&mut self, headers: &[&str], rows: &[Vec<String>]) {
        let widget_id = self.next_widget_id("table");
        self.children
            .push(widgets::render_table(&widget_id, headers, rows));
    }

    /// Display a dataframe — an enhanced table with row numbers, column types,
    /// an optional title, and scrollable overflow.
    ///
    /// `columns` is a slice of `(name, type_name)` pairs.
    /// `rows` is a slice of rows, each a `Vec<String>`.
    ///
    /// # Example
    /// ```rust
    /// # use rustview::prelude::*;
    /// # let mut session = rustview::session::Session::new();
    /// # let mut ui = Ui::new(&mut session);
    /// ui.dataframe(
    ///     &[("Name", "str"), ("Age", "i64")],
    ///     &[
    ///         vec!["Alice".into(), "30".into()],
    ///         vec!["Bob".into(), "25".into()],
    ///     ],
    ///     Some("Users"),
    /// );
    /// ```
    pub fn dataframe(
        &mut self,
        columns: &[(&str, &str)],
        rows: &[Vec<String>],
        title: Option<&str>,
    ) {
        let widget_id = self.next_widget_id("dataframe");
        self.children
            .push(widgets::render_dataframe(&widget_id, columns, rows, title));
    }

    /// Display a spinner with a label.
    ///
    /// # Example
    /// ```rust
    /// # use rustview::prelude::*;
    /// # let mut session = rustview::session::Session::new();
    /// # let mut ui = Ui::new(&mut session);
    /// ui.spinner("Loading...");
    /// ```
    pub fn spinner(&mut self, label: &str) {
        let widget_id = self.next_widget_id("spinner");
        self.children
            .push(widgets::render_spinner(&widget_id, label));
    }

    /// Display a metric tile with label, value, and optional delta.
    ///
    /// # Example
    /// ```rust
    /// # use rustview::prelude::*;
    /// # let mut session = rustview::session::Session::new();
    /// # let mut ui = Ui::new(&mut session);
    /// ui.metric("Users", 1234, Some(5.2));
    /// ```
    pub fn metric(&mut self, label: &str, value: impl Display, delta: Option<f64>) {
        let widget_id = self.next_widget_id("metric");
        let value_str = value.to_string();
        self.children
            .push(widgets::render_metric(&widget_id, label, &value_str, delta));
    }

    /// Display a success alert.
    ///
    /// # Example
    /// ```rust
    /// # use rustview::prelude::*;
    /// # let mut session = rustview::session::Session::new();
    /// # let mut ui = Ui::new(&mut session);
    /// ui.success("Operation completed!");
    /// ```
    pub fn success(&mut self, message: &str) {
        let widget_id = self.next_widget_id("success");
        self.children
            .push(widgets::render_alert(&widget_id, message, "success"));
    }

    /// Display a warning alert.
    ///
    /// # Example
    /// ```rust
    /// # use rustview::prelude::*;
    /// # let mut session = rustview::session::Session::new();
    /// # let mut ui = Ui::new(&mut session);
    /// ui.warning("Disk space low.");
    /// ```
    pub fn warning(&mut self, message: &str) {
        let widget_id = self.next_widget_id("warning");
        self.children
            .push(widgets::render_alert(&widget_id, message, "warning"));
    }

    /// Display an info alert.
    ///
    /// # Example
    /// ```rust
    /// # use rustview::prelude::*;
    /// # let mut session = rustview::session::Session::new();
    /// # let mut ui = Ui::new(&mut session);
    /// ui.info("New version available.");
    /// ```
    pub fn info(&mut self, message: &str) {
        let widget_id = self.next_widget_id("info");
        self.children
            .push(widgets::render_alert(&widget_id, message, "info"));
    }

    /// Display a toast notification that auto-dismisses after a few seconds.
    ///
    /// `level` can be `"success"`, `"info"`, `"warning"`, or `"error"`.
    ///
    /// # Example
    /// ```rust
    /// # use rustview::prelude::*;
    /// # let mut session = rustview::session::Session::new();
    /// # let mut ui = Ui::new(&mut session);
    /// ui.toast("Saved!", "success");
    /// ```
    pub fn toast(&mut self, message: &str, level: &str) {
        let widget_id = self.next_widget_id("toast");
        self.children
            .push(widgets::render_toast(&widget_id, message, level));
    }

    /// Display a horizontal divider.
    ///
    /// # Example
    /// ```rust
    /// # use rustview::prelude::*;
    /// # let mut session = rustview::session::Session::new();
    /// # let mut ui = Ui::new(&mut session);
    /// ui.divider();
    /// ```
    pub fn divider(&mut self) {
        let widget_id = self.next_widget_id("divider");
        self.children.push(widgets::render_divider(&widget_id));
    }

    /// Display a colored badge/label.
    ///
    /// Predefined colors: `"red"`, `"green"`, `"blue"`, `"yellow"`, `"gray"`, `"purple"`, `"orange"`.
    /// You can also pass any valid CSS color value.
    ///
    /// # Example
    /// ```rust
    /// # use rustview::prelude::*;
    /// # let mut session = rustview::session::Session::new();
    /// # let mut ui = Ui::new(&mut session);
    /// ui.badge("Active", "green");
    /// ui.badge("Pending", "yellow");
    /// ui.badge("Error", "red");
    /// ```
    pub fn badge(&mut self, text: &str, color: &str) {
        let widget_id = self.next_widget_id(text);
        self.children
            .push(widgets::render_badge(&widget_id, text, color));
    }

    // ---- Session State ----

    /// Get typed session state value.
    ///
    /// Returns the current value for the key, or the default if not set.
    ///
    /// # Example
    /// ```rust
    /// # use rustview::prelude::*;
    /// # let mut session = rustview::session::Session::new();
    /// # let mut ui = Ui::new(&mut session);
    /// let count = ui.get_state::<i64>("counter", 0);
    /// if ui.button("Increment") {
    ///     ui.set_state("counter", count + 1);
    /// }
    /// ui.write(format!("Count: {}", ui.get_state::<i64>("counter", 0)));
    /// ```
    pub fn get_state<T>(&self, key: &str, default: T) -> T
    where
        T: Serialize + DeserializeOwned + Clone + 'static,
    {
        match self.session.get_state(key) {
            Some(val) => serde_json::from_value(val.clone()).unwrap_or_else(|e| {
                tracing::warn!(
                    key = key,
                    "Failed to deserialize session state, using default: {}",
                    e
                );
                default.clone()
            }),
            None => default,
        }
    }

    /// Set typed session state value.
    ///
    /// Stores the value for the key, persisted across re-runs within the session.
    pub fn set_state<T>(&mut self, key: &str, value: T)
    where
        T: Serialize + 'static,
    {
        if let Ok(val) = serde_json::to_value(&value) {
            self.session.set_state(key, val);
        }
    }

    // ---- Layout ----

    /// Render widgets inside a generic container.
    ///
    /// # Example
    /// ```rust
    /// # use rustview::prelude::*;
    /// # let mut session = rustview::session::Session::new();
    /// # let mut ui = Ui::new(&mut session);
    /// ui.container(|inner| {
    ///     inner.write("Inside a container");
    /// });
    /// ```
    pub fn container(&mut self, f: impl FnOnce(&mut Ui)) {
        let widget_id = self.next_widget_id("container");
        let prev = mem::take(&mut self.children);
        f(self);
        let inner = mem::replace(&mut self.children, prev);
        let mut node = widgets::render_container(&widget_id);
        node.children = inner;
        self.children.push(node);
    }

    /// Render widgets inside a sidebar panel.
    ///
    /// # Example
    /// ```rust
    /// # use rustview::prelude::*;
    /// # let mut session = rustview::session::Session::new();
    /// # let mut ui = Ui::new(&mut session);
    /// ui.sidebar(|sb| {
    ///     sb.write("Sidebar content");
    /// });
    /// ```
    pub fn sidebar(&mut self, f: impl FnOnce(&mut Ui)) {
        let widget_id = self.next_widget_id("sidebar");
        let prev = mem::take(&mut self.children);
        f(self);
        let inner = mem::replace(&mut self.children, prev);
        let mut node = widgets::render_sidebar(&widget_id);
        node.children = inner;
        self.children.push(node);
    }

    /// Render widgets inside a collapsible expander section.
    ///
    /// # Example
    /// ```rust
    /// # use rustview::prelude::*;
    /// # let mut session = rustview::session::Session::new();
    /// # let mut ui = Ui::new(&mut session);
    /// ui.expander("Advanced", |inner| {
    ///     inner.write("Hidden until expanded");
    /// });
    /// ```
    pub fn expander(&mut self, label: &str, f: impl FnOnce(&mut Ui)) {
        let widget_id = self.next_widget_id(label);
        let value = self
            .session
            .get_widget_value(&widget_id, serde_json::json!(false));
        let open = value.as_bool().unwrap_or(false);

        let prev = mem::take(&mut self.children);
        f(self);
        let inner = mem::replace(&mut self.children, prev);

        let mut node = widgets::render_expander(&widget_id, label, open);
        // Structure: div.rustview-expander > details > [summary, div.content]
        if let Some(details) = node.children.get_mut(0) {
            if let Some(content_div) = details.children.get_mut(1) {
                content_div.children = inner;
            }
        }
        self.children.push(node);
    }

    /// Create side-by-side columns.
    ///
    /// Provide column ratios and a builder closure for each column.
    ///
    /// # Example
    /// ```rust
    /// # use rustview::prelude::*;
    /// # let mut session = rustview::session::Session::new();
    /// # let mut ui = Ui::new(&mut session);
    /// ui.columns(&[1, 2], &[
    ///     &|col: &mut Ui| { col.write("Left"); },
    ///     &|col: &mut Ui| { col.write("Right"); },
    /// ]);
    /// ```
    pub fn columns(&mut self, ratios: &[u32], builders: &[&dyn Fn(&mut Ui)]) {
        assert_eq!(
            ratios.len(),
            builders.len(),
            "columns: ratios and builders must have the same length"
        );
        let widget_id = self.next_widget_id("columns");
        let mut node = widgets::render_columns(&widget_id, ratios);

        for (i, builder) in builders.iter().enumerate() {
            let prev = mem::take(&mut self.children);
            builder(self);
            let col_children = mem::replace(&mut self.children, prev);
            if let Some(col_div) = node.children.get_mut(i) {
                col_div.children = col_children;
            }
        }
        self.children.push(node);
    }

    /// Create a tabbed layout. Returns the currently active tab index.
    ///
    /// The closure receives a `&mut Ui` to populate the active tab's content.
    ///
    /// # Example
    /// ```rust
    /// # use rustview::prelude::*;
    /// # let mut session = rustview::session::Session::new();
    /// # let mut ui = Ui::new(&mut session);
    /// let active = ui.tabs(&["Tab A", "Tab B"], |inner| {
    ///     inner.write("Active tab content");
    /// });
    /// ```
    pub fn tabs(&mut self, labels: &[&str], f: impl FnOnce(&mut Ui)) -> usize {
        let widget_id = self.next_widget_id("tabs");
        let default = 0usize;
        let value = self
            .session
            .get_widget_value(&widget_id, serde_json::json!(default));
        let active = value
            .as_u64()
            .map(|v| v as usize)
            .unwrap_or(default)
            .min(labels.len().saturating_sub(1));

        let prev = mem::take(&mut self.children);
        f(self);
        let inner = mem::replace(&mut self.children, prev);

        let mut node = widgets::render_tabs(&widget_id, labels, active);
        // Structure: div.rustview-tabs > [div.tab-bar, div.tab-content]
        if let Some(panel) = node.children.get_mut(1) {
            panel.children = inner;
        }
        self.children.push(node);
        active
    }

    /// Render widgets inside a horizontal flex row.
    ///
    /// # Example
    /// ```rust
    /// # use rustview::prelude::*;
    /// # let mut session = rustview::session::Session::new();
    /// # let mut ui = Ui::new(&mut session);
    /// ui.row(|r| {
    ///     r.button("A");
    ///     r.button("B");
    ///     r.button("C");
    /// });
    /// ```
    pub fn row(&mut self, f: impl FnOnce(&mut Ui)) {
        let widget_id = self.next_widget_id("row");
        let prev = mem::take(&mut self.children);
        f(self);
        let inner = mem::replace(&mut self.children, prev);
        let mut node = widgets::render_row(&widget_id);
        node.children = inner;
        self.children.push(node);
    }

    /// Create an empty placeholder slot that can be filled later.
    ///
    /// Returns the widget ID that can be used to reference this slot.
    ///
    /// # Example
    /// ```rust
    /// # use rustview::prelude::*;
    /// # let mut session = rustview::session::Session::new();
    /// # let mut ui = Ui::new(&mut session);
    /// let slot_id = ui.empty();
    /// // The slot exists in the tree as an empty div
    /// ```
    pub fn empty(&mut self) -> String {
        let widget_id = self.next_widget_id("empty");
        self.children.push(widgets::render_empty(&widget_id));
        widget_id
    }

    /// Render a modal dialog overlay. Returns whether the modal is currently open.
    ///
    /// The `trigger_label` button opens the modal; clicking the close button
    /// closes it. The closure populates the modal body content.
    ///
    /// # Example
    /// ```rust
    /// # use rustview::prelude::*;
    /// # let mut session = rustview::session::Session::new();
    /// # let mut ui = Ui::new(&mut session);
    /// let open = ui.modal("Settings", "Open Settings", |inner| {
    ///     inner.write("Modal content here");
    /// });
    /// ```
    pub fn modal(
        &mut self,
        title: &str,
        trigger_label: &str,
        f: impl FnOnce(&mut Ui),
    ) -> bool {
        let widget_id = self.next_widget_id(title);
        let value = self
            .session
            .get_widget_value(&widget_id, serde_json::json!(false));
        let open = value.as_bool().unwrap_or(false);

        // Render trigger button — built directly so the inner <button> has
        // data-widget-type="modal_trigger" pointing at the modal's widget_id.
        let trigger_id = format!("{widget_id}-trigger");
        self.children.push(
            VNode::new(&trigger_id, "div")
                .with_attr("class", "rustview-widget rustview-button")
                .with_child(
                    VNode::new(format!("{trigger_id}-btn"), "button")
                        .with_text(trigger_label)
                        .with_attr("data-widget-id", &*widget_id)
                        .with_attr("data-widget-type", "modal_trigger"),
                ),
        );

        // Render modal overlay
        let prev = mem::take(&mut self.children);
        f(self);
        let inner = mem::replace(&mut self.children, prev);

        let mut node = widgets::render_modal(&widget_id, title, open);
        // Structure: div.overlay > div.dialog > [div.header, div.body]
        if let Some(dialog) = node.children.get_mut(0) {
            if let Some(body) = dialog.children.get_mut(1) {
                body.children = inner;
            }
        }
        self.children.push(node);
        open
    }

    /// Create a form container that batches widget updates.
    ///
    /// Widgets inside a form are rendered normally, but their values are only
    /// submitted when the form's submit button is clicked. The `submitted`
    /// return value is `true` on the re-run triggered by the submit click.
    ///
    /// # Example
    /// ```rust
    /// # use rustview::prelude::*;
    /// # let mut session = rustview::session::Session::new();
    /// # let mut ui = Ui::new(&mut session);
    /// let submitted = ui.form("user_form", |ui| {
    ///     ui.text_input("Name", "");
    ///     ui.number_input("Age", 30.0);
    /// });
    /// if submitted {
    ///     ui.success("Form submitted!");
    /// }
    /// ```
    pub fn form(&mut self, key: &str, f: impl FnOnce(&mut Ui)) -> bool {
        let widget_id = self.next_widget_id(key);
        let submitted = self
            .session
            .get_widget_value(&widget_id, serde_json::json!(false));
        let was_submitted = submitted.as_bool().unwrap_or(false);

        // Reset the submitted flag after reading it (one-shot)
        if was_submitted {
            self.session
                .set_widget_value(&widget_id, serde_json::json!(false));
        }

        // Build children inside form
        let old_children = mem::take(&mut self.children);
        f(self);
        let form_children = mem::replace(&mut self.children, old_children);

        let mut form_node = widgets::render_form(&widget_id);
        form_node.children = form_children;
        self.children.push(form_node);

        was_submitted
    }

    /// Add a submit button to the current form context.
    ///
    /// This should be called inside a `ui.form()` closure.
    ///
    /// # Example
    /// ```rust
    /// # use rustview::prelude::*;
    /// # let mut session = rustview::session::Session::new();
    /// # let mut ui = Ui::new(&mut session);
    /// ui.form("my_form", |ui| {
    ///     ui.text_input("Email", "");
    ///     ui.form_submit_button("Submit");
    /// });
    /// ```
    pub fn form_submit_button(&mut self, label: &str) {
        let widget_id = self.next_widget_id(label);
        self.children
            .push(widgets::render_form_submit_button(&widget_id, label));
    }

    // ---- Charts ----

    /// Display a line chart with (x, y) data points.
    ///
    /// Renders as an inline SVG with no external dependencies.
    ///
    /// # Example
    /// ```rust
    /// # use rustview::prelude::*;
    /// # let mut session = rustview::session::Session::new();
    /// # let mut ui = Ui::new(&mut session);
    /// let data: Vec<(f64, f64)> = (0..10).map(|i| (i as f64, (i * i) as f64)).collect();
    /// ui.line_chart("f(x) = x²", &data);
    /// ```
    pub fn line_chart(&mut self, title: &str, data: &[(f64, f64)]) {
        let widget_id = self.next_widget_id(title);
        self.children
            .push(widgets::render_line_chart(&widget_id, title, data));
    }

    /// Display a bar chart with labeled values.
    ///
    /// Renders as an inline SVG with no external dependencies.
    ///
    /// # Example
    /// ```rust
    /// # use rustview::prelude::*;
    /// # let mut session = rustview::session::Session::new();
    /// # let mut ui = Ui::new(&mut session);
    /// ui.bar_chart("Sales", &[("Q1", 100.0), ("Q2", 150.0), ("Q3", 120.0)]);
    /// ```
    pub fn bar_chart(&mut self, title: &str, data: &[(&str, f64)]) {
        let widget_id = self.next_widget_id(title);
        self.children
            .push(widgets::render_bar_chart(&widget_id, title, data));
    }

    /// Display a scatter chart with (x, y) data points.
    ///
    /// Renders as an inline SVG with no external dependencies.
    ///
    /// # Example
    /// ```rust
    /// # use rustview::prelude::*;
    /// # let mut session = rustview::session::Session::new();
    /// # let mut ui = Ui::new(&mut session);
    /// let data = vec![(1.0, 2.0), (3.0, 4.0), (5.0, 1.0)];
    /// ui.scatter_chart("Scatter", &data);
    /// ```
    pub fn scatter_chart(&mut self, title: &str, data: &[(f64, f64)]) {
        let widget_id = self.next_widget_id(title);
        self.children
            .push(widgets::render_scatter_chart(&widget_id, title, data));
    }

    /// Display a histogram from raw data values.
    ///
    /// Bins the data into `bins` equal-width buckets and renders as an inline SVG.
    ///
    /// # Example
    /// ```rust
    /// # use rustview::prelude::*;
    /// # let mut session = rustview::session::Session::new();
    /// # let mut ui = Ui::new(&mut session);
    /// let values = vec![1.0, 2.0, 2.5, 3.0, 3.5, 4.0, 4.5, 5.0];
    /// ui.histogram("Distribution", &values, 5);
    /// ```
    pub fn histogram(&mut self, title: &str, data: &[f64], bins: u32) {
        let widget_id = self.next_widget_id(title);
        self.children
            .push(widgets::render_histogram(&widget_id, title, data, bins));
    }

    /// Build the root VNode tree from all widgets added during this re-run.
    pub fn build_tree(self) -> VNode {
        let mut root = VNode::new("rustview-root", "div");
        root.attrs
            .insert("class".to_string(), "rustview-app".to_string());
        root.children = self.children;
        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::session::Session;

    #[test]
    fn test_text_input_default() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        let name = ui.text_input("Name", "World");
        assert_eq!(name, "World");
    }

    #[test]
    fn test_text_input_with_state() {
        let mut session = Session::new();
        // Simulate prior state
        {
            let mut ui = Ui::new(&mut session);
            let _widget_id = ui.next_widget_id("Name");
            // Set state for this widget
            session.set_widget_value(&_widget_id, serde_json::json!("Alice"));
        }
        // Now read from state
        let mut ui = Ui::new(&mut session);
        let name = ui.text_input("Name", "World");
        assert_eq!(name, "Alice");
    }

    #[test]
    fn test_int_slider_default() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        let val = ui.int_slider("Count", 1..=10);
        assert_eq!(val, 1); // min value
    }

    #[test]
    fn test_int_slider_clamped() {
        let mut session = Session::new();
        {
            let mut ui = Ui::new(&mut session);
            let widget_id = ui.next_widget_id("Count");
            session.set_widget_value(&widget_id, serde_json::json!(999));
        }
        let mut ui = Ui::new(&mut session);
        let val = ui.int_slider("Count", 1..=10);
        assert_eq!(val, 10); // clamped to max
    }

    #[test]
    fn test_checkbox_default() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        let checked = ui.checkbox("Enable", false);
        assert!(!checked);
    }

    #[test]
    fn test_button_returns_false_by_default() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        let clicked = ui.button("Submit");
        assert!(!clicked);
    }

    #[test]
    fn test_write_adds_to_tree() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        ui.write("Hello");
        ui.write(42);
        let tree = ui.build_tree();
        assert_eq!(tree.children.len(), 2);
    }

    #[test]
    fn test_build_tree_has_root() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        ui.write("test");
        let tree = ui.build_tree();
        assert_eq!(tree.id, "rustview-root");
        assert_eq!(tree.tag, "div");
    }

    #[test]
    fn test_state_default() {
        let mut session = Session::new();
        let ui = Ui::new(&mut session);
        let count = ui.get_state::<i64>("counter", 0);
        assert_eq!(count, 0);
    }

    #[test]
    fn test_state_set_and_get() {
        let mut session = Session::new();
        {
            let mut ui = Ui::new(&mut session);
            ui.set_state("counter", 5i64);
        }
        // Verify state was persisted
        let ui = Ui::new(&mut session);
        let count = ui.get_state::<i64>("counter", 0);
        assert_eq!(count, 5);
    }

    #[test]
    fn test_progress_renders() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        ui.progress(0.5);
        let tree = ui.build_tree();
        assert_eq!(tree.children.len(), 1);
    }

    #[test]
    fn test_error_renders() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        ui.error("Oops!");
        let tree = ui.build_tree();
        assert_eq!(tree.children.len(), 1);
    }

    #[test]
    fn test_markdown_renders() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        ui.markdown("# Title");
        let tree = ui.build_tree();
        assert_eq!(tree.children.len(), 1);
    }

    // ---- v0.2 Input Widget Tests ----

    #[test]
    fn test_number_input_default() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        let val = ui.number_input("Price", 9.99);
        assert!((val - 9.99).abs() < f64::EPSILON);
    }

    #[test]
    fn test_int_input_default() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        let val = ui.int_input("Count", 42);
        assert_eq!(val, 42);
    }

    #[test]
    fn test_slider_default() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        let val = ui.slider("Weight", 0.0..=1.0);
        assert!((val - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_toggle_default() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        let val = ui.toggle("Dark mode", false);
        assert!(!val);
    }

    #[test]
    fn test_radio_default() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        let val = ui.radio("Size", &["S", "M", "L"]);
        assert_eq!(val, "S");
    }

    #[test]
    fn test_select_default() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        let val = ui.select("Country", &["US", "UK", "DE"]);
        assert_eq!(val, "US");
    }

    #[test]
    fn test_multi_select_default() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        let val = ui.multi_select("Tags", &["rust", "python", "go"]);
        assert!(val.is_empty());
    }

    #[test]
    fn test_text_area_default() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        let val = ui.text_area("Bio", "Hello", 5);
        assert_eq!(val, "Hello");
    }

    #[test]
    fn test_color_picker_default() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        let val = ui.color_picker("Color");
        assert_eq!(val, "#000000");
    }

    #[test]
    fn test_download_button_renders() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        ui.download_button("Download", b"hello world", "test.txt");
        let tree = ui.build_tree();
        assert_eq!(tree.children.len(), 1);
    }

    // ---- v0.2 Output Widget Tests ----

    #[test]
    fn test_heading_renders() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        ui.heading("Title");
        let tree = ui.build_tree();
        assert_eq!(tree.children.len(), 1);
    }

    #[test]
    fn test_subheading_renders() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        ui.subheading("Subtitle");
        let tree = ui.build_tree();
        assert_eq!(tree.children.len(), 1);
    }

    #[test]
    fn test_caption_renders() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        ui.caption("small text");
        let tree = ui.build_tree();
        assert_eq!(tree.children.len(), 1);
    }

    #[test]
    fn test_code_renders() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        ui.code("fn main() {}", "rust");
        let tree = ui.build_tree();
        assert_eq!(tree.children.len(), 1);
    }

    #[test]
    fn test_json_renders() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        ui.json(&serde_json::json!({"key": "value"}));
        let tree = ui.build_tree();
        assert_eq!(tree.children.len(), 1);
    }

    #[test]
    fn test_table_renders() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        ui.table(&["Name", "Age"], &[vec!["Alice".into(), "30".into()]]);
        let tree = ui.build_tree();
        assert_eq!(tree.children.len(), 1);
    }

    #[test]
    fn test_spinner_renders() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        ui.spinner("Loading...");
        let tree = ui.build_tree();
        assert_eq!(tree.children.len(), 1);
    }

    #[test]
    fn test_metric_renders() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        ui.metric("Users", 1234, Some(5.2));
        let tree = ui.build_tree();
        assert_eq!(tree.children.len(), 1);
    }

    #[test]
    fn test_success_renders() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        ui.success("All good!");
        let tree = ui.build_tree();
        assert_eq!(tree.children.len(), 1);
    }

    #[test]
    fn test_warning_renders() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        ui.warning("Watch out!");
        let tree = ui.build_tree();
        assert_eq!(tree.children.len(), 1);
    }

    #[test]
    fn test_info_renders() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        ui.info("FYI");
        let tree = ui.build_tree();
        assert_eq!(tree.children.len(), 1);
    }

    #[test]
    fn test_divider_renders() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        ui.divider();
        let tree = ui.build_tree();
        assert_eq!(tree.children.len(), 1);
    }

    // ---- v0.2 Layout Tests ----

    #[test]
    fn test_container_layout() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        ui.container(|inner| {
            inner.write("Inside container");
            inner.write("Also inside");
        });
        let tree = ui.build_tree();
        assert_eq!(tree.children.len(), 1);
        let container = &tree.children[0];
        assert_eq!(container.children.len(), 2);
    }

    #[test]
    fn test_sidebar_layout() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        ui.sidebar(|sb| {
            sb.write("Sidebar content");
        });
        let tree = ui.build_tree();
        assert_eq!(tree.children.len(), 1);
    }

    #[test]
    fn test_expander_layout() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        ui.expander("Advanced", |inner| {
            inner.write("Hidden content");
        });
        let tree = ui.build_tree();
        assert_eq!(tree.children.len(), 1);
    }

    #[test]
    fn test_columns_layout() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        ui.columns(
            &[1, 2],
            &[
                &|col: &mut Ui| {
                    col.write("Left");
                },
                &|col: &mut Ui| {
                    col.write("Right");
                },
            ],
        );
        let tree = ui.build_tree();
        assert_eq!(tree.children.len(), 1);
        let cols = &tree.children[0];
        assert_eq!(cols.children.len(), 2);
        assert_eq!(cols.children[0].children.len(), 1);
        assert_eq!(cols.children[1].children.len(), 1);
    }

    #[test]
    fn test_tabs_layout() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        let active = ui.tabs(&["Tab A", "Tab B"], |inner| {
            inner.write("Tab content");
        });
        assert_eq!(active, 0);
        let tree = ui.build_tree();
        assert_eq!(tree.children.len(), 1);
    }

    #[test]
    fn test_date_picker_default() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        let date = ui.date_picker("Start date");
        assert_eq!(date, "");
        let tree = ui.build_tree();
        assert_eq!(tree.children.len(), 1);
    }

    #[test]
    fn test_date_picker_with_state() {
        let mut session = Session::new();
        session.set_widget_value("w-Start_date-1", serde_json::json!("2025-06-15"));
        let mut ui = Ui::new(&mut session);
        let date = ui.date_picker("Start date");
        assert_eq!(date, "2025-06-15");
    }

    #[test]
    fn test_file_upload_default() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        let data = ui.file_upload("Upload CSV");
        assert_eq!(data, "");
        let tree = ui.build_tree();
        assert_eq!(tree.children.len(), 1);
    }

    #[test]
    fn test_file_upload_with_state() {
        let mut session = Session::new();
        session.set_widget_value("w-Upload_file-1", serde_json::json!("data:text/csv;base64,YSxiLGM="));
        let mut ui = Ui::new(&mut session);
        let data = ui.file_upload("Upload file");
        assert_eq!(data, "data:text/csv;base64,YSxiLGM=");
    }

    #[test]
    fn test_image_renders() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        ui.image("https://example.com/pic.png", "Logo");
        let tree = ui.build_tree();
        assert_eq!(tree.children.len(), 1);
        assert!(tree.children[0]
            .attrs
            .get("class")
            .unwrap()
            .contains("rustview-image"));
    }

    #[test]
    fn test_audio_renders() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        ui.audio("https://example.com/clip.mp3", "mp3");
        let tree = ui.build_tree();
        assert_eq!(tree.children.len(), 1);
        assert!(tree.children[0]
            .attrs
            .get("class")
            .unwrap()
            .contains("rustview-audio"));
    }

    // ---- New layout/chart UI tests ----

    #[test]
    fn test_row_renders() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        ui.row(|r| {
            r.write("A");
            r.write("B");
        });
        let tree = ui.build_tree();
        assert_eq!(tree.children.len(), 1);
        let row = &tree.children[0];
        assert!(row.attrs.get("class").unwrap().contains("rustview-row"));
        assert_eq!(row.children.len(), 2);
    }

    #[test]
    fn test_empty_renders() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        let slot_id = ui.empty();
        assert!(!slot_id.is_empty());
        let tree = ui.build_tree();
        assert_eq!(tree.children.len(), 1);
        assert!(tree.children[0]
            .attrs
            .get("class")
            .unwrap()
            .contains("rustview-empty"));
    }

    #[test]
    fn test_modal_default_closed() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        let open = ui.modal("Settings", "Open", |inner| {
            inner.write("Body");
        });
        assert!(!open);
        let tree = ui.build_tree();
        // trigger button + modal overlay
        assert_eq!(tree.children.len(), 2);
    }

    #[test]
    fn test_modal_open_state() {
        let mut session = Session::new();
        session.set_widget_value("w-Settings-1", serde_json::json!(true));
        let mut ui = Ui::new(&mut session);
        let open = ui.modal("Settings", "Open", |inner| {
            inner.write("Body");
        });
        assert!(open);
    }

    #[test]
    fn test_line_chart_renders() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        let data = vec![(0.0, 0.0), (1.0, 1.0)];
        ui.line_chart("Test", &data);
        let tree = ui.build_tree();
        assert_eq!(tree.children.len(), 1);
        assert!(tree.children[0]
            .attrs
            .get("class")
            .unwrap()
            .contains("rustview-chart"));
    }

    #[test]
    fn test_bar_chart_renders() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        ui.bar_chart("Sales", &[("Q1", 100.0), ("Q2", 200.0)]);
        let tree = ui.build_tree();
        assert_eq!(tree.children.len(), 1);
    }

    #[test]
    fn test_scatter_chart_renders() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        ui.scatter_chart("Plot", &[(1.0, 2.0), (3.0, 4.0)]);
        let tree = ui.build_tree();
        assert_eq!(tree.children.len(), 1);
    }

    #[test]
    fn test_histogram_renders() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        ui.histogram("Dist", &[1.0, 2.0, 3.0, 4.0, 5.0], 3);
        let tree = ui.build_tree();
        assert_eq!(tree.children.len(), 1);
    }

    #[test]
    fn test_toast_renders() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        ui.toast("Saved!", "success");
        let tree = ui.build_tree();
        assert_eq!(tree.children.len(), 1);
        let toast = &tree.children[0];
        assert!(toast.attrs.get("class").unwrap().contains("rustview-toast"));
        assert_eq!(toast.attrs.get("data-widget-type").unwrap(), "toast");
    }

    #[test]
    fn test_with_key_overrides_id() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        ui.with_key("user_name");
        ui.write("Hello");
        let tree = ui.build_tree();
        assert_eq!(tree.children[0].id, "k-user_name");
    }

    #[test]
    fn test_with_key_consumed_after_one_use() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        ui.with_key("first");
        ui.write("A");
        ui.write("B");
        let tree = ui.build_tree();
        assert_eq!(tree.children[0].id, "k-first");
        assert!(tree.children[1].id.starts_with("w-"));
    }

    #[test]
    fn test_with_key_chaining() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        ui.with_key("threshold");
        let _name = ui.text_input("Name", "");
        let tree = ui.build_tree();
        assert_eq!(tree.children[0].id, "k-threshold");
    }

    #[test]
    fn test_image_upload_default() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        let data = ui.image_upload("Upload photo");
        assert_eq!(data, "");
    }

    #[test]
    fn test_image_upload_with_state() {
        let mut session = Session::new();
        {
            let mut ui = Ui::new(&mut session);
            let widget_id = ui.next_widget_id("Upload photo");
            session.set_widget_value(
                &widget_id,
                serde_json::json!("data:image/png;base64,iVBORw0KGgo="),
            );
        }
        let mut ui = Ui::new(&mut session);
        let data = ui.image_upload("Upload photo");
        assert_eq!(data, "data:image/png;base64,iVBORw0KGgo=");
    }

    #[test]
    fn test_image_upload_shows_preview() {
        let mut session = Session::new();
        {
            let mut ui = Ui::new(&mut session);
            let widget_id = ui.next_widget_id("Photo");
            session.set_widget_value(
                &widget_id,
                serde_json::json!("data:image/jpeg;base64,abc123"),
            );
        }
        let mut ui = Ui::new(&mut session);
        let _data = ui.image_upload("Photo");
        let tree = ui.build_tree();
        let upload_node = &tree.children[0];
        assert_eq!(upload_node.children.len(), 3); // label + input + preview
    }

    #[test]
    fn test_form_default_not_submitted() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        let submitted = ui.form("test_form", |ui| {
            ui.text_input("Name", "");
        });
        assert!(!submitted);
    }

    #[test]
    fn test_form_submitted() {
        let mut session = Session::new();
        // Simulate the form submission event
        {
            let mut ui = Ui::new(&mut session);
            let widget_id = ui.next_widget_id("test_form");
            session.set_widget_value(&widget_id, serde_json::json!(true));
        }
        // Now read the form
        let mut ui = Ui::new(&mut session);
        let submitted = ui.form("test_form", |ui| {
            ui.text_input("Name", "");
        });
        assert!(submitted);
    }

    #[test]
    fn test_form_submitted_resets() {
        let mut session = Session::new();
        // Set submission true
        {
            let mut ui = Ui::new(&mut session);
            let widget_id = ui.next_widget_id("test_form");
            session.set_widget_value(&widget_id, serde_json::json!(true));
        }
        // First run reads and resets
        {
            let mut ui = Ui::new(&mut session);
            let submitted = ui.form("test_form", |ui| {
                ui.text_input("Name", "");
            });
            assert!(submitted);
        }
        // Second run should not be submitted
        {
            let mut ui = Ui::new(&mut session);
            let submitted = ui.form("test_form", |ui| {
                ui.text_input("Name", "");
            });
            assert!(!submitted);
        }
    }

    #[test]
    fn test_form_contains_children() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        let _submitted = ui.form("form1", |ui| {
            ui.text_input("Email", "");
            ui.form_submit_button("Send");
        });
        let tree = ui.build_tree();
        assert_eq!(tree.children.len(), 1);
        let form_node = &tree.children[0];
        assert!(form_node.attrs.get("class").unwrap().contains("rustview-form"));
        assert_eq!(form_node.children.len(), 2); // text_input + submit_button
    }

    #[test]
    fn test_link_renders() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        ui.link("Click", "https://example.com");
        let tree = ui.build_tree();
        assert_eq!(tree.children.len(), 1);
        assert!(tree.children[0].attrs.get("class").unwrap().contains("rustview-link"));
    }

    #[test]
    fn test_video_renders() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        ui.video("test.mp4", "mp4");
        let tree = ui.build_tree();
        assert_eq!(tree.children.len(), 1);
        assert!(tree.children[0].attrs.get("class").unwrap().contains("rustview-video"));
    }

    #[test]
    fn test_badge_renders() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        ui.badge("Active", "green");
        let tree = ui.build_tree();
        assert_eq!(tree.children.len(), 1);
        assert!(tree.children[0].attrs.get("class").unwrap().contains("rustview-badge"));
    }

    #[test]
    fn test_selectbox_is_select_alias() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        let val = ui.selectbox("Pick", &["X", "Y"]);
        assert_eq!(val, "X");
        let tree = ui.build_tree();
        assert!(tree.children[0].attrs.get("class").unwrap().contains("rustview-select"));
    }

    #[test]
    fn test_multiple_toasts_render() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        ui.toast("Message 1", "success");
        ui.toast("Message 2", "error");
        ui.toast("Message 3", "info");
        let tree = ui.build_tree();
        assert_eq!(tree.children.len(), 3);
        assert!(tree.children[0].attrs.get("class").unwrap().contains("rustview-toast"));
        assert!(tree.children[1].attrs.get("class").unwrap().contains("rustview-toast"));
        assert!(tree.children[2].attrs.get("class").unwrap().contains("rustview-toast"));
    }

    #[test]
    fn test_dataframe_renders() {
        let mut session = Session::new();
        let mut ui = Ui::new(&mut session);
        ui.dataframe(
            &[("Name", "str"), ("Age", "i64")],
            &[
                vec!["Alice".into(), "30".into()],
                vec!["Bob".into(), "25".into()],
            ],
            Some("Users"),
        );
        let tree = ui.build_tree();
        assert_eq!(tree.children.len(), 1);
        assert!(tree.children[0]
            .attrs
            .get("class")
            .unwrap()
            .contains("rustview-dataframe"));
    }
}
