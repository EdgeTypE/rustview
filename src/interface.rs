//! Interface mode — Gradio-style `fn → UI` automatic generation.
//!
//! Wraps a plain Rust function and automatically generates input/output widgets
//! based on type inference. Inspired by Gradio's `gr.Interface(fn, inputs, outputs)`.
//!
//! # Example
//! ```rust,ignore
//! fn greet(name: String) -> String {
//!     format!("Hello, {}!", name)
//! }
//!
//! fn main() {
//!     rustview::Interface::from_fn(greet)
//!         .title("Greeter")
//!         .description("Enter a name to greet.")
//!         .launch();
//! }
//! ```
//!
//! # Type Inference Rules
//! | Rust Type | Input Widget | Output Widget |
//! |-----------|-------------|---------------|
//! | `String` | `text_input` | `write` |
//! | `f64` | `number_input` | `metric` |
//! | `f32` | `number_input` | `metric` |
//! | `i64` | `int_input` | `metric` |
//! | `bool` | `checkbox` | `write` |
//! | `Vec<String>` | `multi_select` | `json` |
//! | `(A, B)` | — | two-field output |

use crate::ui::Ui;
use serde::{de::DeserializeOwned, Serialize};

/// Trait for types that can be automatically rendered as an input widget.
pub trait WidgetInput: Sized + Send + Sync + 'static {
    /// Render this type's input widget and return the current value.
    fn render_input(ui: &mut Ui, label: &str) -> Self;
}

/// Trait for types that can be automatically rendered as an output widget.
pub trait WidgetOutput: Serialize + DeserializeOwned + Send + Sync + 'static {
    /// Render this type's output widget.
    fn render_output(ui: &mut Ui, label: &str, value: &Self);
}

// ---- WidgetInput implementations ----

impl WidgetInput for String {
    fn render_input(ui: &mut Ui, label: &str) -> Self {
        ui.text_input(label, "")
    }
}

impl WidgetInput for f64 {
    fn render_input(ui: &mut Ui, label: &str) -> Self {
        ui.number_input(label, 0.0)
    }
}

impl WidgetInput for f32 {
    /// Renders as `number_input` returning `f64`, then converts to `f32`.
    /// Minor precision loss is possible for values beyond f32 range.
    fn render_input(ui: &mut Ui, label: &str) -> Self {
        ui.number_input(label, 0.0) as f32
    }
}

impl WidgetInput for i64 {
    fn render_input(ui: &mut Ui, label: &str) -> Self {
        ui.int_input(label, 0)
    }
}

impl WidgetInput for bool {
    fn render_input(ui: &mut Ui, label: &str) -> Self {
        ui.checkbox(label, false)
    }
}

// ---- WidgetOutput implementations ----

impl WidgetOutput for String {
    fn render_output(ui: &mut Ui, label: &str, value: &Self) {
        ui.subheading(label);
        ui.write(value);
    }
}

impl WidgetOutput for f64 {
    fn render_output(ui: &mut Ui, label: &str, value: &Self) {
        ui.metric(label, value, None);
    }
}

impl WidgetOutput for f32 {
    fn render_output(ui: &mut Ui, label: &str, value: &Self) {
        ui.metric(label, value, None);
    }
}

impl WidgetOutput for i64 {
    fn render_output(ui: &mut Ui, label: &str, value: &Self) {
        ui.metric(label, value, None);
    }
}

impl WidgetOutput for bool {
    fn render_output(ui: &mut Ui, label: &str, value: &Self) {
        ui.subheading(label);
        ui.write(if *value { "true" } else { "false" });
    }
}

impl WidgetOutput for Vec<String> {
    fn render_output(ui: &mut Ui, label: &str, value: &Self) {
        ui.subheading(label);
        ui.json(value);
    }
}

// Tuple output: (A, B)
impl<A: WidgetOutput, B: WidgetOutput> WidgetOutput for (A, B) {
    fn render_output(ui: &mut Ui, _label: &str, value: &Self) {
        A::render_output(ui, "Result 1", &value.0);
        B::render_output(ui, "Result 2", &value.1);
    }
}

