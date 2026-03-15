# Charts

RustView renders charts as inline SVG — no JavaScript libraries required.

All charts are generated server-side as SVG `<svg>` elements embedded directly into the DOM.

---

## `line_chart`

Display a line chart connecting `(x, y)` data points.

```rust
let data = vec![
    (0.0, 15.0),
    (1.0, 18.2),
    (2.0, 17.5),
    (3.0, 21.3),
    (4.0, 24.5),
    (5.0, 22.0),
    (6.0, 23.8),
];
ui.line_chart("Temperature (°C)", &data);
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `title` | `&str` | Chart title |
| `data` | `&[(f64, f64)]` | Slice of (x, y) data points |

---

## `bar_chart`

Display a bar chart with labeled categories.

```rust
ui.bar_chart("Sales by Region", &[
    ("Europe", 420.0),
    ("Asia", 380.0),
    ("Americas", 310.0),
    ("Africa", 150.0),
    ("Oceania", 90.0),
]);
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `title` | `&str` | Chart title |
| `data` | `&[(&str, f64)]` | Slice of (label, value) pairs |

---

## `scatter_chart`

Display a scatter plot of `(x, y)` points.

```rust
let points = vec![
    (160.0, 55.0), (170.0, 65.0), (175.0, 70.0),
    (180.0, 78.0), (185.0, 85.0), (168.0, 72.0),
];
ui.scatter_chart("Height vs Weight", &points);
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `title` | `&str` | Chart title |
| `data` | `&[(f64, f64)]` | Slice of (x, y) points |

---

## `histogram`

Display a histogram from raw data values.

```rust
let response_times = vec![12.5, 14.2, 15.8, 11.0, 18.3, 22.1, 25.0, 30.0, 16.4, 19.7];
ui.histogram("Response Time Distribution", &response_times, 5);
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `title` | `&str` | Chart title |
| `data` | `&[f64]` | Raw data values |
| `bins` | `u32` | Number of histogram bins |

---

## Styling

Charts use the theme's primary color (`--rustview-primary`, default `#ff4b4b`) for lines, bars, and points. Axes and labels use the secondary text color (`--rustview-text-secondary`).

Charts are rendered in a 600×300 SVG viewBox and are responsive — they scale to fit their container width.

## Charts in Columns

```rust
ui.columns(&[1, 1], &[
    &|c: &mut Ui| {
        c.line_chart("Revenue", &[(0.0, 100.0), (1.0, 120.0), (2.0, 115.0)]);
    },
    &|c: &mut Ui| {
        c.bar_chart("Categories", &[("A", 30.0), ("B", 50.0), ("C", 20.0)]);
    },
]);
```
