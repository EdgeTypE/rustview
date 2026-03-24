/// Comprehensive RustView Showcase — demonstrates ALL available widgets and layouts.
///
/// Run with: cargo run --example showcase
use rustview::prelude::*;
use rustview::server::Layout;

fn showcase(ui: &mut Ui) {
    // ── Header ──────────────────────────────────────────────────────
    ui.heading("🦀 RustView Widget Showcase");
    ui.caption("Every widget and layout primitive in a single app");
    ui.divider();

    // ── Sidebar ─────────────────────────────────────────────────────
    ui.sidebar(|sb| {
        sb.heading("⚙ Sidebar");
        sb.write("This panel stays fixed on the left.");
        sb.divider();

        sb.subheading("Quick Filters");
        let _dark = sb.toggle("Dark mode", true);
        let _lang = sb.radio("Language", &["Rust", "Python", "Go"]);
        let _priority = sb.select("Priority", &["All", "High", "Medium", "Low"]);

        sb.divider();
        sb.badge("v0.2", "green");
        sb.badge("beta", "yellow");
        sb.link("Documentation", "https://github.com/EdgeTypE/rustview");
    });

    // ── Section 1: Input Widgets ────────────────────────────────────
    ui.subheading("📥 Input Widgets");

    ui.columns(
        &[1, 1],
        &[
            &|col: &mut Ui| {
                col.caption("Text Inputs");
                let name = col.text_input("Name", "World");
                let bio = col.text_area("Bio", "Tell us about yourself...", 3);
                col.write(format!("Hello, {}! Bio: {}", name, bio));
            },
            &|col: &mut Ui| {
                col.caption("Numeric Inputs");
                let age = col.int_input("Age", 25);
                let weight = col.number_input("Weight (kg)", 70.5);
                let score = col.int_slider("Score", 0..=100, 50);
                let ratio = col.slider("Ratio", 0.0..=1.0, 0.5);
                col.write(format!(
                    "Age: {}, Weight: {:.1}, Score: {}, Ratio: {:.2}",
                    age, weight, score, ratio
                ));
            },
        ],
    );

    ui.columns(
        &[1, 1, 1],
        &[
            &|col: &mut Ui| {
                col.caption("Boolean Inputs");
                let agree = col.checkbox("I agree", false);
                let notify = col.toggle("Notifications", true);
                col.write(format!("Agree: {}, Notify: {}", agree, notify));
            },
            &|col: &mut Ui| {
                col.caption("Selection");
                let choice = col.radio("Pick one", &["Alpha", "Beta", "Gamma"]);
                let category = col.select("Category", &["Tech", "Science", "Art"]);
                let _alias = col.selectbox("Selectbox alias", &["A", "B", "C"]);
                col.write(format!("{} / {}", choice, category));
            },
            &|col: &mut Ui| {
                col.caption("Multi & Special");
                let tags = col.multi_select("Tags", &["rust", "web", "ui", "sse"]);
                let color = col.color_picker("Accent color");
                let date = col.date_picker("Start date");
                col.write(format!("Tags: {:?}", tags));
                col.write(format!("Color: {}, Date: {}", color, date));
            },
        ],
    );

    ui.divider();

    // ── Section 2: Buttons & Actions ────────────────────────────────
    ui.subheading("🔘 Buttons & Actions");

    ui.row(|r| {
        if r.button("Click me") {
            r.toast("Button clicked!", "success");
        }
        if r.button("Show warning") {
            r.toast("This is a warning toast", "warning");
        }
        r.download_button("Download sample", b"Hello from RustView!", "sample.txt");
    });

    ui.divider();

    // ── Section 3: Display Widgets ──────────────────────────────────
    ui.subheading("📤 Display Widgets");

    ui.columns(
        &[1, 1],
        &[
            &|col: &mut Ui| {
                col.caption("Text Display");
                col.write("Plain text via write()");
                col.markdown("**Bold**, *italic*, `code`, and [links](https://rust-lang.org)");
                col.heading("Heading (h1)");
                col.subheading("Subheading (h2)");
                col.caption("Caption (small)");
            },
            &|col: &mut Ui| {
                col.caption("Code & Data");
                col.code(
                    "fn main() {\n    println!(\"Hello, RustView!\");\n}",
                    "rust",
                );
                col.json(&serde_json::json!({
                    "name": "RustView",
                    "version": "0.2.0",
                    "widgets": 55
                }));
            },
        ],
    );

    // Table
    ui.table(
        &["Feature", "Status", "Version"],
        &[
            vec!["Widgets".into(), "Complete".into(), "v0.2".into()],
            vec!["Layouts".into(), "Complete".into(), "v0.2".into()],
            vec!["Charts".into(), "Complete".into(), "v0.2".into()],
            vec!["Interface".into(), "Complete".into(), "v0.3".into()],
            vec!["Theming".into(), "Complete".into(), "v0.2".into()],
        ],
    );

    // Dataframe — enhanced table with row numbers, column types, title, shape
    ui.dataframe(
        &[("Name", "str"), ("Age", "i64"), ("Score", "f64"), ("Active", "bool")],
        &[
            vec!["Alice".into(), "30".into(), "95.5".into(), "true".into()],
            vec!["Bob".into(), "25".into(), "82.3".into(), "false".into()],
            vec!["Carol".into(), "28".into(), "91.0".into(), "true".into()],
            vec!["Dave".into(), "35".into(), "78.8".into(), "true".into()],
            vec!["Eve".into(), "22".into(), "99.1".into(), "false".into()],
        ],
        Some("User Dataset"),
    );

    ui.divider();

    // ── Section 4: Metrics ──────────────────────────────────────────
    ui.subheading("📊 Metrics");

    ui.columns(
        &[1, 1, 1, 1],
        &[
            &|c: &mut Ui| c.metric("Users", 12_456, Some(5.2)),
            &|c: &mut Ui| c.metric("Revenue", "$98.7K", Some(-2.1)),
            &|c: &mut Ui| c.metric("Latency", "12ms", Some(-8.0)),
            &|c: &mut Ui| c.metric("Uptime", "99.99%", None),
        ],
    );

    ui.divider();

    // ── Section 5: Progress & Status ────────────────────────────────
    ui.subheading("⏳ Progress & Status");

    ui.progress(0.73);
    ui.spinner("Loading data...");

    ui.row(|r| {
        r.badge("stable", "green");
        r.badge("performance", "blue");
        r.badge("security", "red");
        r.badge("new", "purple");
        r.badge("WIP", "orange");
        r.badge("archived", "gray");
        r.badge("custom", "#e91e63");
    });

    ui.divider();

    // ── Section 6: Alerts ───────────────────────────────────────────
    ui.subheading("🔔 Alerts");

    ui.success("Operation completed successfully.");
    ui.info("RustView supports 55+ widgets out of the box.");
    ui.warning("This is a preview release — API may change.");
    ui.error("Connection to database lost (demo error).");

    ui.divider();

    // ── Section 7: Charts ───────────────────────────────────────────
    ui.subheading("📈 Charts (Inline SVG)");

    ui.columns(
        &[1, 1],
        &[
            &|col: &mut Ui| {
                col.line_chart(
                    "Temperature (°C)",
                    &[
                        (0.0, 15.0),
                        (1.0, 17.0),
                        (2.0, 16.5),
                        (3.0, 19.0),
                        (4.0, 22.0),
                        (5.0, 24.5),
                        (6.0, 23.0),
                        (7.0, 21.0),
                    ],
                );
            },
            &|col: &mut Ui| {
                col.bar_chart(
                    "Sales by Region",
                    &[
                        ("Europe", 42.0),
                        ("Asia", 38.0),
                        ("Americas", 31.0),
                        ("Africa", 15.0),
                        ("Oceania", 9.0),
                    ],
                );
            },
        ],
    );

    ui.columns(
        &[1, 1],
        &[
            &|col: &mut Ui| {
                col.scatter_chart(
                    "Height vs Weight",
                    &[
                        (160.0, 55.0),
                        (165.0, 62.0),
                        (170.0, 68.0),
                        (175.0, 75.0),
                        (180.0, 80.0),
                        (168.0, 70.0),
                        (172.0, 73.0),
                        (178.0, 78.0),
                        (163.0, 58.0),
                        (185.0, 85.0),
                    ],
                );
            },
            &|col: &mut Ui| {
                col.histogram(
                    "Response Time Distribution",
                    &[
                        12.0, 15.0, 14.0, 18.0, 22.0, 19.0, 25.0, 30.0, 28.0, 16.0, 17.0, 20.0,
                        21.0, 23.0, 27.0, 13.0, 11.0, 24.0, 26.0, 29.0,
                    ],
                    5,
                );
            },
        ],
    );

    ui.divider();

    // ── Section 8: Layouts ──────────────────────────────────────────
    ui.subheading("🧱 Layout Primitives");

    // Container
    ui.container(|c| {
        c.caption("Container — a scoped vertical block");
        c.write("Content inside a container with optional border.");
    });

    // Expander
    ui.expander("Expander — click to expand", |inner| {
        inner.write("This content is hidden by default.");
        inner.code("let x = 42;\nprintln!(\"{}\", x);", "rust");
    });

    // Tabs
    let active_tab = ui.tabs(&["Overview", "Details", "Settings"], |tab_ui| {
        tab_ui.write("Tab content renders here based on the selected tab.");
    });
    ui.caption(&format!("Active tab index: {}", active_tab));

    // Row
    ui.row(|r| {
        r.write("Item A");
        r.write("Item B");
        r.write("Item C");
        r.badge("inline", "blue");
    });

    // Empty placeholder
    let _slot_id = ui.empty();

    ui.divider();

    // ── Section 9: Modal ────────────────────────────────────────────
    ui.subheading("🪟 Modal Dialog");

    let _open = ui.modal("Confirmation", "Open Modal", |m| {
        m.write("Are you sure you want to proceed?");
        m.warning("This action cannot be undone.");
        if m.button("Confirm") {
            m.toast("Action confirmed!", "success");
        }
    });

    ui.divider();

    // ── Section 10: Form ────────────────────────────────────────────
    ui.subheading("📋 Form (Batch Submit)");

    let submitted = ui.form("contact_form", |f| {
        f.text_input("Full name", "");
        f.text_input("Email", "");
        f.text_area("Message", "", 4);
        f.checkbox("Subscribe to newsletter", false);
        f.form_submit_button("Send Message");
    });

    if submitted {
        ui.success("Form submitted successfully!");
    }

    ui.divider();

    // ── Section 11: Media ───────────────────────────────────────────
    ui.subheading("🎬 Media Widgets");

    ui.columns(
        &[1, 1],
        &[
            &|col: &mut Ui| {
                col.caption("Image");
                col.image(
                    "https://www.rust-lang.org/logos/rust-logo-512x512.png",
                    "Rust Logo",
                );
            },
            &|col: &mut Ui| {
                col.caption("File Uploads");
                let _file = col.file_upload("Upload any file");
                let _img = col.image_upload("Upload an image");
            },
        ],
    );

    ui.divider();

    // ── Section 12: Stateful Counter ────────────────────────────────
    ui.subheading("🔄 Session State");

    let count = ui.get_state::<i64>("showcase_counter", 0);
    ui.columns(
        &[1, 1, 1],
        &[
            &|col: &mut Ui| {
                if col.button("➕ Increment") {
                    let c = col.get_state::<i64>("showcase_counter", 0);
                    col.set_state("showcase_counter", c + 1);
                }
            },
            &|col: &mut Ui| {
                if col.button("➖ Decrement") {
                    let c = col.get_state::<i64>("showcase_counter", 0);
                    col.set_state("showcase_counter", c - 1);
                }
            },
            &|col: &mut Ui| {
                if col.button("🔄 Reset") {
                    col.set_state("showcase_counter", 0i64);
                }
            },
        ],
    );
    // Read counter AFTER buttons to show the latest value
    let count = ui.get_state::<i64>("showcase_counter", count);
    ui.metric("Counter", count, None);

    ui.divider();

    // ── Section 13: Widget Keys ─────────────────────────────────────
    ui.subheading("🔑 Widget Keys");
    ui.caption("with_key() ensures stable IDs across conditional re-renders");

    ui.with_key("stable-input");
    let _keyed = ui.text_input("Keyed input", "stable");

    ui.divider();

    // ── Footer ──────────────────────────────────────────────────────
    ui.caption("Built with RustView — A Streamlit/Gradio equivalent for pure Rust 🦀");
    ui.link("View source on GitHub", "https://github.com/EdgeTypE/rustview");
}

fn main() {
    let layout = Layout {
        max_width_percent: 80,
        padding: "3rem 1rem".into(),
    };

    let config = RustViewConfig {
        layout,
        open_browser: true,
        ..Default::default()
    };

    rustview::run_with_config(showcase, config);
}
