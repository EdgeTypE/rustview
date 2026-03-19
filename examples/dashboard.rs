/// Example: RustView Dashboard — showcases v0.2 widgets and layout.
use rustview::prelude::*;

fn dashboard(ui: &mut Ui) {
    ui.heading("RustView Dashboard");
    ui.caption("A showcase of v0.2 widgets");
    ui.divider();

    // Metrics row
    ui.columns(
        &[1, 1, 1],
        &[
            &|col: &mut Ui| {
                col.metric("Users", 1_234, Some(12.5));
            },
            &|col: &mut Ui| {
                col.metric("Revenue", "$45.2K", Some(-3.1));
            },
            &|col: &mut Ui| {
                col.metric("Uptime", "99.9%", None);
            },
        ],
    );

    ui.divider();

    // Input widgets
    ui.subheading("Configuration");

    let name = ui.text_input("Project name", "My App");
    let priority = ui.select("Priority", &["Low", "Medium", "High"]);
    let dark_mode = ui.toggle("Dark mode", true);
    let threshold = ui.slider("Threshold", 0.0..=1.0, 0.5);
    let notes = ui.text_area("Notes", "", 3);
    let color = ui.color_picker("Accent color");

    // Show selections
    ui.expander("Current Settings", |inner| {
        inner.json(&serde_json::json!({
            "name": name,
            "priority": priority,
            "dark_mode": dark_mode,
            "threshold": threshold,
            "notes": notes,
            "accent_color": color,
        }));
    });

    // Data table
    ui.subheading("Recent Activity");
    ui.table(
        &["Time", "Event", "Status"],
        &[
            vec!["10:30".into(), "Deploy v1.2".into(), "Success".into()],
            vec!["10:15".into(), "Build #42".into(), "Running".into()],
            vec!["09:45".into(), "Test Suite".into(), "Passed".into()],
        ],
    );

    // Alerts
    ui.success("All systems operational");
    ui.info("Next deployment scheduled for 14:00 UTC");
}

fn main() {
    rustview::run(dashboard);
}
