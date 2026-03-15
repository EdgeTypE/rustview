# Layouts

RustView provides layout primitives for organizing widgets into columns, sidebars, tabs, and more.

---

## `columns`

Create side-by-side columns with specified width ratios.

```rust
ui.columns(
    &[1, 1, 1],  // 3 equal-width columns
    &[
        &|col: &mut Ui| {
            col.metric("Users", 12_456, Some(5.2));
        },
        &|col: &mut Ui| {
            col.metric("Revenue", "$98.7K", Some(-2.1));
        },
        &|col: &mut Ui| {
            col.metric("Uptime", "99.99%", None);
        },
    ],
);
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `ratios` | `&[u32]` | Relative width of each column (e.g., `&[2, 1]` = 2:1) |
| `builders` | `&[&dyn Fn(&mut Ui)]` | Closure for each column's content |

Columns use CSS Grid with `fr` units, so `&[1, 2, 1]` creates a 1fr–2fr–1fr grid.

---

## `sidebar`

Render widgets inside a fixed sidebar panel on the left side of the page.

```rust
ui.sidebar(|sb| {
    sb.heading("Settings");
    let theme = sb.select("Theme", &["Dark", "Light"]);
    sb.divider();
    sb.caption("v0.2.0");
});
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `f` | `impl FnOnce(&mut Ui)` | Closure that builds sidebar content |

The sidebar is always rendered before the main content area. It stays fixed on scroll.

---

## `container`

Render widgets inside a bordered vertical block.

```rust
ui.container(|c| {
    c.subheading("Details");
    c.write("Content inside a container.");
});
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `f` | `impl FnOnce(&mut Ui)` | Closure that builds container content |

Containers provide visual grouping with a border and padding.

---

## `expander`

Collapsible section — click to expand/collapse.

```rust
ui.expander("Advanced Settings", |exp| {
    exp.write("These are hidden by default.");
    let debug = exp.checkbox("Debug mode", false);
});
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `label` | `&str` | Expander title |
| `f` | `impl FnOnce(&mut Ui)` | Closure that builds expander content |

The expander remembers its open/closed state across re-renders.

---

## `tabs`

Create a tabbed layout. Returns the index of the currently selected tab.

```rust
let active_tab = ui.tabs(&["Overview", "Details", "Settings"], |tab_ui| {
    tab_ui.write("Content for the active tab appears here.");
});
// active_tab: usize — 0, 1, or 2
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `labels` | `&[&str]` | Tab labels |
| `f` | `impl FnOnce(&mut Ui)` | Closure for the active tab's content |
| **Returns** | `usize` | Index of the currently selected tab |

Use the return value to conditionally render different content:

```rust
let tab = ui.tabs(&["Users", "Logs", "Config"], |tab_ui| {
    // This closure receives the inner Ui for the tab body
});

match tab {
    0 => ui.table(&["Name", "Role"], &[/* ... */]),
    1 => ui.code("2025-01-15 INFO: started", "log"),
    2 => ui.write("Config options here"),
    _ => {}
}
```

---

## `row`

Render widgets in a horizontal flex row.

```rust
ui.row(|r| {
    r.write("Item A");
    r.write("Item B");
    r.write("Item C");
    r.badge("inline", "blue");
});
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `f` | `impl FnOnce(&mut Ui)` | Closure that builds row content |

Items are laid out horizontally with flex-wrap.

---

## `modal`

Render a modal dialog overlay with a trigger button.

```rust
let submitted = ui.modal("Confirm Action", "Open Modal", |m| {
    m.write("Are you sure you want to proceed?");
    m.warning("This action cannot be undone.");
});

if submitted {
    // Modal was confirmed
}
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `title` | `&str` | Modal title |
| `trigger_label` | `&str` | Button text that opens the modal |
| `f` | `impl FnOnce(&mut Ui)` | Closure that builds modal body content |
| **Returns** | `bool` | `true` if the modal was submitted/confirmed |

---

## `form`

Create a form container that batches all widget updates until a submit button is clicked.

```rust
let submitted = ui.form("contact_form", |f| {
    let name = f.text_input("Full name", "");
    let email = f.text_input("Email", "");
    let message = f.text_area("Message", "", 3);
    let subscribe = f.checkbox("Subscribe to newsletter", false);
    f.form_submit_button("Send Message");
});
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `key` | `&str` | Unique form key |
| `f` | `impl FnOnce(&mut Ui)` | Closure that builds form content |
| **Returns** | `bool` | `true` when the form is submitted |

Inside a form, widget values are not committed to session state until the submit button is clicked. This prevents intermediate state changes during user input.

### `form_submit_button`

```rust
f.form_submit_button("Send");
```

Must be called inside a `form()` closure. Renders a submit button that triggers the batch update.

---

## `empty`

Create an empty placeholder that can be filled later.

```rust
let slot_id = ui.empty();
// ... later logic can update this slot
```

| **Returns** | `String` | The placeholder widget ID |

---

## Nesting Layouts

Layouts can be nested freely:

```rust
ui.sidebar(|sb| {
    sb.heading("Filters");
    sb.columns(&[1, 1], &[
        &|c: &mut Ui| c.checkbox("Active", true),
        &|c: &mut Ui| c.checkbox("Verified", false),
    ]);
});

ui.columns(&[2, 1], &[
    &|main: &mut Ui| {
        main.container(|c| {
            c.subheading("Main Content");
            c.table(&["A", "B"], &[vec!["1".into(), "2".into()]]);
        });
    },
    &|aside: &mut Ui| {
        aside.expander("More Info", |e| {
            e.write("Hidden details here.");
        });
    },
]);
```
