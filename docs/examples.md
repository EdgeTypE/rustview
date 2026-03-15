# Examples

RustView ships with several example apps in the `examples/` directory.

---

## Running Examples

```bash
cargo run --example hello
cargo run --example counter
cargo run --example dashboard
cargo run --example showcase
```

---

## hello.rs — Minimal App

The smallest possible RustView app:

```rust
use rustview::prelude::*;

#[rustview::app]
fn app(ui: &mut Ui) {
    let name = ui.text_input("Name", "World");
    ui.write(format!("Hello, {name}!"));
}
```

**Demonstrates:** Text input, text output, `#[app]` macro.

---

## counter.rs — Session State

A counter app showing session state management:

```rust
use rustview::prelude::*;

fn app(ui: &mut Ui) {
    ui.heading("Counter");
    let count: i64 = ui.get_state("count", 0);

    if ui.button("Increment") {
        ui.set_state("count", count + 1);
    }
    if ui.button("Reset") {
        ui.set_state("count", 0_i64);
    }

    ui.metric("Count", count, None);
}

fn main() {
    rustview::run(app);
}
```

**Demonstrates:** `get_state`/`set_state`, buttons, metric display.

---

## dashboard.rs — Multi-Column Layout

A dashboard with sidebar, columns, metrics, and charts:

```rust
use rustview::prelude::*;

fn app(ui: &mut Ui) {
    ui.sidebar(|sb| {
        sb.heading("Filters");
        let _region = sb.select("Region", &["All", "US", "EU", "Asia"]);
    });

    ui.heading("Dashboard");

    ui.columns(&[1, 1, 1], &[
        &|c: &mut Ui| c.metric("Users", 12_456, Some(5.2)),
        &|c: &mut Ui| c.metric("Revenue", "$98.7K", Some(-2.1)),
        &|c: &mut Ui| c.metric("Uptime", "99.99%", None),
    ]);

    ui.line_chart("Weekly Users", &[
        (0.0, 100.0), (1.0, 120.0), (2.0, 115.0),
        (3.0, 130.0), (4.0, 145.0),
    ]);
}

fn main() {
    rustview::run(app);
}
```

**Demonstrates:** Sidebar, columns, metrics, line chart.

---

## showcase.rs — All Widgets

The complete showcase includes every RustView widget organized into 13 sections:

1. **Sidebar** — sidebar, toggle, radio, select, badge, link
2. **Input Widgets** — text_input, text_area, int_input, number_input, slider, int_slider, checkbox, toggle, radio, select, selectbox, multi_select, color_picker, date_picker
3. **Display Widgets** — write, markdown, heading, subheading, caption, code, json, table, dataframe
4. **Metrics** — metric with positive/negative/no delta
5. **Progress & Status** — progress, spinner, badge
6. **Alerts** — success, info, warning, error
7. **Charts** — line_chart, bar_chart, scatter_chart, histogram
8. **Layout Primitives** — container, expander, tabs, row, columns
9. **Modal Dialog** — modal
10. **Form** — form, form_submit_button
11. **Media** — image, file_upload, image_upload
12. **Session State** — get_state/set_state with counter
13. **Widget Keys** — with_key for stable IDs

Run it:

```bash
cargo run --example showcase
```

---

## Writing Your Own Example

```rust
use rustview::prelude::*;

fn app(ui: &mut Ui) {
    ui.heading("My App");

    // Input section
    let name = ui.text_input("Name", "");
    let age = ui.int_input("Age", 25);

    // Display section
    if !name.is_empty() {
        ui.success(format!("{name} is {age} years old."));
    }

    // Table
    ui.table(
        &["Field", "Value"],
        &[
            vec!["Name".into(), name],
            vec!["Age".into(), age.to_string()],
        ],
    );

    // Dataframe
    ui.dataframe(
        &[("Name", "str"), ("Age", "i64")],
        &[vec!["Alice".into(), "30".into()]],
        Some("Sample Data"),
    );
}

fn main() {
    rustview::run(app);
}
```

Save this as `examples/my_app.rs` and run:

```bash
cargo run --example my_app
```
