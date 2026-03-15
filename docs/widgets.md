# Widget Reference

RustView provides 56+ widgets organized into input widgets, output widgets, and special widgets.

---

## Input Widgets

Input widgets return their current value. Every re-render, the value is read from session state.

### `text_input`

Single-line text input.

```rust
let name = ui.text_input("Label", "default value");
// name: String
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `label` | `&str` | Label displayed above the input |
| `default` | `&str` | Default value shown initially |
| **Returns** | `String` | Current text value |

### `text_area`

Multi-line text input.

```rust
let bio = ui.text_area("Bio", "Tell us about yourself...", 5);
// bio: String
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `label` | `&str` | Label displayed above the textarea |
| `default` | `&str` | Default text |
| `rows` | `u32` | Number of visible rows |
| **Returns** | `String` | Current text value |

### `number_input`

Float number input with step controls.

```rust
let weight = ui.number_input("Weight (kg)", 70.5);
// weight: f64
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `label` | `&str` | Label |
| `default` | `f64` | Default value |
| **Returns** | `f64` | Current numeric value |

### `int_input`

Integer number input.

```rust
let age = ui.int_input("Age", 25);
// age: i64
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `label` | `&str` | Label |
| `default` | `i64` | Default value |
| **Returns** | `i64` | Current integer value |

### `slider`

Float slider widget.

```rust
let ratio = ui.slider("Ratio", 0.0..=1.0);
// ratio: f64
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `label` | `&str` | Label with current value shown |
| `range` | `RangeInclusive<f64>` | Min..=Max range |
| **Returns** | `f64` | Current slider value |

### `int_slider`

Integer slider widget.

```rust
let score = ui.int_slider("Score", 0..=100);
// score: i64
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `label` | `&str` | Label with current value shown |
| `range` | `RangeInclusive<i64>` | Min..=Max range |
| **Returns** | `i64` | Current slider value |

### `checkbox`

Boolean checkbox.

```rust
let agreed = ui.checkbox("I agree", false);
// agreed: bool
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `label` | `&str` | Text next to the checkbox |
| `default` | `bool` | Initial state |
| **Returns** | `bool` | Current checked state |

### `toggle`

Boolean toggle switch (visual alternative to checkbox).

```rust
let notifications = ui.toggle("Notifications", true);
// notifications: bool
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `label` | `&str` | Text next to the toggle |
| `default` | `bool` | Initial state |
| **Returns** | `bool` | Current toggle state |

### `button`

Click button. Returns `true` on the re-render triggered by a click.

```rust
if ui.button("Submit") {
    // Handle click
    ui.success("Submitted!");
}
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `label` | `&str` | Button text |
| **Returns** | `bool` | `true` if clicked this render cycle |

### `radio`

Radio button group — single selection from a list.

```rust
let choice = ui.radio("Pick one", &["Alpha", "Beta", "Gamma"]);
// choice: String — e.g., "Alpha"
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `label` | `&str` | Group label |
| `options` | `&[&str]` | Available options |
| **Returns** | `String` | Currently selected option |

### `select`

Dropdown select widget.

```rust
let category = ui.select("Category", &["Tech", "Science", "Art"]);
// category: String
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `label` | `&str` | Label |
| `options` | `&[&str]` | Available options |
| **Returns** | `String` | Currently selected option |

### `selectbox`

Alias for `select` — provided for Streamlit compatibility.

```rust
let item = ui.selectbox("Item", &["A", "B", "C"]);
```

### `multi_select`

Multi-selection listbox.

```rust
let tags = ui.multi_select("Tags", &["rust", "web", "ui", "sse"]);
// tags: Vec<String>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `label` | `&str` | Label |
| `options` | `&[&str]` | Available options |
| **Returns** | `Vec<String>` | Currently selected options |

### `color_picker`

Color picker input returning a hex string.

```rust
let color = ui.color_picker("Accent color");
// color: String — e.g., "#ff4b4b"
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `label` | `&str` | Label |
| **Returns** | `String` | Hex color string |

### `date_picker`

Date picker input.

```rust
let date = ui.date_picker("Start date");
// date: String — e.g., "2025-01-15"
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `label` | `&str` | Label |
| **Returns** | `String` | Date string (ISO 8601) |

### `file_upload`

General file upload. Returns a base64-encoded data URI.

```rust
let file_data = ui.file_upload("Upload any file");
// file_data: String — data URI or empty string
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `label` | `&str` | Button label |
| **Returns** | `String` | Base64 data URI of uploaded file |

### `image_upload`

Image-only file upload with inline preview.

```rust
let img_data = ui.image_upload("Upload an image");
// img_data: String — data URI or empty string
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `label` | `&str` | Button label |
| **Returns** | `String` | Base64 data URI of uploaded image |

---

## Output Widgets

Output widgets display content but do not return a value.

### `write`

Universal display — renders any `Display` type as a paragraph.

```rust
ui.write("Plain text");
ui.write(42);
ui.write(format!("Result: {:.2}", 3.14));
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `value` | `impl Display` | Any type that implements `Display` |

### `heading`

Top-level heading (rendered as `<h1>`).

```rust
ui.heading("Dashboard");
```

### `subheading`

Secondary heading (rendered as `<h2>`).

```rust
ui.subheading("Charts Section");
```

### `caption`

Small text (useful for labels, footnotes).

```rust
ui.caption("Last updated: 2025-01-15");
```

### `markdown`

Render markdown text. Supports bold, italic, code, and links.

```rust
ui.markdown("**Bold**, *italic*, `code`, [link](https://example.com)");
```

### `code`

Display a code block with a language hint.

```rust
ui.code("fn main() { println!(\"Hello!\"); }", "rust");
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `source` | `&str` | Code text |
| `language` | `&str` | Language name (for syntax highlighting) |

