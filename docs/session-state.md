# Session State

RustView provides two mechanisms for persisting data between re-renders: **widget state** (automatic) and **user state** (explicit).

---

## Widget State (Automatic)

Every input widget automatically persists its value in the session. When the app re-runs, each widget reads its current value from the session:

```rust
fn app(ui: &mut Ui) {
    // First render: returns "World" (the default)
    // After user types "Alice": returns "Alice"
    let name = ui.text_input("Your name", "World");
    ui.write(format!("Hello, {name}!"));
}
```

You don't need to manage widget state manually — it's handled by the framework.

---

## User State (Explicit)

For data that isn't tied to a specific widget (counters, computed values, lists), use `get_state` and `set_state`:

```rust
fn app(ui: &mut Ui) {
    // Read counter (default 0 if not set)
    let count: i64 = ui.get_state("counter", 0);

    if ui.button("➕ Increment") {
        ui.set_state("counter", count + 1);
    }
    if ui.button("➖ Decrement") {
        ui.set_state("counter", count - 1);
    }
    if ui.button("🔄 Reset") {
        ui.set_state("counter", 0_i64);
    }

    ui.metric("Counter", count, None);
}
```

### `get_state`

```rust
let value: T = ui.get_state("key", default_value);
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `key` | `&str` | State key name |
| `default` | `T` | Value to return if the key doesn't exist |
| **Returns** | `T` | Current value (must be `Serialize + DeserializeOwned + Clone`) |

### `set_state`

```rust
ui.set_state("key", new_value);
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `key` | `&str` | State key name |
| `value` | `T` | Value to store (must be `Serialize`) |

---

## State Types

State values are serialized to JSON internally. Any type that implements `serde::Serialize` and `serde::de::DeserializeOwned` can be used:

```rust
// Primitives
let count: i64 = ui.get_state("count", 0);
let name: String = ui.get_state("name", "".to_string());
let flag: bool = ui.get_state("flag", false);

// Collections
let items: Vec<String> = ui.get_state("items", vec![]);

// Custom types (with #[derive(Serialize, Deserialize, Clone)])
let config: MyConfig = ui.get_state("config", MyConfig::default());
```

---

## Session Lifecycle

- Each browser tab gets its own session (identified by a UUID).
- Sessions expire after a configurable TTL (default: 24 hours).
- Session data is stored in-memory using `DashMap` (concurrent hash map).
- When a session expires, all widget and user state is lost.

Configure the TTL:

```rust
let config = RustViewConfig {
    session_ttl_secs: 3600, // 1 hour
    ..Default::default()
};
rustview::run_with_config(app, config);
```

---

## Common Patterns

### Growing a List

```rust
fn app(ui: &mut Ui) {
    let mut items: Vec<String> = ui.get_state("items", vec![]);
    let new_item = ui.text_input("New item", "");

    if ui.button("Add") && !new_item.is_empty() {
        items.push(new_item);
        ui.set_state("items", &items);
    }

    for item in &items {
        ui.write(item);
    }
}
```

### Toggle Visibility

```rust
fn app(ui: &mut Ui) {
    let show: bool = ui.get_state("show_details", false);

    if ui.button("Toggle Details") {
        ui.set_state("show_details", !show);
    }

    if show {
        ui.container(|c| {
            c.write("These are the details!");
        });
    }
}
```
