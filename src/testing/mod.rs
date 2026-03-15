//! Testing utilities for RustView applications.
//!
//! The `TestUi` struct provides a mock-like harness that lets you
//! unit-test your RustView app functions without starting the HTTP server.
//!
//! # Example
//! ```rust
//! use rustview::testing::TestUi;
//! use rustview::ui::Ui;
//!
//! fn counter_app(ui: &mut Ui) {
//!     let count = ui.get_state::<i64>("n", 0);
//!     if ui.button("Inc") {
//!         ui.set_state("n", count + 1);
//!     }
//!     ui.write(format!("Count: {}", ui.get_state::<i64>("n", 0)));
//! }
//!
//! let mut tui = TestUi::new();
//! tui.click_button("Inc");
//! tui.run(counter_app);
//! assert!(tui.contains_text("Count: 1"));
//! ```

use crate::session::Session;
use crate::ui::Ui;
use crate::vdom::VNode;
use serde::Serialize;

/// Maximum widget counter value to try when resolving label → widget ID.
const MAX_WIDGET_COUNTER: u64 = 100;

/// A test harness for RustView applications.
///
/// Allows simulating widget interactions and asserting on output
/// without a browser or HTTP server.
pub struct TestUi {
    /// The backing session for state persistence across runs.
    session: Session,
    /// Pre-configured widget inputs (label → value).
    /// These are resolved to widget IDs at run time.
    pending_inputs: Vec<(String, serde_json::Value)>,
    /// Button clicks to simulate (by label).
    pending_clicks: Vec<String>,
    /// Last rendered VNode tree (after `run()`).
    last_tree: Option<VNode>,
}

impl TestUi {
    /// Create a new test harness with an empty session.
    pub fn new() -> Self {
        TestUi {
            session: Session::new(),
            pending_inputs: Vec::new(),
            pending_clicks: Vec::new(),
            last_tree: None,
        }
    }

    /// Simulate a button click by label.
    ///
    /// The button with this label will return `true` on the next `run()`.
    pub fn click_button(&mut self, label: &str) {
        self.pending_clicks.push(label.to_string());
    }

    /// Simulate setting a widget input value by label.
    ///
    /// Works for text_input, int_slider, number_input, checkbox, etc.
    pub fn set_input<V: Serialize>(&mut self, label: &str, value: V) {
        let json_val = serde_json::to_value(value).expect("Value must be serializable");
        self.pending_inputs.push((label.to_string(), json_val));
    }