### `json`

Display a JSON value in a formatted code block.

```rust
ui.json(&serde_json::json!({
    "name": "RustView",
    "version": "0.2.0"
}));
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `value` | `&impl Serialize` | Any serializable value |

### `table`

Display a simple table with headers and rows.

```rust
ui.table(
    &["Name", "Age"],
    &[
        vec!["Alice".into(), "30".into()],
        vec!["Bob".into(), "25".into()],
    ],
);
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `headers` | `&[&str]` | Column headers |
| `rows` | `&[Vec<String>]` | Row data |

### `dataframe`

Display an enhanced data table with row numbers, column type annotations,
an optional title, and scrollable overflow for large datasets.

```rust
ui.dataframe(
    &[("Name", "str"), ("Age", "i64"), ("Score", "f64")],
    &[
        vec!["Alice".into(), "30".into(), "95.5".into()],
        vec!["Bob".into(), "25".into(), "82.3".into()],
    ],
    Some("User Dataset"),
);
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `columns` | `&[(&str, &str)]` | Column `(name, type)` pairs |
| `rows` | `&[Vec<String>]` | Row data |
| `title` | `Option<&str>` | Optional title above the table |

**Features:**
- Row index column (0-based)
- Column type annotations displayed below column names
- Numeric columns (`i64`, `f64`, `u64`, `i32`, `f32`, `u32`) are right-aligned
- Sticky headers during vertical scroll
- Horizontal scrolling for wide datasets
- Shape indicator showing `N rows × M columns`

### `image`

Display an image from a URL or data URI.

```rust
ui.image("https://example.com/photo.png", "A photo");
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `src` | `&str` | Image URL or base64 data URI |
| `caption` | `&str` | Alt text / caption |

### `audio`

Display an audio player.

```rust
ui.audio("https://example.com/song.mp3", "mp3");
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `src` | `&str` | Audio URL or base64 data URI |
| `format` | `&str` | Format (`"mp3"`, `"wav"`, `"ogg"`) |

### `video`

Display a video player.

```rust
ui.video("https://example.com/clip.mp4", "mp4");
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `src` | `&str` | Video URL or base64 data URI |
| `format` | `&str` | Format (`"mp4"`, `"webm"`, `"ogg"`) |

### `download_button`

Button that triggers a file download.

```rust
ui.download_button("Download CSV", b"name,age\nAlice,30", "data.csv");
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `label` | `&str` | Button label |
| `data` | `&[u8]` | File contents |
| `filename` | `&str` | Suggested filename |

### `link`

Display a clickable hyperlink.

```rust
ui.link("Documentation", "https://github.com/EdgeTypE/rustview");
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `text` | `&str` | Link text |
| `url` | `&str` | Target URL |

### `metric`

Display a KPI metric tile with label, value, and optional delta.

```rust
ui.metric("Revenue", "$98.7K", Some(-2.1));
ui.metric("Uptime", "99.99%", None);
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `label` | `&str` | Metric name |
| `value` | `impl Display` | Current value |
| `delta` | `Option<f64>` | Change indicator (green if positive, red if negative) |

### `progress`

Display a progress bar.

```rust
ui.progress(0.73); // 73%
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `value` | `f64` | Progress from 0.0 to 1.0 |

### `spinner`

Display a loading spinner with a label.

```rust
ui.spinner("Loading data...");
```

### `badge`

Display a colored badge / label.

```rust
ui.badge("stable", "green");
ui.badge("WIP", "yellow");
ui.badge("archived", "gray");
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `text` | `&str` | Badge text |
| `color` | `&str` | Color name: `red`, `green`, `blue`, `yellow`, `gray`, `purple`, `orange`, or any CSS color |

### `divider`

Display a horizontal rule separator.

```rust
ui.divider();
```

---

## Alert Widgets

### `success`

Green success alert.

```rust
ui.success("Operation completed!");
```

### `info`

Blue informational alert.

```rust
ui.info("RustView supports 56+ widgets.");
```

### `warning`

Yellow warning alert.

```rust
ui.warning("API may change in preview releases.");
```

### `error`

Red error alert.

```rust
ui.error("Connection lost.");
```

### `toast`

Temporary notification that auto-dismisses after 5 seconds.

```rust
ui.toast("File saved!", "success");
ui.toast("Warning!", "warning");
ui.toast("Error occurred", "error");
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `message` | `&str` | Notification text |
| `level` | `&str` | `"success"`, `"info"`, `"warning"`, or `"error"` |

---

## Widget Keys

Use `with_key()` to assign stable identifiers to widgets. This ensures consistent state when widgets are added or removed conditionally.

```rust
ui.with_key("username").text_input("Name", "World");
```

Without keys, widgets are identified by their render order (auto-incrementing counter). If you conditionally show/hide widgets, this can cause state to shift between widgets. Keys solve this by giving each widget a fixed identity.

```rust
let show_extra = ui.checkbox("Show extra field", false);

// Always use a key for conditional widgets
ui.with_key("name").text_input("Name", "");

if show_extra {
    ui.with_key("email").text_input("Email", "");
}

// Without keys, hiding "extra" would cause state confusion
```