// Tuple output: (A, B, C)
impl<A: WidgetOutput, B: WidgetOutput, C: WidgetOutput> WidgetOutput for (A, B, C) {
    fn render_output(ui: &mut Ui, _label: &str, value: &Self) {
        A::render_output(ui, "Result 1", &value.0);
        B::render_output(ui, "Result 2", &value.1);
        C::render_output(ui, "Result 3", &value.2);
    }
}

/// The Interface builder — wraps a function and generates a complete UI.
///
/// Use `Interface::from_fn()` to create from a 1-argument function,
/// or `Interface::from_fn2()` for 2-argument functions.
///
/// # Example
/// ```rust,ignore
/// fn double(x: f64) -> f64 {
///     x * 2.0
/// }
///
/// fn main() {
///     rustview::Interface::from_fn(double)
///         .title("Doubler")
///         .description("Doubles the input number.")
///         .launch();
/// }
/// ```
pub struct Interface {
    app_fn: Box<dyn Fn(&mut Ui) + Send + Sync + 'static>,
    title: String,
    description: String,
    _examples: Vec<String>,
}

impl Interface {
    /// Create an Interface from a 1-argument function.
    ///
    /// The argument and return types are inferred from the function signature.
    /// Input type determines the input widget; return type determines output widget.
    pub fn from_fn<A, R>(func: impl Fn(A) -> R + Send + Sync + 'static) -> Self
    where
        A: WidgetInput,
        R: WidgetOutput,
    {
        let app_fn: Box<dyn Fn(&mut Ui) + Send + Sync + 'static> =
            Box::new(move |ui: &mut Ui| {
                let input = A::render_input(ui, "Input");

                if ui.button("Run") {
                    let result = func(input);
                    if let Ok(val) = serde_json::to_value(&result) {
                        ui.set_state("__iface_result", Some(val));
                    }
                }

                ui.divider();

                if let Some(val) =
                    ui.get_state::<Option<serde_json::Value>>("__iface_result", None)
                {
                    if let Ok(result) = serde_json::from_value::<R>(val) {
                        R::render_output(ui, "Result", &result);
                    }
                }
            });

        Interface {
            app_fn,
            title: "RustView App".to_string(),
            description: String::new(),
            _examples: Vec::new(),
        }
    }

    /// Create an Interface from a 2-argument function.
    ///
    /// Both argument types are inferred and rendered as separate input widgets.
    pub fn from_fn2<A, B, R>(func: impl Fn(A, B) -> R + Send + Sync + 'static) -> Self
    where
        A: WidgetInput,
        B: WidgetInput,
        R: WidgetOutput,
    {
        let app_fn: Box<dyn Fn(&mut Ui) + Send + Sync + 'static> =
            Box::new(move |ui: &mut Ui| {
                let input1 = A::render_input(ui, "Input 1");
                let input2 = B::render_input(ui, "Input 2");

                if ui.button("Run") {
                    let result = func(input1, input2);
                    if let Ok(val) = serde_json::to_value(&result) {
                        ui.set_state("__iface_result", Some(val));
                    }
                }

                ui.divider();

                if let Some(val) =
                    ui.get_state::<Option<serde_json::Value>>("__iface_result", None)
                {
                    if let Ok(result) = serde_json::from_value::<R>(val) {
                        R::render_output(ui, "Result", &result);
                    }
                }
            });

        Interface {
            app_fn,
            title: "RustView App".to_string(),
            description: String::new(),
            _examples: Vec::new(),
        }
    }

    /// Create an Interface from a 3-argument function.
    pub fn from_fn3<A, B, C, R>(func: impl Fn(A, B, C) -> R + Send + Sync + 'static) -> Self
    where
        A: WidgetInput,
        B: WidgetInput,
        C: WidgetInput,
        R: WidgetOutput,
    {
        let app_fn: Box<dyn Fn(&mut Ui) + Send + Sync + 'static> =
            Box::new(move |ui: &mut Ui| {
                let input1 = A::render_input(ui, "Input 1");
                let input2 = B::render_input(ui, "Input 2");
                let input3 = C::render_input(ui, "Input 3");

                if ui.button("Run") {
                    let result = func(input1, input2, input3);
                    if let Ok(val) = serde_json::to_value(&result) {
                        ui.set_state("__iface_result", Some(val));
                    }
                }

                ui.divider();

                if let Some(val) =
                    ui.get_state::<Option<serde_json::Value>>("__iface_result", None)
                {
                    if let Ok(result) = serde_json::from_value::<R>(val) {
                        R::render_output(ui, "Result", &result);
                    }
                }
            });

        Interface {
            app_fn,
            title: "RustView App".to_string(),
            description: String::new(),
            _examples: Vec::new(),
        }
    }

    /// Set the title displayed at the top of the UI.
    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    /// Set the description displayed below the title.
    pub fn description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }

    /// Add example inputs (displayed as clickable presets).
    pub fn examples(mut self, examples: Vec<&str>) -> Self {
        self._examples = examples.into_iter().map(|s| s.to_string()).collect();
        self
    }

    /// Launch the Interface app with default configuration.
    ///
    /// Starts the HTTP server and opens the browser.
    pub fn launch(self) {
        let title = self.title;
        let description = self.description;
        let app_fn = self.app_fn;

        crate::run(move |ui: &mut Ui| {
            ui.heading(&title);
            if !description.is_empty() {
                ui.caption(&description);
            }
            ui.divider();
            app_fn(ui);
        });
    }

    /// Launch the Interface app from an existing async runtime.
    pub async fn launch_async(self) {
        let title = self.title;
        let description = self.description;
        let app_fn = self.app_fn;

        crate::run_async(move |ui: &mut Ui| {
            ui.heading(&title);
            if !description.is_empty() {
                ui.caption(&description);
            }
            ui.divider();
            app_fn(ui);
        })
        .await;
    }

    /// Build the app function without launching (useful for testing).
    pub fn build_app(self) -> impl Fn(&mut Ui) + Send + Sync + 'static {
        let title = self.title;
        let description = self.description;
        let app_fn = self.app_fn;

        move |ui: &mut Ui| {
            ui.heading(&title);
            if !description.is_empty() {
                ui.caption(&description);
            }
            ui.divider();
            app_fn(ui);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::TestUi;

    fn greet(name: String) -> String {
        format!("Hello, {}!", name)
    }

    fn double(x: f64) -> f64 {
        x * 2.0
    }

    fn classify(text: String) -> (String, f32) {
        let label = if text.contains("great") {
            "Positive"
        } else {
            "Negative"
        };
        (label.into(), 0.87)
    }

    fn negate(b: bool) -> bool {
        !b
    }

    fn add_ints(x: i64) -> i64 {
        x + 10
    }

    fn add_two(a: f64, b: f64) -> f64 {
        a + b
    }

    #[test]
    fn test_widget_input_string() {
        let mut tui = TestUi::new();
        tui.set_input("Name", "Alice");
        tui.run(|ui| {
            let val = String::render_input(ui, "Name");
            ui.write(format!("Got: {}", val));
        });
        assert!(tui.contains_text("Got: Alice"));
    }

    #[test]
    fn test_widget_input_f64() {
        let mut tui = TestUi::new();
        tui.set_input("X", 3.14);
        tui.run(|ui| {
            let val = f64::render_input(ui, "X");
            ui.write(format!("Got: {}", val));
        });
        assert!(tui.contains_text("Got: 3.14"));
    }

    #[test]
    fn test_widget_input_bool() {
        let mut tui = TestUi::new();
        tui.set_input("Flag", true);
        tui.run(|ui| {
            let val = bool::render_input(ui, "Flag");
            ui.write(format!("Got: {}", val));
        });
        assert!(tui.contains_text("Got: true"));
    }

    #[test]
    fn test_widget_input_i64() {
        let mut tui = TestUi::new();
        tui.set_input("Count", 42);
        tui.run(|ui| {
            let val = i64::render_input(ui, "Count");
            ui.write(format!("Got: {}", val));
        });
        assert!(tui.contains_text("Got: 42"));
    }

    #[test]
    fn test_widget_output_string() {
        let mut tui = TestUi::new();
        tui.run(|ui| {
            String::render_output(ui, "Greeting", &"Hello!".to_string());
        });
        assert!(tui.contains_text("Hello!"));
    }

    #[test]
    fn test_widget_output_f64() {
        let mut tui = TestUi::new();
        tui.run(|ui| {
            f64::render_output(ui, "Score", &0.95);
        });
        assert!(tui.has_widget("rustview-metric"));
    }

    #[test]
    fn test_widget_output_bool() {
        let mut tui = TestUi::new();
        tui.run(|ui| {
            bool::render_output(ui, "Active", &true);
        });
        assert!(tui.contains_text("true"));
    }

    #[test]
    fn test_widget_output_tuple() {
        let mut tui = TestUi::new();
        tui.run(|ui| {
            <(String, f32)>::render_output(ui, "Results", &("Positive".to_string(), 0.87f32));
        });
        assert!(tui.contains_text("Positive"));
    }

    #[test]
    fn test_interface_builder() {
        let iface = Interface::from_fn(greet).title("Greeter").description("Enter a name");
        assert_eq!(iface.title, "Greeter");
        assert_eq!(iface.description, "Enter a name");
    }

    #[test]
    fn test_interface_build_app() {
        let app = Interface::from_fn(greet)
            .title("Greeter")
            .description("Enter a name to greet.")
            .build_app();

        let mut tui = TestUi::new();
        tui.run(&app);
        assert!(tui.contains_text("Greeter"));
        assert!(tui.contains_text("Enter a name to greet."));
        assert!(tui.has_widget("rustview-button"));
    }

    #[test]
    fn test_interface_run_and_click() {
        let app = Interface::from_fn(double)
            .title("Doubler")
            .description("Doubles a number.")
            .build_app();

        let mut tui = TestUi::new();
        tui.set_input("Input", 5.0);
        tui.click_button("Run");
        tui.run(&app);
        assert!(tui.contains_text("Doubler"));
        // After clicking Run, the result should appear
        assert!(tui.contains_text("10"));
    }

    #[test]
    fn test_interface_classify_fn() {
        let app = Interface::from_fn(classify).title("Classifier").build_app();

        let mut tui = TestUi::new();
        tui.set_input("Input", "This is great!");
        tui.click_button("Run");
        tui.run(&app);
        assert!(tui.contains_text("Classifier"));
    }

    #[test]
    fn test_interface_bool_fn() {
        let app = Interface::from_fn(negate).title("Negator").build_app();

        let mut tui = TestUi::new();
        tui.set_input("Input", true);
        tui.click_button("Run");
        tui.run(&app);
        assert!(tui.contains_text("Negator"));
        assert!(tui.contains_text("false"));
    }

    #[test]
    fn test_interface_i64_fn() {
        let app = Interface::from_fn(add_ints).title("Adder").build_app();

        let mut tui = TestUi::new();
        tui.set_input("Input", 5);
        tui.click_button("Run");
        tui.run(&app);
        assert!(tui.contains_text("Adder"));
        assert!(tui.contains_text("15"));
    }

    #[test]
    fn test_interface_2arg_fn() {
        let app = Interface::from_fn2(add_two)
            .title("Adder")
            .description("Adds two numbers.")
            .build_app();

        let mut tui = TestUi::new();
        tui.set_input("Input 1", 3.0);
        tui.set_input("Input 2", 7.0);
        tui.click_button("Run");
        tui.run(&app);
        assert!(tui.contains_text("Adder"));
        assert!(tui.contains_text("10"));
    }

    #[test]
    fn test_interface_examples() {
        let iface = Interface::from_fn(greet).examples(vec!["Alice", "Bob"]);
        assert_eq!(iface._examples.len(), 2);
    }

    #[test]
    fn test_interface_default_title() {
        let iface = Interface::from_fn(greet);
        assert_eq!(iface.title, "RustView App");
    }

    #[test]
    fn test_interface_no_result_before_run() {
        let app = Interface::from_fn(double).title("Test").build_app();

        let mut tui = TestUi::new();
        tui.run(&app);
        // No "Run" button clicked, so no output result
        assert!(!tui.contains_text("Result"));
    }
}
