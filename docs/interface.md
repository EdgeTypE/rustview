# Interface Mode

Interface mode provides a Gradio-style API that automatically generates a UI from a plain Rust function. No manual widget placement needed.

---

## Basic Usage

```rust
use rustview::prelude::*;

fn greet(name: String) -> String {
    format!("Hello, {name}!")
}

fn main() {
    Interface::from_fn(greet)
        .title("Greeting App")
        .description("Enter a name to get a greeting.")
        .launch();
}
```

This generates a UI with:
- A text input for the `name` parameter
- A text display for the `String` return value
- Automatic re-computation when the input changes

---

## Multi-Argument Functions

### Two Arguments

```rust
fn add(a: f64, b: f64) -> f64 {
    a + b
}

Interface::from_fn2(add)
    .title("Calculator")
    .launch();
```

### Three Arguments

```rust
fn describe(name: String, age: i64, is_member: bool) -> String {
    let status = if is_member { "member" } else { "guest" };
    format!("{name} is {age} years old ({status})")
}

Interface::from_fn3(describe)
    .title("Person Info")
    .launch();
```

---

## Builder Methods

| Method | Description |
|--------|-------------|
| `.title("Title")` | Set the title displayed at the top |
| `.description("Desc")` | Set the description below the title |
| `.examples(vec!["input1", "input2"])` | Add clickable example inputs |
| `.launch()` | Start the server with default config |
| `.launch_async()` | Start the server from an async context |
| `.build_app()` | Build the app closure without launching |

---

## Supported Types

### Input Types (`WidgetInput`)

Types that can be automatically rendered as input widgets:

| Type | Widget |
|------|--------|
| `String` | `text_input` |
| `f64` | `number_input` |
| `f32` | `number_input` (converted) |
| `i64` | `int_input` |
| `bool` | `checkbox` |

### Output Types (`WidgetOutput`)

Types that can be automatically rendered as output widgets:

| Type | Widget |
|------|--------|
| `String` | `write` |
| `f64` | `write` |
| `f32` | `write` |
| `i64` | `write` |
| `bool` | `write` |
| `Vec<String>` | `write` (each item) |
| `(A, B)` | Two output widgets |
| `(A, B, C)` | Three output widgets |

---

## Examples with Preset Inputs

```rust
Interface::from_fn(|name: String| format!("Hello, {name}!"))
    .title("Greeting")
    .examples(vec!["Alice", "Bob", "World"])
    .launch();
```

Example inputs appear as clickable buttons. Clicking one fills in the input field.

---

## Testing Interface Apps

Use `build_app()` to get the app closure for testing:

```rust
use rustview::testing::TestUi;

let app = Interface::from_fn(|name: String| format!("Hello, {name}!"))
    .title("Test")
    .build_app();

let mut test = TestUi::new();
test.set_input("Input 1", "Alice");
test.run(&app);
assert!(test.contains_text("Hello, Alice!"));
```

---

## Async Support

```rust
#[tokio::main]
async fn main() {
    Interface::from_fn(|x: f64| x * x)
        .title("Square Calculator")
        .launch_async()
        .await;
}
```