    /// Sanitize a label to match the widget ID format used by `Ui::next_widget_id`.
    fn sanitize_label(label: &str) -> String {
        label
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '_' })
            .collect()
    }

    /// Run the app function once, collecting all output into the VNode tree.
    ///
    /// All pending button clicks and input values are applied before the run.
    /// After the run, query methods like `text_content()` reflect the output.
    pub fn run(&mut self, app_fn: impl Fn(&mut Ui)) {
        // Apply pending clicks: set matching widget IDs to true.
        // Widget IDs follow the pattern: w-{sanitized_label}-{counter}
        // We set all plausible counter values (1..=100) since we don't know
        // the exact counter value without running the app first.
        for label in &self.pending_clicks {
            let sanitized = Self::sanitize_label(label);
            for counter in 1..=MAX_WIDGET_COUNTER {
                let widget_id = format!("w-{sanitized}-{counter}");
                self.session
                    .set_widget_value(&widget_id, serde_json::json!(true));
            }
        }

        // Apply pending inputs
        for (label, value) in &self.pending_inputs {
            let sanitized = Self::sanitize_label(label);
            for counter in 1..=MAX_WIDGET_COUNTER {
                let widget_id = format!("w-{sanitized}-{counter}");
                self.session.set_widget_value(&widget_id, value.clone());
            }
        }

        // Run the app function
        let mut ui = Ui::new(&mut self.session);
        app_fn(&mut ui);
        let tree = ui.build_tree();
        self.last_tree = Some(tree);

        // Clear pending actions after run
        self.pending_clicks.clear();
        self.pending_inputs.clear();

        // Reset button states after run (buttons are one-shot)
        if let Some(ref tree) = self.last_tree {
            Self::reset_buttons(tree, &mut self.session);
        }
    }

    /// Reset all button widget values to false after a run.
    fn reset_buttons(node: &VNode, session: &mut Session) {
        if let Some(wtype) = node.attrs.get("data-widget-type") {
            if wtype == "button" {
                if let Some(wid) = node.attrs.get("data-widget-id") {
                    session.set_widget_value(wid, serde_json::json!(false));
                }
            }
        }
        for child in &node.children {
            Self::reset_buttons(child, session);
        }
    }

    /// Get the concatenated text content of all output widgets from the last run.
    ///
    /// Extracts text from `write`, `heading`, `subheading`, `caption`, `error`,
    /// `success`, `warning`, `info`, and `metric` widgets.
    pub fn text_content(&self) -> String {
        let Some(ref tree) = self.last_tree else {
            return String::new();
        };
        let mut texts = Vec::new();
        Self::collect_texts(tree, &mut texts);
        texts.join("\n")
    }

    /// Recursively collect text content from VNode tree.
    fn collect_texts(node: &VNode, out: &mut Vec<String>) {
        if let Some(ref text) = node.text {
            if !text.is_empty() {
                out.push(text.clone());
            }
        }
        for child in &node.children {
            Self::collect_texts(child, out);
        }
    }

    /// Get the text content of widgets matching a specific CSS class substring.
    ///
    /// For example, `find_widget_text("rustview-write")` returns text from write widgets.
    pub fn find_widget_text(&self, class_substr: &str) -> Vec<String> {
        let Some(ref tree) = self.last_tree else {
            return Vec::new();
        };
        let mut results = Vec::new();
        Self::collect_widget_texts(tree, class_substr, &mut results);
        results
    }

    fn collect_widget_texts(node: &VNode, class_substr: &str, out: &mut Vec<String>) {
        let matches_class = node
            .attrs
            .get("class")
            .is_some_and(|c| c.contains(class_substr));
        if matches_class {
            let mut texts = Vec::new();
            Self::collect_texts(node, &mut texts);
            if !texts.is_empty() {
                out.push(texts.join(" "));
            }
        } else {
            for child in &node.children {
                Self::collect_widget_texts(child, class_substr, out);
            }
        }
    }

    /// Check if the output tree contains a specific text string.
    pub fn contains_text(&self, needle: &str) -> bool {
        self.text_content().contains(needle)
    }

    /// Get the number of top-level widgets rendered in the last run.
    pub fn widget_count(&self) -> usize {
        self.last_tree
            .as_ref()
            .map(|t| t.children.len())
            .unwrap_or(0)
    }

    /// Get a typed user state value (mirrors `Ui::get_state`).
    pub fn get_state<T>(&self, key: &str, default: T) -> T
    where
        T: serde::Serialize + serde::de::DeserializeOwned + Clone + 'static,
    {
        match self.session.get_state(key) {
            Some(val) => serde_json::from_value(val.clone()).unwrap_or(default),
            None => default,
        }
    }

    /// Set a typed user state value (mirrors `Ui::set_state`).
    pub fn set_state<T>(&mut self, key: &str, value: T)
    where
        T: serde::Serialize + 'static,
    {
        if let Ok(val) = serde_json::to_value(&value) {
            self.session.set_state(key, val);
        }
    }

    /// Get the full VNode tree from the last run (for advanced assertions).
    pub fn tree(&self) -> Option<&VNode> {
        self.last_tree.as_ref()
    }

    /// Check if a specific widget type exists in the output.
    ///
    /// Widget types match CSS classes like "rustview-write", "rustview-button", etc.
    pub fn has_widget(&self, widget_class: &str) -> bool {
        let Some(ref tree) = self.last_tree else {
            return false;
        };
        Self::find_class(tree, widget_class)
    }

    fn find_class(node: &VNode, class_substr: &str) -> bool {
        if node
            .attrs
            .get("class")
            .is_some_and(|c| c.contains(class_substr))
        {
            return true;
        }
        node.children
            .iter()
            .any(|child| Self::find_class(child, class_substr))
    }
}

impl Default for TestUi {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hello_app(ui: &mut Ui) {
        ui.write("Hello, World!");
    }

    fn counter_app(ui: &mut Ui) {
        let count = ui.get_state::<i64>("n", 0);
        if ui.button("Inc") {
            ui.set_state("n", count + 1);
        }
        ui.write(format!("Count: {}", ui.get_state::<i64>("n", 0)));
    }

    fn input_app(ui: &mut Ui) {
        let name = ui.text_input("Name", "World");
        ui.write(format!("Hello, {}!", name));
    }

    fn slider_app(ui: &mut Ui) {
        let val = ui.int_slider("Amount", 0..=100);
        ui.write(format!("Amount: {}", val));
    }

    fn checkbox_app(ui: &mut Ui) {
        let checked = ui.checkbox("Enable", false);
        if checked {
            ui.write("Enabled!");
        } else {
            ui.write("Disabled.");
        }
    }

    fn multi_widget_app(ui: &mut Ui) {
        ui.heading("Dashboard");
        ui.write("Welcome");
        ui.progress(0.5);
        if ui.button("Action") {
            ui.success("Done!");
        }
    }

