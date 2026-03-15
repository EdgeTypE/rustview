# Testing

RustView includes a `TestUi` harness for unit testing apps without starting a server.

---

## Setup

```rust
use rustview::testing::TestUi;
use rustview::ui::Ui;
```

---

## Basic Testing

```rust
#[test]
fn test_hello_app() {
    fn app(ui: &mut Ui) {
        let name = ui.text_input("Name", "World");
        ui.write(format!("Hello, {name}!"));
    }

    let mut test = TestUi::new();
    test.run(app);
    assert!(test.contains_text("Hello, World!"));
}
```

---

## Simulating User Input

### Setting Text Input

```rust
#[test]
fn test_custom_name() {
    fn app(ui: &mut Ui) {
        let name = ui.text_input("Name", "World");
        ui.write(format!("Hello, {name}!"));
    }

    let mut test = TestUi::new();
    test.set_input("Name", "Alice");
    test.run(app);
    assert!(test.contains_text("Hello, Alice!"));
}
```

### Clicking Buttons

```rust
#[test]
fn test_button_click() {
    fn app(ui: &mut Ui) {
        let count: i64 = ui.get_state("count", 0);
        if ui.button("Increment") {
            ui.set_state("count", count + 1);
        }
        ui.write(format!("Count: {count}"));
    }

    let mut test = TestUi::new();

    // First run — count is 0
    test.run(app);
    assert!(test.contains_text("Count: 0"));

    // Click and re-run — count becomes 1
    test.click_button("Increment");
    test.run(app);
    assert!(test.contains_text("Count: 1"));
}
```

---

## TestUi API Reference

### Construction

```rust
let mut test = TestUi::new();
```

### Input Simulation

```rust
test.set_input("Label", value);     // Set any widget's value by label
test.click_button("Label");          // Simulate a button click by label
```

### Running the App

```rust
test.run(app_fn);                    // Run the app function once
```

### Output Assertions

| Method | Returns | Description |
|--------|---------|-------------|
| `text_content()` | `String` | All text content concatenated |
| `contains_text("needle")` | `bool` | Check if output contains text |
| `find_widget_text("class")` | `Vec<String>` | Find widgets by CSS class |
| `widget_count()` | `usize` | Number of top-level widgets |
| `has_widget("class")` | `bool` | Check if a widget type exists |
| `tree()` | `Option<&VNode>` | Get the full VNode tree |

### State Access

```rust
let value: T = test.get_state("key", default);
test.set_state("key", value);
```

---

## Testing Patterns

### Test Multiple Interactions

```rust
#[test]
fn test_counter_up_and_down() {
    fn app(ui: &mut Ui) {
        let count: i64 = ui.get_state("c", 0);
        if ui.button("Up") { ui.set_state("c", count + 1); }
        if ui.button("Down") { ui.set_state("c", count - 1); }
        ui.metric("Count", count, None);
    }

    let mut t = TestUi::new();

    t.click_button("Up");
    t.run(app);
    t.click_button("Up");
    t.run(app);
    t.click_button("Up");
    t.run(app);
    assert_eq!(t.get_state::<i64>("c", 0), 3);

    t.click_button("Down");
    t.run(app);
    assert_eq!(t.get_state::<i64>("c", 0), 2);
}
```

### Test Widget Existence

```rust
#[test]
fn test_widgets_rendered() {
    fn app(ui: &mut Ui) {
        ui.heading("Title");
        ui.progress(0.5);
        ui.spinner("Loading...");
    }

    let mut t = TestUi::new();
    t.run(app);
    assert!(t.has_widget("rustview-heading"));
    assert!(t.has_widget("rustview-progress"));
    assert!(t.has_widget("rustview-spinner"));
    assert_eq!(t.widget_count(), 3);
}
```

### Test Interface Apps

```rust
#[test]
fn test_interface_square() {
    use rustview::prelude::*;

    let app = Interface::from_fn(|x: f64| x * x)
        .title("Square")
        .build_app();

    let mut t = TestUi::new();
    t.set_input("Input 1", 5.0f64);
    t.run(&app);
    assert!(t.contains_text("25"));
}
```