    #[test]
    fn test_testui_new() {
        let tui = TestUi::new();
        assert!(tui.last_tree.is_none());
        assert_eq!(tui.widget_count(), 0);
    }

    #[test]
    fn test_testui_run_hello() {
        let mut tui = TestUi::new();
        tui.run(hello_app);
        assert!(tui.contains_text("Hello, World!"));
        assert_eq!(tui.widget_count(), 1);
    }

    #[test]
    fn test_testui_text_content() {
        let mut tui = TestUi::new();
        tui.run(hello_app);
        let content = tui.text_content();
        assert!(content.contains("Hello, World!"));
    }

    #[test]
    fn test_testui_counter_default() {
        let mut tui = TestUi::new();
        tui.run(counter_app);
        assert!(tui.contains_text("Count: 0"));
    }

    #[test]
    fn test_testui_counter_increment() {
        let mut tui = TestUi::new();
        tui.click_button("Inc");
        tui.run(counter_app);
        assert!(tui.contains_text("Count: 1"));
    }

    #[test]
    fn test_testui_counter_double_increment() {
        let mut tui = TestUi::new();
        tui.click_button("Inc");
        tui.run(counter_app);
        assert!(tui.contains_text("Count: 1"));

        tui.click_button("Inc");
        tui.run(counter_app);
        assert!(tui.contains_text("Count: 2"));
    }

    #[test]
    fn test_testui_text_input() {
        let mut tui = TestUi::new();
        tui.set_input("Name", "Alice");
        tui.run(input_app);
        assert!(tui.contains_text("Hello, Alice!"));
    }

    #[test]
    fn test_testui_text_input_default() {
        let mut tui = TestUi::new();
        tui.run(input_app);
        assert!(tui.contains_text("Hello, World!"));
    }

    #[test]
    fn test_testui_slider_input() {
        let mut tui = TestUi::new();
        tui.set_input("Amount", 42);
        tui.run(slider_app);
        assert!(tui.contains_text("Amount: 42"));
    }

    #[test]
    fn test_testui_checkbox() {
        let mut tui = TestUi::new();
        tui.run(checkbox_app);
        assert!(tui.contains_text("Disabled."));

        tui.set_input("Enable", true);
        tui.run(checkbox_app);
        assert!(tui.contains_text("Enabled!"));
    }

    #[test]
    fn test_testui_has_widget() {
        let mut tui = TestUi::new();
        tui.run(multi_widget_app);
        assert!(tui.has_widget("rustview-heading"));
        assert!(tui.has_widget("rustview-write"));
        assert!(tui.has_widget("rustview-progress"));
        assert!(tui.has_widget("rustview-button"));
    }

    #[test]
    fn test_testui_widget_count() {
        let mut tui = TestUi::new();
        tui.run(multi_widget_app);
        // heading + write + progress + button = 4
        assert_eq!(tui.widget_count(), 4);
    }

    #[test]
    fn test_testui_button_one_shot() {
        let mut tui = TestUi::new();
        tui.click_button("Action");
        tui.run(multi_widget_app);
        assert!(tui.contains_text("Done!"));

        // Without clicking again, action should not fire
        tui.run(multi_widget_app);
        assert!(!tui.contains_text("Done!"));
    }

    #[test]
    fn test_testui_get_set_state() {
        let mut tui = TestUi::new();
        tui.set_state("key", 42i64);
        assert_eq!(tui.get_state::<i64>("key", 0), 42);
    }

    #[test]
    fn test_testui_state_persists_across_runs() {
        let mut tui = TestUi::new();
        tui.click_button("Inc");
        tui.run(counter_app);
        assert!(tui.contains_text("Count: 1"));

        // State should persist, run again without clicking
        tui.run(counter_app);
        assert!(tui.contains_text("Count: 1"));
    }

    #[test]
    fn test_testui_find_widget_text() {
        let mut tui = TestUi::new();
        tui.run(|ui: &mut Ui| {
            ui.heading("Title");
            ui.write("Body text");
            ui.write("More text");
        });
        let writes = tui.find_widget_text("rustview-write");
        assert_eq!(writes.len(), 2);
        assert!(writes[0].contains("Body text"));
        assert!(writes[1].contains("More text"));
    }

    #[test]
    fn test_testui_tree_access() {
        let mut tui = TestUi::new();
        tui.run(hello_app);
        let tree = tui.tree().unwrap();
        assert_eq!(tree.tag, "div");
        assert!(!tree.children.is_empty());
    }

    #[test]
    fn test_testui_default() {
        let tui = TestUi::default();
        assert!(tui.last_tree.is_none());
    }
}
