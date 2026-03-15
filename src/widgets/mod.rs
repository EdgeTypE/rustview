/// Widget rendering — converts widget calls to VNode trees.
///
/// Each widget type maps to a specific VNode structure that the browser
/// shim knows how to render and handle events for.
use crate::vdom::VNode;

/// Render a text input widget as a VNode.
pub fn render_text_input(widget_id: &str, label: &str, value: &str) -> VNode {
    VNode::new(widget_id, "div")
        .with_attr("class", "rustview-widget rustview-text-input")
        .with_child(
            VNode::new(format!("{widget_id}-label"), "label")
                .with_text(label)
                .with_attr("for", format!("{widget_id}-input")),
        )
        .with_child(
            VNode::new(format!("{widget_id}-input"), "input")
                .with_attr("type", "text")
                .with_attr("value", value)
                .with_attr("data-widget-id", widget_id)
                .with_attr("data-widget-type", "text_input"),
        )
}

/// Render an integer slider widget as a VNode.
pub fn render_slider(widget_id: &str, label: &str, value: i64, min: i64, max: i64) -> VNode {
    VNode::new(widget_id, "div")
        .with_attr("class", "rustview-widget rustview-slider")
        .with_child(
            VNode::new(format!("{widget_id}-label"), "label")
                .with_text(format!("{label}: {value}")),
        )
        .with_child(
            VNode::new(format!("{widget_id}-input"), "input")
                .with_attr("type", "range")
                .with_attr("min", min.to_string())
                .with_attr("max", max.to_string())
                .with_attr("value", value.to_string())
                .with_attr("data-widget-id", widget_id)
                .with_attr("data-widget-type", "slider"),
        )
}

/// Render a checkbox widget as a VNode.
pub fn render_checkbox(widget_id: &str, label: &str, checked: bool) -> VNode {
    let mut input = VNode::new(format!("{widget_id}-input"), "input")
        .with_attr("type", "checkbox")
        .with_attr("data-widget-id", widget_id)
        .with_attr("data-widget-type", "checkbox");
    if checked {
        input = input.with_attr("checked", "true");
    }

    VNode::new(widget_id, "div")
        .with_attr("class", "rustview-widget rustview-checkbox")
        .with_child(
            VNode::new(format!("{widget_id}-label-wrapper"), "label")
                .with_child(input)
                .with_child(VNode::new(format!("{widget_id}-label"), "span").with_text(label)),
        )
}

/// Render a button widget as a VNode.
pub fn render_button(widget_id: &str, label: &str) -> VNode {
    VNode::new(widget_id, "div")
        .with_attr("class", "rustview-widget rustview-button")
        .with_child(
            VNode::new(format!("{widget_id}-btn"), "button")
                .with_text(label)
                .with_attr("data-widget-id", widget_id)
                .with_attr("data-widget-type", "button"),
        )
}

/// Render a write/text display widget as a VNode.
pub fn render_write(widget_id: &str, text: &str) -> VNode {
    VNode::new(widget_id, "div")
        .with_attr("class", "rustview-widget rustview-write")
        .with_child(VNode::new(format!("{widget_id}-text"), "p").with_text(text))
}

/// Render a markdown display widget as a VNode.
pub fn render_markdown(widget_id: &str, text: &str) -> VNode {
    VNode::new(widget_id, "div")
        .with_attr("class", "rustview-widget rustview-markdown")
        .with_child(
            VNode::new(format!("{widget_id}-content"), "div")
                .with_text(text)
                .with_attr("data-markdown", "true"),
        )
}

/// Render a progress bar widget as a VNode.
pub fn render_progress(widget_id: &str, value: f64) -> VNode {
    let percent = (value.clamp(0.0, 1.0) * 100.0) as u32;
    VNode::new(widget_id, "div")
        .with_attr("class", "rustview-widget rustview-progress")
        .with_child(
            VNode::new(format!("{widget_id}-bar-bg"), "div")
                .with_attr("class", "rustview-progress-bg")
                .with_child(
                    VNode::new(format!("{widget_id}-bar"), "div")
                        .with_attr("class", "rustview-progress-fill")
                        .with_attr("style", format!("width: {percent}%")),
                ),
        )
        .with_child(
            VNode::new(format!("{widget_id}-label"), "span").with_text(format!("{percent}%")),
        )
}

/// Render an error alert widget as a VNode.
pub fn render_error(widget_id: &str, message: &str) -> VNode {
    VNode::new(widget_id, "div")
        .with_attr("class", "rustview-widget rustview-alert rustview-alert-error")
        .with_child(
            VNode::new(format!("{widget_id}-icon"), "span")
                .with_attr("class", "rustview-alert-icon")
                .with_text("\u{26A0}"),
        )
        .with_child(
            VNode::new(format!("{widget_id}-msg"), "span")
                .with_attr("class", "rustview-alert-message")
                .with_text(message),
        )
}

// ── Input widgets ──────────────────────────────────────────────────────

/// Render a numeric (floating-point) input widget as a VNode.
pub fn render_number_input(widget_id: &str, label: &str, value: f64) -> VNode {
    VNode::new(widget_id, "div")
        .with_attr("class", "rustview-widget rustview-number-input")
        .with_child(
            VNode::new(format!("{widget_id}-label"), "label")
                .with_text(label)
                .with_attr("for", format!("{widget_id}-input")),
        )
        .with_child(
            VNode::new(format!("{widget_id}-input"), "input")
                .with_attr("type", "number")
                .with_attr("step", "any")
                .with_attr("value", value.to_string())
                .with_attr("data-widget-id", widget_id)
                .with_attr("data-widget-type", "number_input"),
        )
}

/// Render an integer input widget as a VNode.
pub fn render_int_input(widget_id: &str, label: &str, value: i64) -> VNode {
    VNode::new(widget_id, "div")
        .with_attr("class", "rustview-widget rustview-int-input")
        .with_child(
            VNode::new(format!("{widget_id}-label"), "label")
                .with_text(label)
                .with_attr("for", format!("{widget_id}-input")),
        )
        .with_child(
            VNode::new(format!("{widget_id}-input"), "input")
                .with_attr("type", "number")
                .with_attr("step", "1")
                .with_attr("value", value.to_string())
                .with_attr("data-widget-id", widget_id)
                .with_attr("data-widget-type", "int_input"),
        )
}

/// Render a floating-point slider widget as a VNode.
pub fn render_float_slider(
    widget_id: &str,
    label: &str,
    value: f64,
    min: f64,
    max: f64,
    step: f64,
) -> VNode {
    VNode::new(widget_id, "div")
        .with_attr("class", "rustview-widget rustview-slider")
        .with_child(
            VNode::new(format!("{widget_id}-label"), "label")
                .with_text(format!("{label}: {value}")),
        )
        .with_child(
            VNode::new(format!("{widget_id}-input"), "input")
                .with_attr("type", "range")
                .with_attr("min", min.to_string())
                .with_attr("max", max.to_string())
                .with_attr("step", step.to_string())
                .with_attr("value", value.to_string())
                .with_attr("data-widget-id", widget_id)
                .with_attr("data-widget-type", "float_slider"),
        )
}

/// Render a toggle (switch-style checkbox) widget as a VNode.
pub fn render_toggle(widget_id: &str, label: &str, checked: bool) -> VNode {
    let mut input = VNode::new(format!("{widget_id}-input"), "input")
        .with_attr("type", "checkbox")
        .with_attr("data-widget-id", widget_id)
        .with_attr("data-widget-type", "toggle");
    if checked {
        input = input.with_attr("checked", "true");
    }

    VNode::new(widget_id, "div")
        .with_attr("class", "rustview-widget rustview-toggle")
        .with_child(
            VNode::new(format!("{widget_id}-label-wrapper"), "label")
                .with_text(label)
                .with_child(input)
                .with_child(
                    VNode::new(format!("{widget_id}-track"), "span")
                        .with_attr("class", "rustview-toggle-track"),
                ),
        )
}

/// Render a radio button group widget as a VNode.
pub fn render_radio(widget_id: &str, label: &str, options: &[&str], selected: &str) -> VNode {
    let mut options_container = VNode::new(format!("{widget_id}-options"), "div");

    for (i, &opt) in options.iter().enumerate() {
        let mut radio = VNode::new(format!("{widget_id}-radio-{i}"), "input")
            .with_attr("type", "radio")
            .with_attr("name", widget_id)
            .with_attr("value", opt)
            .with_attr("data-widget-id", widget_id)
            .with_attr("data-widget-type", "radio");
        if opt == selected {
            radio = radio.with_attr("checked", "true");
        }

        options_container = options_container.with_child(
            VNode::new(format!("{widget_id}-opt-{i}"), "label")
                .with_child(radio)
                .with_child(VNode::new(format!("{widget_id}-opt-{i}-text"), "span").with_text(opt)),
        );
    }

    VNode::new(widget_id, "div")
        .with_attr("class", "rustview-widget rustview-radio")
        .with_child(VNode::new(format!("{widget_id}-label"), "label").with_text(label))
        .with_child(options_container)
}

/// Render a single-select dropdown widget as a VNode.
pub fn render_select(widget_id: &str, label: &str, options: &[&str], selected: &str) -> VNode {
    let mut select_el = VNode::new(format!("{widget_id}-select"), "select")
        .with_attr("data-widget-id", widget_id)
        .with_attr("data-widget-type", "select");

    for (i, &opt) in options.iter().enumerate() {
        let mut option = VNode::new(format!("{widget_id}-opt-{i}"), "option")
            .with_attr("value", opt)
            .with_text(opt);
        if opt == selected {
            option = option.with_attr("selected", "true");
        }
        select_el = select_el.with_child(option);
    }

    VNode::new(widget_id, "div")
        .with_attr("class", "rustview-widget rustview-select")
        .with_child(
            VNode::new(format!("{widget_id}-label"), "label")
                .with_text(label)
                .with_attr("for", format!("{widget_id}-select")),
        )
        .with_child(select_el)
}

/// Render a multi-select dropdown widget as a VNode.
pub fn render_multi_select(
    widget_id: &str,
    label: &str,
    options: &[&str],
    selected: &[String],
) -> VNode {
    let mut select_el = VNode::new(format!("{widget_id}-select"), "select")
        .with_attr("multiple", "true")
        .with_attr("data-widget-id", widget_id)
        .with_attr("data-widget-type", "multi_select");

    for (i, &opt) in options.iter().enumerate() {
        let mut option = VNode::new(format!("{widget_id}-opt-{i}"), "option")
            .with_attr("value", opt)
            .with_text(opt);
        if selected.iter().any(|s| s == opt) {
            option = option.with_attr("selected", "true");
        }
        select_el = select_el.with_child(option);
    }

    VNode::new(widget_id, "div")
        .with_attr("class", "rustview-widget rustview-multi-select")
        .with_child(
            VNode::new(format!("{widget_id}-label"), "label")
                .with_text(label)
                .with_attr("for", format!("{widget_id}-select")),
        )
        .with_child(select_el)
}

/// Render a multi-line text area widget as a VNode.
pub fn render_text_area(widget_id: &str, label: &str, value: &str, rows: u32) -> VNode {
    VNode::new(widget_id, "div")
        .with_attr("class", "rustview-widget rustview-text-area")
        .with_child(
            VNode::new(format!("{widget_id}-label"), "label")
                .with_text(label)
                .with_attr("for", format!("{widget_id}-textarea")),
        )
        .with_child(
            VNode::new(format!("{widget_id}-textarea"), "textarea")
                .with_attr("rows", rows.to_string())
                .with_attr("data-widget-id", widget_id)
                .with_attr("data-widget-type", "text_area")
                .with_text(value),
        )
}

/// Render a color picker widget as a VNode.
pub fn render_color_picker(widget_id: &str, label: &str, color: &str) -> VNode {
    VNode::new(widget_id, "div")
        .with_attr("class", "rustview-widget rustview-color-picker")
        .with_child(
            VNode::new(format!("{widget_id}-label"), "label")
                .with_text(label)
                .with_attr("for", format!("{widget_id}-input")),
        )
        .with_child(
            VNode::new(format!("{widget_id}-input"), "input")
                .with_attr("type", "color")
                .with_attr("value", color)
                .with_attr("data-widget-id", widget_id)
                .with_attr("data-widget-type", "color_picker"),
        )
}

/// Render a download button widget as a VNode.
pub fn render_download_button(
    widget_id: &str,
    label: &str,
    data_uri: &str,
    filename: &str,
) -> VNode {
    VNode::new(widget_id, "div")
        .with_attr("class", "rustview-widget rustview-download-button")
        .with_child(
            VNode::new(format!("{widget_id}-link"), "a")
                .with_attr("href", data_uri)
                .with_attr("download", filename)
                .with_child(VNode::new(format!("{widget_id}-btn"), "button").with_text(label)),
        )
}

// ── Output / display widgets ───────────────────────────────────────────

/// Render a heading (h1) widget as a VNode.
pub fn render_heading(widget_id: &str, text: &str) -> VNode {
    VNode::new(widget_id, "div")
        .with_attr("class", "rustview-widget rustview-heading")
        .with_child(VNode::new(format!("{widget_id}-h1"), "h1").with_text(text))
}

/// Render a subheading (h2) widget as a VNode.
pub fn render_subheading(widget_id: &str, text: &str) -> VNode {
    VNode::new(widget_id, "div")
        .with_attr("class", "rustview-widget rustview-subheading")
        .with_child(VNode::new(format!("{widget_id}-h2"), "h2").with_text(text))
}

/// Render a caption (small) widget as a VNode.
pub fn render_caption(widget_id: &str, text: &str) -> VNode {
    VNode::new(widget_id, "div")
        .with_attr("class", "rustview-widget rustview-caption")
        .with_child(VNode::new(format!("{widget_id}-small"), "small").with_text(text))
}

/// Render a code block widget as a VNode.
pub fn render_code(widget_id: &str, source: &str, language: &str) -> VNode {
    VNode::new(widget_id, "div")
        .with_attr("class", "rustview-widget rustview-code")
        .with_child(
            VNode::new(format!("{widget_id}-pre"), "pre").with_child(
                VNode::new(format!("{widget_id}-code"), "code")
                    .with_attr("data-language", language)
                    .with_text(source),
            ),
        )
}

/// Render a JSON display widget as a VNode.
pub fn render_json(widget_id: &str, json_text: &str) -> VNode {
    VNode::new(widget_id, "div")
        .with_attr("class", "rustview-widget rustview-json")
        .with_child(
            VNode::new(format!("{widget_id}-pre"), "pre").with_child(
                VNode::new(format!("{widget_id}-code"), "code")
                    .with_attr("data-language", "json")
                    .with_text(json_text),
            ),
        )
}

/// Render a table widget as a VNode.
pub fn render_table(widget_id: &str, headers: &[&str], rows: &[Vec<String>]) -> VNode {
    let mut thead_tr = VNode::new(format!("{widget_id}-thead-tr"), "tr");
    for (i, &h) in headers.iter().enumerate() {
        thead_tr =
            thead_tr.with_child(VNode::new(format!("{widget_id}-th-{i}"), "th").with_text(h));
    }

    let mut tbody = VNode::new(format!("{widget_id}-tbody"), "tbody");
    for (r, row) in rows.iter().enumerate() {
        let mut tr = VNode::new(format!("{widget_id}-tr-{r}"), "tr");
        for (c, cell) in row.iter().enumerate() {
            tr = tr.with_child(VNode::new(format!("{widget_id}-td-{r}-{c}"), "td").with_text(cell));
        }
        tbody = tbody.with_child(tr);
    }

    VNode::new(widget_id, "div")
        .with_attr("class", "rustview-widget rustview-table")
        .with_child(
            VNode::new(format!("{widget_id}-table"), "table")
                .with_child(VNode::new(format!("{widget_id}-thead"), "thead").with_child(thead_tr))
                .with_child(tbody),
        )
}

/// Render a dataframe widget as a VNode.
///
/// A dataframe is an enhanced table with row numbers, column types,
/// optional title, scrolling, and right-alignment for numeric columns.
pub fn render_dataframe(
    widget_id: &str,
    columns: &[(&str, &str)],
    rows: &[Vec<String>],
    title: Option<&str>,
) -> VNode {
    // Caption / title
    let caption = if let Some(t) = title {
        Some(
            VNode::new(format!("{widget_id}-caption"), "div")
                .with_attr("class", "rustview-dataframe-title")
                .with_text(t),
        )
    } else {
        None
    };

    // Shape label: "N rows × M columns"
    let shape_text = format!("{} rows \u{00D7} {} columns", rows.len(), columns.len());
    let shape_node = VNode::new(format!("{widget_id}-shape"), "div")
        .with_attr("class", "rustview-dataframe-shape")
        .with_text(shape_text);

    // Build header row — row-number header + column headers with type hints
    let mut thead_tr = VNode::new(format!("{widget_id}-thead-tr"), "tr");
    thead_tr = thead_tr.with_child(
        VNode::new(format!("{widget_id}-th-idx"), "th")
            .with_attr("class", "rustview-dataframe-idx")
            .with_text(""),
    );
    for (i, (name, dtype)) in columns.iter().enumerate() {
        let th = VNode::new(format!("{widget_id}-th-{i}"), "th")
            .with_child(
                VNode::new(format!("{widget_id}-th-{i}-name"), "span")
                    .with_attr("class", "rustview-dataframe-col-name")
                    .with_text(*name),
            )
            .with_child(
                VNode::new(format!("{widget_id}-th-{i}-type"), "span")
                    .with_attr("class", "rustview-dataframe-col-type")
                    .with_text(*dtype),
            );
        thead_tr = thead_tr.with_child(th);
    }

    // Build data rows with row-number column
    let mut tbody = VNode::new(format!("{widget_id}-tbody"), "tbody");
    for (r, row) in rows.iter().enumerate() {
        let mut tr = VNode::new(format!("{widget_id}-tr-{r}"), "tr");
        // Row index
        tr = tr.with_child(
            VNode::new(format!("{widget_id}-idx-{r}"), "td")
                .with_attr("class", "rustview-dataframe-idx")
                .with_text(r.to_string()),
        );
        for (c, cell) in row.iter().enumerate() {
            let mut td = VNode::new(format!("{widget_id}-td-{r}-{c}"), "td").with_text(cell);
            // Right-align numeric columns
            if c < columns.len() {
                let dtype = columns[c].1;
                if dtype == "i64" || dtype == "f64" || dtype == "u64" || dtype == "i32" || dtype == "f32" || dtype == "u32" {
                    td = td.with_attr("class", "rustview-dataframe-num");
                }
            }
            tr = tr.with_child(td);
        }
        tbody = tbody.with_child(tr);
    }

    // Assemble
    let table = VNode::new(format!("{widget_id}-table"), "table")
        .with_child(VNode::new(format!("{widget_id}-thead"), "thead").with_child(thead_tr))
        .with_child(tbody);

    let scroll = VNode::new(format!("{widget_id}-scroll"), "div")
        .with_attr("class", "rustview-dataframe-scroll")
        .with_child(table);

    let mut root = VNode::new(widget_id, "div")
        .with_attr("class", "rustview-widget rustview-dataframe");

    if let Some(cap) = caption {
        root = root.with_child(cap);
    }
    root = root.with_child(scroll).with_child(shape_node);
    root
}

/// Render a spinner / loading indicator widget as a VNode.
pub fn render_spinner(widget_id: &str, label: &str) -> VNode {
    VNode::new(widget_id, "div")
        .with_attr("class", "rustview-widget rustview-spinner")
        .with_child(
            VNode::new(format!("{widget_id}-icon"), "span")
                .with_attr("class", "rustview-spinner-icon")
                .with_text("\u{27F3}"),
        )
        .with_child(VNode::new(format!("{widget_id}-label"), "span").with_text(label))
}

/// Render a metric display widget as a VNode.
pub fn render_metric(widget_id: &str, label: &str, value: &str, delta: Option<f64>) -> VNode {
    let mut node = VNode::new(widget_id, "div")
        .with_attr("class", "rustview-widget rustview-metric")
        .with_child(
            VNode::new(format!("{widget_id}-label"), "span")
                .with_attr("class", "rustview-metric-label")
                .with_text(label),
        )
        .with_child(
            VNode::new(format!("{widget_id}-value"), "span")
                .with_attr("class", "rustview-metric-value")
                .with_text(value),
        );

    if let Some(d) = delta {
        let (text, color) = if d >= 0.0 {
            (format!("+{d}"), "green")
        } else {
            (format!("{d}"), "red")
        };
        node = node.with_child(
            VNode::new(format!("{widget_id}-delta"), "span")
                .with_attr("class", "rustview-metric-delta")
                .with_attr("style", format!("color: {color}"))
                .with_text(text),
        );
    }

    node
}

/// Render a styled alert widget as a VNode.
pub fn render_alert(widget_id: &str, message: &str, level: &str) -> VNode {
    let icon = match level {
        "success" => "\u{2713}",
        "info" => "\u{2139}",
        _ => "\u{26A0}", // warning, error
    };

    VNode::new(widget_id, "div")
        .with_attr(
            "class",
            format!("rustview-widget rustview-alert rustview-alert-{level}"),
        )
        .with_child(
            VNode::new(format!("{widget_id}-icon"), "span")
                .with_attr("class", "rustview-alert-icon")
                .with_text(icon),
        )
        .with_child(
            VNode::new(format!("{widget_id}-msg"), "span")
                .with_attr("class", "rustview-alert-message")
                .with_text(message),
        )
}

/// Render a toast notification widget as a VNode.
pub fn render_toast(widget_id: &str, message: &str, level: &str) -> VNode {
    let icon = match level {
        "success" => "\u{2713}",
        "error" => "\u{2717}",
        "info" => "\u{2139}",
        _ => "\u{26A0}", // warning
    };

    VNode::new(widget_id, "div")
        .with_attr(
            "class",
            format!("rustview-toast rustview-toast-{level}"),
        )
        .with_attr("data-widget-type", "toast")
        .with_child(
            VNode::new(format!("{widget_id}-icon"), "span")
                .with_attr("class", "rustview-toast-icon")
                .with_text(icon),
        )
        .with_child(
            VNode::new(format!("{widget_id}-msg"), "span")
                .with_attr("class", "rustview-toast-message")
                .with_text(message),
        )
}

/// Render a horizontal divider widget as a VNode.
pub fn render_divider(widget_id: &str) -> VNode {
    VNode::new(widget_id, "div")
        .with_attr("class", "rustview-widget rustview-divider")
        .with_child(VNode::new(format!("{widget_id}-hr"), "hr"))
}

// ── Layout widgets ─────────────────────────────────────────────────────

/// Render a multi-column layout widget as a VNode.
pub fn render_columns(widget_id: &str, ratios: &[u32]) -> VNode {
    let cols: Vec<String> = ratios.iter().map(|r| format!("{r}fr")).collect();
    let template = cols.join(" ");

    let mut node = VNode::new(widget_id, "div")
        .with_attr("class", "rustview-widget rustview-columns")
        .with_attr(
            "style",
            format!("display:grid; grid-template-columns: {template}; gap: 1rem;"),
        );

    for (i, _) in ratios.iter().enumerate() {
        node = node.with_child(
            VNode::new(format!("{widget_id}-col-{i}"), "div").with_attr("class", "rustview-column"),
        );
    }

    node
}

/// Render a sidebar layout container as a VNode.
pub fn render_sidebar(widget_id: &str) -> VNode {
    VNode::new(widget_id, "div").with_attr("class", "rustview-widget rustview-sidebar")
}

/// Render an expander (collapsible section) widget as a VNode.
pub fn render_expander(widget_id: &str, label: &str, open: bool) -> VNode {
    let mut details = VNode::new(format!("{widget_id}-details"), "details")
        .with_attr("data-widget-id", widget_id)
        .with_attr("data-widget-type", "expander")
        .with_child(VNode::new(format!("{widget_id}-summary"), "summary").with_text(label))
        .with_child(
            VNode::new(format!("{widget_id}-content"), "div")
                .with_attr("class", "rustview-expander-content"),
        );

    if open {
        details = details.with_attr("open", "true");
    }

    VNode::new(widget_id, "div")
        .with_attr("class", "rustview-widget rustview-expander")
        .with_child(details)
}

/// Render a tab bar with a content panel widget as a VNode.
pub fn render_tabs(widget_id: &str, labels: &[&str], active_index: usize) -> VNode {
    let mut tab_bar =
        VNode::new(format!("{widget_id}-bar"), "div").with_attr("class", "rustview-tab-bar");

    for (i, &lbl) in labels.iter().enumerate() {
        let mut btn = VNode::new(format!("{widget_id}-tab-{i}"), "button")
            .with_text(lbl)
            .with_attr("data-widget-id", widget_id)
            .with_attr("data-widget-type", "tab")
            .with_attr("data-tab-index", i.to_string());
        if i == active_index {
            btn = btn.with_attr("class", "active");
        }
        tab_bar = tab_bar.with_child(btn);
    }

    VNode::new(widget_id, "div")
        .with_attr("class", "rustview-widget rustview-tabs")
        .with_child(tab_bar)
        .with_child(
            VNode::new(format!("{widget_id}-panel"), "div")
                .with_attr("class", "rustview-tab-content"),
        )
}

/// Render a generic container widget as a VNode.
pub fn render_container(widget_id: &str) -> VNode {
    VNode::new(widget_id, "div").with_attr("class", "rustview-widget rustview-container")
}

/// Render a date picker widget as a VNode.
pub fn render_date_picker(widget_id: &str, label: &str, value: &str) -> VNode {
    VNode::new(widget_id, "div")
        .with_attr("class", "rustview-widget rustview-date-picker")
        .with_child(
            VNode::new(format!("{widget_id}-label"), "label")
                .with_text(label)
                .with_attr("for", format!("{widget_id}-input")),
        )
        .with_child(
            VNode::new(format!("{widget_id}-input"), "input")
                .with_attr("type", "date")
                .with_attr("value", value)
                .with_attr("data-widget-id", widget_id)
                .with_attr("data-widget-type", "date_picker"),
        )
}

/// Render a file upload widget as a VNode.
pub fn render_file_upload(widget_id: &str, label: &str) -> VNode {
    VNode::new(widget_id, "div")
        .with_attr("class", "rustview-widget rustview-file-upload")
        .with_child(
            VNode::new(format!("{widget_id}-label"), "label")
                .with_text(label)
                .with_attr("for", format!("{widget_id}-input")),
        )
        .with_child(
            VNode::new(format!("{widget_id}-input"), "input")
                .with_attr("type", "file")
                .with_attr("data-widget-id", widget_id)
                .with_attr("data-widget-type", "file_upload"),
        )
}

/// Render an image upload widget with preview as a VNode.
pub fn render_image_upload(widget_id: &str, label: &str, current_data: &str) -> VNode {
    let mut container = VNode::new(widget_id, "div")
        .with_attr("class", "rustview-widget rustview-image-upload")
        .with_child(
            VNode::new(format!("{widget_id}-label"), "label")
                .with_text(label)
                .with_attr("for", format!("{widget_id}-input")),
        )
        .with_child(
            VNode::new(format!("{widget_id}-input"), "input")
                .with_attr("type", "file")
                .with_attr("accept", "image/*")
                .with_attr("data-widget-id", widget_id)
                .with_attr("data-widget-type", "image_upload"),
        );

    // Show preview if image data exists
    if !current_data.is_empty() {
        container = container.with_child(
            VNode::new(format!("{widget_id}-preview"), "img")
                .with_attr("src", current_data)
                .with_attr("alt", "Preview")
                .with_attr("class", "rustview-image-upload-preview"),
        );
    }

    container
}

/// Render an image display widget as a VNode.
pub fn render_image(widget_id: &str, src: &str, caption: &str) -> VNode {
    let mut container = VNode::new(widget_id, "div")
        .with_attr("class", "rustview-widget rustview-image")
        .with_child(
            VNode::new(format!("{widget_id}-img"), "img")
                .with_attr("src", src)
                .with_attr("alt", caption),
        );
    if !caption.is_empty() {
        container = container.with_child(
            VNode::new(format!("{widget_id}-caption"), "small").with_text(caption),
        );
    }
    container
}

/// Render an audio player widget as a VNode.
pub fn render_audio(widget_id: &str, src: &str, format: &str) -> VNode {
    let mime = match format {
        "mp3" => "audio/mpeg",
        "wav" => "audio/wav",
        "ogg" => "audio/ogg",
        _ => "audio/mpeg",
    };
    VNode::new(widget_id, "div")
        .with_attr("class", "rustview-widget rustview-audio")
        .with_child(
            VNode::new(format!("{widget_id}-player"), "audio")
                .with_attr("controls", "true")
                .with_child(
                    VNode::new(format!("{widget_id}-source"), "source")
                        .with_attr("src", src)
                        .with_attr("type", mime),
                ),
        )
}

/// Render a video player widget as a VNode.
pub fn render_video(widget_id: &str, src: &str, format: &str) -> VNode {
    let mime = match format {
        "mp4" => "video/mp4",
        "webm" => "video/webm",
        "ogg" => "video/ogg",
        _ => "video/mp4",
    };
    VNode::new(widget_id, "div")
        .with_attr("class", "rustview-widget rustview-video")
        .with_child(
            VNode::new(format!("{widget_id}-player"), "video")
                .with_attr("controls", "true")
                .with_child(
                    VNode::new(format!("{widget_id}-source"), "source")
                        .with_attr("src", src)
                        .with_attr("type", mime),
                ),
        )
}

/// Render a row layout (horizontal flex) as a VNode.
pub fn render_row(widget_id: &str) -> VNode {
    VNode::new(widget_id, "div").with_attr("class", "rustview-widget rustview-row")
}

/// Render an empty placeholder slot as a VNode.
pub fn render_empty(widget_id: &str) -> VNode {
    VNode::new(widget_id, "div").with_attr("class", "rustview-widget rustview-empty")
}

/// Render a modal dialog overlay as a VNode.
pub fn render_modal(widget_id: &str, title: &str, open: bool) -> VNode {
    let mut overlay = VNode::new(widget_id, "div")
        .with_attr("class", "rustview-widget rustview-modal-overlay")
        .with_attr("data-widget-id", widget_id)
        .with_attr("data-widget-type", "modal");

    if !open {
        overlay = overlay.with_attr("style", "display:none");
    }

    let dialog = VNode::new(format!("{widget_id}-dialog"), "div")
        .with_attr("class", "rustview-modal-dialog")
        .with_child(
            VNode::new(format!("{widget_id}-header"), "div")
                .with_attr("class", "rustview-modal-header")
                .with_child(
                    VNode::new(format!("{widget_id}-title"), "h3").with_text(title),
                )
                .with_child(
                    VNode::new(format!("{widget_id}-close"), "button")
                        .with_text("✕")
                        .with_attr("class", "rustview-modal-close")
                        .with_attr("data-widget-id", widget_id)
                        .with_attr("data-widget-type", "modal_close"),
                ),
        )
        .with_child(
            VNode::new(format!("{widget_id}-body"), "div")
                .with_attr("class", "rustview-modal-body"),
        );

    overlay.with_child(dialog)
}

/// Render a form container as a VNode.
pub fn render_form(widget_id: &str) -> VNode {
    VNode::new(widget_id, "div")
        .with_attr("class", "rustview-widget rustview-form")
        .with_attr("data-widget-id", widget_id)
        .with_attr("data-widget-type", "form")
}

/// Render a form submit button as a VNode.
pub fn render_form_submit_button(widget_id: &str, label: &str) -> VNode {
    VNode::new(widget_id, "div")
        .with_attr("class", "rustview-widget rustview-form-submit")
        .with_child(
            VNode::new(format!("{widget_id}-btn"), "button")
                .with_text(label)
                .with_attr("data-widget-id", widget_id)
                .with_attr("data-widget-type", "form_submit"),
        )
}

/// Render a hyperlink widget as a VNode.
pub fn render_link(widget_id: &str, text: &str, url: &str) -> VNode {
    VNode::new(widget_id, "div")
        .with_attr("class", "rustview-widget rustview-link")
        .with_child(
            VNode::new(format!("{widget_id}-a"), "a")
                .with_text(text)
                .with_attr("href", url)
                .with_attr("target", "_blank")
                .with_attr("rel", "noopener noreferrer"),
        )
}

/// Render a badge/label widget as a VNode.
///
/// `color` supports predefined names: "red", "green", "blue", "yellow", "gray", "purple", "orange"
/// or any valid CSS color value.
pub fn render_badge(widget_id: &str, text: &str, color: &str) -> VNode {
    let (bg, fg) = match color {
        "red" => ("#3a1a1a", "#ff6b6b"),
        "green" => ("#1a3a2a", "#50fa7b"),
        "blue" => ("#1a2a3a", "#64b5f6"),
        "yellow" => ("#3a2e1a", "#ffb74d"),
        "gray" => ("#2a2a2e", "#a3a8b8"),
        "purple" => ("#2a1a3a", "#b39ddb"),
        "orange" => ("#3a2a1a", "#ff8a65"),
        _ => ("#2a2a2e", color),
    };
    VNode::new(widget_id, "span")
        .with_attr("class", "rustview-widget rustview-badge")
        .with_attr("style", format!("background:{bg};color:{fg};padding:0.25rem 0.625rem;border-radius:1rem;font-size:0.8125rem;font-weight:500;display:inline-block"))
        .with_text(text)
}

/// Render an inline SVG line chart as a VNode.
pub fn render_line_chart(widget_id: &str, title: &str, data: &[(f64, f64)]) -> VNode {
    let svg = generate_line_chart_svg(widget_id, data);
    let mut node =
        VNode::new(widget_id, "div").with_attr("class", "rustview-widget rustview-chart");
    if !title.is_empty() {
        node = node.with_child(
            VNode::new(format!("{widget_id}-title"), "div")
                .with_attr("class", "rustview-chart-title")
                .with_text(title),
        );
    }
    node.with_child(
        VNode::new(format!("{widget_id}-svg"), "div")
            .with_attr("class", "rustview-chart-svg")
            .with_attr("data-innerHTML", &svg),
    )
}

/// Render an inline SVG bar chart as a VNode.
pub fn render_bar_chart(widget_id: &str, title: &str, data: &[(&str, f64)]) -> VNode {
    let svg = generate_bar_chart_svg(widget_id, data);
    let mut node =
        VNode::new(widget_id, "div").with_attr("class", "rustview-widget rustview-chart");
    if !title.is_empty() {
        node = node.with_child(
            VNode::new(format!("{widget_id}-title"), "div")
                .with_attr("class", "rustview-chart-title")
                .with_text(title),
        );
    }
    node.with_child(
        VNode::new(format!("{widget_id}-svg"), "div")
            .with_attr("class", "rustview-chart-svg")
            .with_attr("data-innerHTML", &svg),
    )
}

/// Render an inline SVG scatter chart as a VNode.
pub fn render_scatter_chart(widget_id: &str, title: &str, data: &[(f64, f64)]) -> VNode {
    let svg = generate_scatter_chart_svg(widget_id, data);
    let mut node =
        VNode::new(widget_id, "div").with_attr("class", "rustview-widget rustview-chart");
    if !title.is_empty() {
        node = node.with_child(
            VNode::new(format!("{widget_id}-title"), "div")
                .with_attr("class", "rustview-chart-title")
                .with_text(title),
        );
    }
    node.with_child(
        VNode::new(format!("{widget_id}-svg"), "div")
            .with_attr("class", "rustview-chart-svg")
            .with_attr("data-innerHTML", &svg),
    )
}

/// Render an inline SVG histogram as a VNode.
pub fn render_histogram(widget_id: &str, title: &str, data: &[f64], bins: u32) -> VNode {
    let svg = generate_histogram_svg(widget_id, data, bins);
    let mut node =
        VNode::new(widget_id, "div").with_attr("class", "rustview-widget rustview-chart");
    if !title.is_empty() {
        node = node.with_child(
            VNode::new(format!("{widget_id}-title"), "div")
                .with_attr("class", "rustview-chart-title")
                .with_text(title),
        );
    }
    node.with_child(
        VNode::new(format!("{widget_id}-svg"), "div")
            .with_attr("class", "rustview-chart-svg")
            .with_attr("data-innerHTML", &svg),
    )
}

// --- SVG Chart Generators (inline, no external deps) ---

/// Default chart SVG viewBox width in logical pixels.
const CHART_WIDTH: f64 = 600.0;
/// Default chart SVG viewBox height in logical pixels.
const CHART_HEIGHT: f64 = 300.0;
/// Padding around the chart plot area for axes and labels.
const CHART_PADDING: f64 = 50.0;

fn generate_line_chart_svg(_widget_id: &str, data: &[(f64, f64)]) -> String {
    if data.is_empty() {
        return format!(
            "<svg viewBox=\"0 0 {CHART_WIDTH} {CHART_HEIGHT}\" xmlns=\"http://www.w3.org/2000/svg\">\
             <text x=\"{}\" y=\"{}\" fill=\"#a3a8b8\" text-anchor=\"middle\" font-size=\"14\">No data</text>\
             </svg>",
            CHART_WIDTH / 2.0,
            CHART_HEIGHT / 2.0
        );
    }

    let (min_x, max_x, min_y, max_y) = data_bounds(data);
    let plot_w = CHART_WIDTH - 2.0 * CHART_PADDING;
    let plot_h = CHART_HEIGHT - 2.0 * CHART_PADDING;

    let points: Vec<String> = data
        .iter()
        .map(|&(x, y)| {
            let px = CHART_PADDING + scale(x, min_x, max_x, plot_w);
            let py = CHART_PADDING + plot_h - scale(y, min_y, max_y, plot_h);
            format!("{:.1},{:.1}", px, py)
        })
        .collect();

    let polyline_points = points.join(" ");
    let axes = render_axes(min_x, max_x, min_y, max_y);

    format!(
        "<svg viewBox=\"0 0 {CHART_WIDTH} {CHART_HEIGHT}\" xmlns=\"http://www.w3.org/2000/svg\">\
         {axes}\
         <polyline points=\"{polyline_points}\" fill=\"none\" stroke=\"#ff4b4b\" stroke-width=\"2\" />\
         </svg>"
    )
}

fn generate_bar_chart_svg(_widget_id: &str, data: &[(&str, f64)]) -> String {
    if data.is_empty() {
        return format!(
            "<svg viewBox=\"0 0 {CHART_WIDTH} {CHART_HEIGHT}\" xmlns=\"http://www.w3.org/2000/svg\">\
             <text x=\"{}\" y=\"{}\" fill=\"#a3a8b8\" text-anchor=\"middle\" font-size=\"14\">No data</text>\
             </svg>",
            CHART_WIDTH / 2.0,
            CHART_HEIGHT / 2.0
        );
    }

    let max_val = data
        .iter()
        .map(|(_, v)| *v)
        .fold(f64::NEG_INFINITY, f64::max);
    let max_val = if max_val <= 0.0 { 1.0 } else { max_val };

    let plot_w = CHART_WIDTH - 2.0 * CHART_PADDING;
    let plot_h = CHART_HEIGHT - 2.0 * CHART_PADDING;
    let bar_count = data.len() as f64;
    let bar_gap = 4.0;
    let bar_w = (plot_w - (bar_count - 1.0) * bar_gap) / bar_count;

    let mut bars = String::new();
    for (i, (label, val)) in data.iter().enumerate() {
        let x = CHART_PADDING + i as f64 * (bar_w + bar_gap);
        let h = (val / max_val) * plot_h;
        let y = CHART_PADDING + plot_h - h;
        bars.push_str(&format!(
            "<rect x=\"{x:.1}\" y=\"{y:.1}\" width=\"{bar_w:.1}\" height=\"{h:.1}\" fill=\"#ff4b4b\" rx=\"2\" />"
        ));
        let label_x = x + bar_w / 2.0;
        let label_y = CHART_PADDING + plot_h + 16.0;
        bars.push_str(&format!(
            "<text x=\"{label_x:.1}\" y=\"{label_y:.1}\" fill=\"#a3a8b8\" text-anchor=\"middle\" font-size=\"11\">{label}</text>"
        ));
    }

    // Y-axis line
    let y_axis = format!(
        "<line x1=\"{p}\" y1=\"{p}\" x2=\"{p}\" y2=\"{b}\" stroke=\"#4a4a5a\" stroke-width=\"1\" />",
        p = CHART_PADDING,
        b = CHART_PADDING + plot_h
    );

    format!(
        "<svg viewBox=\"0 0 {CHART_WIDTH} {CHART_HEIGHT}\" xmlns=\"http://www.w3.org/2000/svg\">\
         {y_axis}\
         {bars}\
         </svg>"
    )
}

fn generate_scatter_chart_svg(_widget_id: &str, data: &[(f64, f64)]) -> String {
    if data.is_empty() {
        return format!(
            "<svg viewBox=\"0 0 {CHART_WIDTH} {CHART_HEIGHT}\" xmlns=\"http://www.w3.org/2000/svg\">\
             <text x=\"{}\" y=\"{}\" fill=\"#a3a8b8\" text-anchor=\"middle\" font-size=\"14\">No data</text>\
             </svg>",
            CHART_WIDTH / 2.0,
            CHART_HEIGHT / 2.0
        );
    }

    let (min_x, max_x, min_y, max_y) = data_bounds(data);
    let plot_w = CHART_WIDTH - 2.0 * CHART_PADDING;
    let plot_h = CHART_HEIGHT - 2.0 * CHART_PADDING;
    let axes = render_axes(min_x, max_x, min_y, max_y);

    let mut dots = String::new();
    for &(x, y) in data {
        let cx = CHART_PADDING + scale(x, min_x, max_x, plot_w);
        let cy = CHART_PADDING + plot_h - scale(y, min_y, max_y, plot_h);
        dots.push_str(&format!(
            "<circle cx=\"{cx:.1}\" cy=\"{cy:.1}\" r=\"4\" fill=\"#ff4b4b\" opacity=\"0.8\" />"
        ));
    }

    format!(
        "<svg viewBox=\"0 0 {CHART_WIDTH} {CHART_HEIGHT}\" xmlns=\"http://www.w3.org/2000/svg\">\
         {axes}\
         {dots}\
         </svg>"
    )
}

fn generate_histogram_svg(_widget_id: &str, data: &[f64], bins: u32) -> String {
    if data.is_empty() || bins == 0 {
        return format!(
            "<svg viewBox=\"0 0 {CHART_WIDTH} {CHART_HEIGHT}\" xmlns=\"http://www.w3.org/2000/svg\">\
             <text x=\"{}\" y=\"{}\" fill=\"#a3a8b8\" text-anchor=\"middle\" font-size=\"14\">No data</text>\
             </svg>",
            CHART_WIDTH / 2.0,
            CHART_HEIGHT / 2.0
        );
    }

    let min_val = data.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_val = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let range = if (max_val - min_val).abs() < f64::EPSILON {
        1.0
    } else {
        max_val - min_val
    };
    let bin_width = range / bins as f64;

    let mut counts = vec![0u32; bins as usize];
    for &val in data {
        let idx = ((val - min_val) / bin_width).floor() as usize;
        let idx = idx.min(counts.len() - 1);
        counts[idx] += 1;
    }

    let max_count = *counts.iter().max().unwrap_or(&1) as f64;
    let max_count = if max_count == 0.0 { 1.0 } else { max_count };

    let plot_w = CHART_WIDTH - 2.0 * CHART_PADDING;
    let plot_h = CHART_HEIGHT - 2.0 * CHART_PADDING;
    let bar_w = plot_w / bins as f64;

    let mut bars = String::new();
    for (i, &count) in counts.iter().enumerate() {
        let x = CHART_PADDING + i as f64 * bar_w;
        let h = (count as f64 / max_count) * plot_h;
        let y = CHART_PADDING + plot_h - h;
        bars.push_str(&format!(
            "<rect x=\"{x:.1}\" y=\"{y:.1}\" width=\"{w:.1}\" height=\"{h:.1}\" fill=\"#ff4b4b\" stroke=\"#0e1117\" stroke-width=\"1\" />",
            w = bar_w
        ));
    }

    // X-axis labels
    let mut labels = String::new();
    for i in 0..=bins {
        let val = min_val + i as f64 * bin_width;
        let x = CHART_PADDING + i as f64 * bar_w;
        let y = CHART_PADDING + plot_h + 16.0;
        labels.push_str(&format!(
            "<text x=\"{x:.1}\" y=\"{y:.1}\" fill=\"#a3a8b8\" text-anchor=\"middle\" font-size=\"10\">{val:.1}</text>"
        ));
    }

    let axes = format!(
        "<line x1=\"{p}\" y1=\"{p}\" x2=\"{p}\" y2=\"{b}\" stroke=\"#4a4a5a\" stroke-width=\"1\" />\
         <line x1=\"{p}\" y1=\"{b}\" x2=\"{r}\" y2=\"{b}\" stroke=\"#4a4a5a\" stroke-width=\"1\" />",
        p = CHART_PADDING,
        b = CHART_PADDING + plot_h,
        r = CHART_PADDING + plot_w
    );

    format!(
        "<svg viewBox=\"0 0 {CHART_WIDTH} {CHART_HEIGHT}\" xmlns=\"http://www.w3.org/2000/svg\">\
         {axes}\
         {bars}\
         {labels}\
         </svg>"
    )
}

fn data_bounds(data: &[(f64, f64)]) -> (f64, f64, f64, f64) {
    let min_x = data.iter().map(|d| d.0).fold(f64::INFINITY, f64::min);
    let max_x = data.iter().map(|d| d.0).fold(f64::NEG_INFINITY, f64::max);
    let min_y = data.iter().map(|d| d.1).fold(f64::INFINITY, f64::min);
    let max_y = data.iter().map(|d| d.1).fold(f64::NEG_INFINITY, f64::max);
    // Prevent zero-range
    let max_x = if (max_x - min_x).abs() < f64::EPSILON {
        min_x + 1.0
    } else {
        max_x
    };
    let max_y = if (max_y - min_y).abs() < f64::EPSILON {
        min_y + 1.0
    } else {
        max_y
    };
    (min_x, max_x, min_y, max_y)
}

fn scale(val: f64, min: f64, max: f64, size: f64) -> f64 {
    ((val - min) / (max - min)) * size
}

fn render_axes(min_x: f64, max_x: f64, min_y: f64, max_y: f64) -> String {
    let plot_w = CHART_WIDTH - 2.0 * CHART_PADDING;
    let plot_h = CHART_HEIGHT - 2.0 * CHART_PADDING;
    let bottom = CHART_PADDING + plot_h;
    let right = CHART_PADDING + plot_w;

    // Axes lines
    let mut svg = format!(
        "<line x1=\"{p}\" y1=\"{p}\" x2=\"{p}\" y2=\"{b}\" stroke=\"#4a4a5a\" stroke-width=\"1\" />\
         <line x1=\"{p}\" y1=\"{b}\" x2=\"{r}\" y2=\"{b}\" stroke=\"#4a4a5a\" stroke-width=\"1\" />",
        p = CHART_PADDING,
        b = bottom,
        r = right
    );

    // X-axis labels (5 ticks)
    for i in 0..=4 {
        let frac = i as f64 / 4.0;
        let val = min_x + frac * (max_x - min_x);
        let x = CHART_PADDING + frac * plot_w;
        svg.push_str(&format!(
            "<text x=\"{x:.1}\" y=\"{y:.1}\" fill=\"#a3a8b8\" text-anchor=\"middle\" font-size=\"10\">{val:.1}</text>",
            y = bottom + 16.0
        ));
    }

    // Y-axis labels (5 ticks)
    for i in 0..=4 {
        let frac = i as f64 / 4.0;
        let val = min_y + frac * (max_y - min_y);
        let y = bottom - frac * plot_h;
        svg.push_str(&format!(
            "<text x=\"{x:.1}\" y=\"{y:.1}\" fill=\"#a3a8b8\" text-anchor=\"end\" font-size=\"10\">{val:.1}</text>",
            x = CHART_PADDING - 6.0
        ));
    }

    svg
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_text_input() {
        let node = render_text_input("name-input", "Your name", "World");
        assert_eq!(node.tag, "div");
        assert_eq!(node.children.len(), 2);
        assert_eq!(node.children[0].text.as_deref(), Some("Your name"));
        assert_eq!(node.children[1].attrs.get("value").unwrap(), "World");
    }

    #[test]
    fn test_render_slider() {
        let node = render_slider("slider1", "Count", 50, 0, 100);
        assert_eq!(node.children.len(), 2);
        assert_eq!(node.children[0].text.as_deref(), Some("Count: 50"));
        assert_eq!(node.children[1].attrs.get("min").unwrap(), "0");
        assert_eq!(node.children[1].attrs.get("max").unwrap(), "100");
    }

    #[test]
    fn test_render_checkbox_checked() {
        let node = render_checkbox("cb1", "Enable", true);
        let label_wrapper = &node.children[0];
        let input = &label_wrapper.children[0];
        assert_eq!(input.attrs.get("checked").unwrap(), "true");
    }

    #[test]
    fn test_render_checkbox_unchecked() {
        let node = render_checkbox("cb1", "Enable", false);
        let label_wrapper = &node.children[0];
        let input = &label_wrapper.children[0];
        assert!(!input.attrs.contains_key("checked"));
    }

    #[test]
    fn test_render_button() {
        let node = render_button("btn1", "Click me");
        let btn = &node.children[0];
        assert_eq!(btn.text.as_deref(), Some("Click me"));
        assert_eq!(btn.attrs.get("data-widget-type").unwrap(), "button");
    }

    #[test]
    fn test_render_write() {
        let node = render_write("w1", "Hello, World!");
        assert_eq!(node.children[0].text.as_deref(), Some("Hello, World!"));
    }

    #[test]
    fn test_render_progress() {
        let node = render_progress("p1", 0.75);
        let label = &node.children[1];
        assert_eq!(label.text.as_deref(), Some("75%"));
    }

    #[test]
    fn test_render_progress_clamped() {
        let node = render_progress("p1", 1.5);
        let label = &node.children[1];
        assert_eq!(label.text.as_deref(), Some("100%"));
    }

    #[test]
    fn test_render_error() {
        let node = render_error("e1", "Something went wrong");
        assert!(node.attrs.get("class").unwrap().contains("error"));
        let msg = &node.children[1];
        assert_eq!(msg.text.as_deref(), Some("Something went wrong"));
    }

    // ── New widget tests ───────────────────────────────────────────────

    #[test]
    fn test_render_number_input() {
        let node = render_number_input("ni1", "Price", 9.99);
        assert_eq!(node.tag, "div");
        assert!(node
            .attrs
            .get("class")
            .unwrap()
            .contains("rustview-number-input"));
        assert_eq!(node.children.len(), 2);
        assert_eq!(node.children[0].text.as_deref(), Some("Price"));
        assert_eq!(node.children[1].attrs.get("value").unwrap(), "9.99");
        assert_eq!(node.children[1].attrs.get("step").unwrap(), "any");
        assert_eq!(
            node.children[1].attrs.get("data-widget-type").unwrap(),
            "number_input"
        );
    }

    #[test]
    fn test_render_int_input() {
        let node = render_int_input("ii1", "Count", 42);
        assert_eq!(node.tag, "div");
        assert!(node
            .attrs
            .get("class")
            .unwrap()
            .contains("rustview-int-input"));
        assert_eq!(node.children.len(), 2);
        assert_eq!(node.children[0].text.as_deref(), Some("Count"));
        assert_eq!(node.children[1].attrs.get("value").unwrap(), "42");
        assert_eq!(node.children[1].attrs.get("step").unwrap(), "1");
        assert_eq!(
            node.children[1].attrs.get("data-widget-type").unwrap(),
            "int_input"
        );
    }

    #[test]
    fn test_render_float_slider() {
        let node = render_float_slider("fs1", "Opacity", 0.5, 0.0, 1.0, 0.01);
        assert_eq!(node.tag, "div");
        assert!(node.attrs.get("class").unwrap().contains("rustview-slider"));
        assert_eq!(node.children.len(), 2);
        assert_eq!(node.children[0].text.as_deref(), Some("Opacity: 0.5"));
        let input = &node.children[1];
        assert_eq!(input.attrs.get("min").unwrap(), "0");
        assert_eq!(input.attrs.get("max").unwrap(), "1");
        assert_eq!(input.attrs.get("step").unwrap(), "0.01");
        assert_eq!(input.attrs.get("value").unwrap(), "0.5");
        assert_eq!(input.attrs.get("data-widget-type").unwrap(), "float_slider");
    }

    #[test]
    fn test_render_toggle_checked() {
        let node = render_toggle("tg1", "Dark mode", true);
        assert!(node.attrs.get("class").unwrap().contains("rustview-toggle"));
        let label_wrapper = &node.children[0];
        let input = &label_wrapper.children[0];
        assert_eq!(input.attrs.get("checked").unwrap(), "true");
        assert_eq!(input.attrs.get("data-widget-type").unwrap(), "toggle");
        let track = &label_wrapper.children[1];
        assert_eq!(track.attrs.get("class").unwrap(), "rustview-toggle-track");
    }

    #[test]
    fn test_render_toggle_unchecked() {
        let node = render_toggle("tg1", "Dark mode", false);
        let label_wrapper = &node.children[0];
        let input = &label_wrapper.children[0];
        assert!(!input.attrs.contains_key("checked"));
    }

    #[test]
    fn test_render_radio() {
        let node = render_radio("r1", "Size", &["S", "M", "L"], "M");
        assert!(node.attrs.get("class").unwrap().contains("rustview-radio"));
        assert_eq!(node.children.len(), 2); // label + options container
        let options = &node.children[1];
        assert_eq!(options.children.len(), 3);
        // Second option ("M") should be checked
        let m_input = &options.children[1].children[0];
        assert_eq!(m_input.attrs.get("checked").unwrap(), "true");
        assert_eq!(m_input.attrs.get("data-widget-type").unwrap(), "radio");
        assert_eq!(m_input.attrs.get("name").unwrap(), "r1");
        // First option should not be checked
        let s_input = &options.children[0].children[0];
        assert!(!s_input.attrs.contains_key("checked"));
    }

    #[test]
    fn test_render_select() {
        let node = render_select("sel1", "Country", &["US", "UK", "DE"], "UK");
        assert!(node.attrs.get("class").unwrap().contains("rustview-select"));
        assert_eq!(node.children.len(), 2);
        let select = &node.children[1];
        assert_eq!(select.tag, "select");
        assert_eq!(select.attrs.get("data-widget-type").unwrap(), "select");
        assert_eq!(select.children.len(), 3);
        // UK should be selected
        assert_eq!(select.children[1].attrs.get("selected").unwrap(), "true");
        assert!(!select.children[0].attrs.contains_key("selected"));
    }

    #[test]
    fn test_render_multi_select() {
        let selected = vec!["A".to_string(), "C".to_string()];
        let node = render_multi_select("ms1", "Tags", &["A", "B", "C"], &selected);
        assert!(node
            .attrs
            .get("class")
            .unwrap()
            .contains("rustview-multi-select"));
        let select = &node.children[1];
        assert_eq!(select.attrs.get("multiple").unwrap(), "true");
        assert_eq!(
            select.attrs.get("data-widget-type").unwrap(),
            "multi_select"
        );
        assert_eq!(select.children.len(), 3);
        assert!(select.children[0].attrs.contains_key("selected")); // A
        assert!(!select.children[1].attrs.contains_key("selected")); // B
        assert!(select.children[2].attrs.contains_key("selected")); // C
    }

    #[test]
    fn test_render_text_area() {
        let node = render_text_area("ta1", "Bio", "Hello world", 5);
        assert!(node
            .attrs
            .get("class")
            .unwrap()
            .contains("rustview-text-area"));
        assert_eq!(node.children.len(), 2);
        let textarea = &node.children[1];
        assert_eq!(textarea.tag, "textarea");
        assert_eq!(textarea.attrs.get("rows").unwrap(), "5");
        assert_eq!(textarea.attrs.get("data-widget-type").unwrap(), "text_area");
        assert_eq!(textarea.text.as_deref(), Some("Hello world"));
    }

    #[test]
    fn test_render_color_picker() {
        let node = render_color_picker("cp1", "Background", "#ff0000");
        assert!(node
            .attrs
            .get("class")
            .unwrap()
            .contains("rustview-color-picker"));
        assert_eq!(node.children.len(), 2);
        let input = &node.children[1];
        assert_eq!(input.attrs.get("type").unwrap(), "color");
        assert_eq!(input.attrs.get("value").unwrap(), "#ff0000");
        assert_eq!(input.attrs.get("data-widget-type").unwrap(), "color_picker");
    }

    #[test]
    fn test_render_download_button() {
        let node = render_download_button("dl1", "Download CSV", "data:text/csv,a,b", "report.csv");
        assert!(node
            .attrs
            .get("class")
            .unwrap()
            .contains("rustview-download-button"));
        let link = &node.children[0];
        assert_eq!(link.tag, "a");
        assert_eq!(link.attrs.get("href").unwrap(), "data:text/csv,a,b");
        assert_eq!(link.attrs.get("download").unwrap(), "report.csv");
        let btn = &link.children[0];
        assert_eq!(btn.tag, "button");
        assert_eq!(btn.text.as_deref(), Some("Download CSV"));
    }

    #[test]
    fn test_render_heading() {
        let node = render_heading("h1", "Welcome");
        assert!(node.attrs.get("class").unwrap().contains("rustview-heading"));
        assert_eq!(node.children[0].tag, "h1");
        assert_eq!(node.children[0].text.as_deref(), Some("Welcome"));
    }

    #[test]
    fn test_render_subheading() {
        let node = render_subheading("sh1", "Details");
        assert!(node
            .attrs
            .get("class")
            .unwrap()
            .contains("rustview-subheading"));
        assert_eq!(node.children[0].tag, "h2");
        assert_eq!(node.children[0].text.as_deref(), Some("Details"));
    }

    #[test]
    fn test_render_caption() {
        let node = render_caption("cap1", "A small note");
        assert!(node.attrs.get("class").unwrap().contains("rustview-caption"));
        assert_eq!(node.children[0].tag, "small");
        assert_eq!(node.children[0].text.as_deref(), Some("A small note"));
    }

    #[test]
    fn test_render_code() {
        let node = render_code("c1", "fn main() {}", "rust");
        assert!(node.attrs.get("class").unwrap().contains("rustview-code"));
        let pre = &node.children[0];
        assert_eq!(pre.tag, "pre");
        let code = &pre.children[0];
        assert_eq!(code.tag, "code");
        assert_eq!(code.attrs.get("data-language").unwrap(), "rust");
        assert_eq!(code.text.as_deref(), Some("fn main() {}"));
    }

    #[test]
    fn test_render_json() {
        let node = render_json("j1", r#"{"key": "value"}"#);
        assert!(node.attrs.get("class").unwrap().contains("rustview-json"));
        let code = &node.children[0].children[0];
        assert_eq!(code.attrs.get("data-language").unwrap(), "json");
        assert_eq!(code.text.as_deref(), Some(r#"{"key": "value"}"#));
    }

    #[test]
    fn test_render_table() {
        let rows = vec![
            vec!["Alice".into(), "30".into()],
            vec!["Bob".into(), "25".into()],
        ];
        let node = render_table("t1", &["Name", "Age"], &rows);
        assert!(node.attrs.get("class").unwrap().contains("rustview-table"));
        let table = &node.children[0];
        assert_eq!(table.tag, "table");
        let thead = &table.children[0];
        let tbody = &table.children[1];
        assert_eq!(thead.children[0].children.len(), 2); // 2 headers
        assert_eq!(thead.children[0].children[0].text.as_deref(), Some("Name"));
        assert_eq!(tbody.children.len(), 2); // 2 rows
        assert_eq!(tbody.children[0].children[0].text.as_deref(), Some("Alice"));
    }

    #[test]
    fn test_render_dataframe_basic() {
        let columns = vec![("Name", "str"), ("Age", "i64")];
        let rows = vec![
            vec!["Alice".into(), "30".into()],
            vec!["Bob".into(), "25".into()],
        ];
        let node = render_dataframe("df1", &columns, &rows, Some("Users"));
        assert!(node
            .attrs
            .get("class")
            .unwrap()
            .contains("rustview-dataframe"));
        // Title is first child
        let title = &node.children[0];
        assert!(title
            .attrs
            .get("class")
            .unwrap()
            .contains("rustview-dataframe-title"));
        assert_eq!(title.text.as_deref(), Some("Users"));
        // Scroll container is second child
        let scroll = &node.children[1];
        assert!(scroll
            .attrs
            .get("class")
            .unwrap()
            .contains("rustview-dataframe-scroll"));
        // Table inside scroll
        let table = &scroll.children[0];
        assert_eq!(table.tag, "table");
        let thead = &table.children[0];
        let tbody = &table.children[1];
        // Headers: idx + 2 columns = 3
        assert_eq!(thead.children[0].children.len(), 3);
        // Row count
        assert_eq!(tbody.children.len(), 2);
        // Shape label is third child
        let shape = &node.children[2];
        assert!(shape
            .attrs
            .get("class")
            .unwrap()
            .contains("rustview-dataframe-shape"));
        assert_eq!(
            shape.text.as_deref(),
            Some("2 rows \u{00D7} 2 columns")
        );
    }

    #[test]
    fn test_render_dataframe_no_title() {
        let columns = vec![("X", "f64")];
        let rows = vec![vec!["1.5".into()]];
        let node = render_dataframe("df2", &columns, &rows, None);
        assert!(node
            .attrs
            .get("class")
            .unwrap()
            .contains("rustview-dataframe"));
        // No title — first child is scroll container
        let scroll = &node.children[0];
        assert!(scroll
            .attrs
            .get("class")
            .unwrap()
            .contains("rustview-dataframe-scroll"));
        // Shape is second child
        let shape = &node.children[1];
        assert_eq!(
            shape.text.as_deref(),
            Some("1 rows \u{00D7} 1 columns")
        );
    }

    #[test]
    fn test_render_dataframe_numeric_alignment() {
        let columns = vec![("Name", "str"), ("Score", "f64")];
        let rows = vec![vec!["Alice".into(), "95.5".into()]];
        let node = render_dataframe("df3", &columns, &rows, None);
        let scroll = &node.children[0];
        let table = &scroll.children[0];
        let tbody = &table.children[1];
        let row = &tbody.children[0];
        // row.children: [idx, Name, Score]
        let score_td = &row.children[2];
        assert_eq!(score_td.attrs.get("class").unwrap(), "rustview-dataframe-num");
    }

    #[test]
    fn test_render_dataframe_empty() {
        let columns: Vec<(&str, &str)> = vec![("A", "str"), ("B", "i64")];
        let rows: Vec<Vec<String>> = vec![];
        let node = render_dataframe("df4", &columns, &rows, Some("Empty"));
        let shape = &node.children[2];
        assert_eq!(
            shape.text.as_deref(),
            Some("0 rows \u{00D7} 2 columns")
        );
    }

    #[test]
    fn test_render_spinner() {
        let node = render_spinner("sp1", "Loading...");
        assert!(node.attrs.get("class").unwrap().contains("rustview-spinner"));
        assert_eq!(node.children.len(), 2);
        assert_eq!(
            node.children[0].attrs.get("class").unwrap(),
            "rustview-spinner-icon"
        );
        assert_eq!(node.children[1].text.as_deref(), Some("Loading..."));
    }

    #[test]
    fn test_render_metric_with_positive_delta() {
        let node = render_metric("m1", "Revenue", "$1000", Some(5.2));
        assert!(node.attrs.get("class").unwrap().contains("rustview-metric"));
        assert_eq!(node.children.len(), 3);
        assert_eq!(node.children[0].text.as_deref(), Some("Revenue"));
        assert_eq!(node.children[1].text.as_deref(), Some("$1000"));
        let delta = &node.children[2];
        assert_eq!(delta.text.as_deref(), Some("+5.2"));
        assert!(delta.attrs.get("style").unwrap().contains("green"));
    }

    #[test]
    fn test_render_metric_with_negative_delta() {
        let node = render_metric("m2", "Users", "50", Some(-3.0));
        assert_eq!(node.children.len(), 3);
        let delta = &node.children[2];
        assert_eq!(delta.text.as_deref(), Some("-3"));
        assert!(delta.attrs.get("style").unwrap().contains("red"));
    }

    #[test]
    fn test_render_metric_no_delta() {
        let node = render_metric("m3", "Score", "100", None);
        assert_eq!(node.children.len(), 2);
    }

    #[test]
    fn test_render_alert_success() {
        let node = render_alert("a1", "Done!", "success");
        assert!(node
            .attrs
            .get("class")
            .unwrap()
            .contains("rustview-alert-success"));
        assert_eq!(node.children[0].text.as_deref(), Some("\u{2713}"));
        assert_eq!(node.children[1].text.as_deref(), Some("Done!"));
    }

    #[test]
    fn test_render_alert_warning() {
        let node = render_alert("a2", "Watch out", "warning");
        assert!(node
            .attrs
            .get("class")
            .unwrap()
            .contains("rustview-alert-warning"));
        assert_eq!(node.children[0].text.as_deref(), Some("\u{26A0}"));
    }

    #[test]
    fn test_render_alert_info() {
        let node = render_alert("a3", "FYI", "info");
        assert!(node
            .attrs
            .get("class")
            .unwrap()
            .contains("rustview-alert-info"));
        assert_eq!(node.children[0].text.as_deref(), Some("\u{2139}"));
    }

    #[test]
    fn test_render_divider() {
        let node = render_divider("d1");
        assert!(node.attrs.get("class").unwrap().contains("rustview-divider"));
        assert_eq!(node.children.len(), 1);
        assert_eq!(node.children[0].tag, "hr");
    }

    #[test]
    fn test_render_columns() {
        let node = render_columns("cols1", &[1, 2, 1]);
        assert!(node.attrs.get("class").unwrap().contains("rustview-columns"));
        let style = node.attrs.get("style").unwrap();
        assert!(style.contains("1fr 2fr 1fr"));
        assert_eq!(node.children.len(), 3);
        assert_eq!(node.children[0].id, "cols1-col-0");
        assert_eq!(
            node.children[0].attrs.get("class").unwrap(),
            "rustview-column"
        );
    }

    #[test]
    fn test_render_sidebar() {
        let node = render_sidebar("sb1");
        assert!(node.attrs.get("class").unwrap().contains("rustview-sidebar"));
        assert_eq!(node.tag, "div");
    }

    #[test]
    fn test_render_expander_open() {
        let node = render_expander("exp1", "More info", true);
        assert!(node
            .attrs
            .get("class")
            .unwrap()
            .contains("rustview-expander"));
        let details = &node.children[0];
        assert_eq!(details.tag, "details");
        assert_eq!(details.attrs.get("open").unwrap(), "true");
        assert_eq!(details.attrs.get("data-widget-type").unwrap(), "expander");
        let summary = &details.children[0];
        assert_eq!(summary.tag, "summary");
        assert_eq!(summary.text.as_deref(), Some("More info"));
    }

    #[test]
    fn test_render_expander_closed() {
        let node = render_expander("exp2", "Details", false);
        let details = &node.children[0];
        assert!(!details.attrs.contains_key("open"));
    }

    #[test]
    fn test_render_tabs() {
        let node = render_tabs("tabs1", &["Tab A", "Tab B", "Tab C"], 1);
        assert!(node.attrs.get("class").unwrap().contains("rustview-tabs"));
        let bar = &node.children[0];
        assert_eq!(bar.attrs.get("class").unwrap(), "rustview-tab-bar");
        assert_eq!(bar.children.len(), 3);
        // Tab B (index 1) should be active
        assert_eq!(bar.children[1].attrs.get("class").unwrap(), "active");
        assert!(!bar.children[0].attrs.contains_key("class"));
        assert_eq!(
            bar.children[0].attrs.get("data-widget-type").unwrap(),
            "tab"
        );
        assert_eq!(bar.children[0].attrs.get("data-tab-index").unwrap(), "0");
        // Content panel
        let panel = &node.children[1];
        assert_eq!(panel.attrs.get("class").unwrap(), "rustview-tab-content");
    }

    #[test]
    fn test_render_container() {
        let node = render_container("ct1");
        assert!(node
            .attrs
            .get("class")
            .unwrap()
            .contains("rustview-container"));
        assert_eq!(node.tag, "div");
        assert!(node.children.is_empty());
    }

    #[test]
    fn test_render_date_picker() {
        let node = render_date_picker("dp1", "Start date", "2025-01-15");
        assert!(node
            .attrs
            .get("class")
            .unwrap()
            .contains("rustview-date-picker"));
        assert_eq!(node.children.len(), 2);
        assert_eq!(node.children[0].text.as_deref(), Some("Start date"));
        let input = &node.children[1];
        assert_eq!(input.attrs.get("type").unwrap(), "date");
        assert_eq!(input.attrs.get("value").unwrap(), "2025-01-15");
        assert_eq!(input.attrs.get("data-widget-type").unwrap(), "date_picker");
    }

    #[test]
    fn test_render_date_picker_empty() {
        let node = render_date_picker("dp2", "End date", "");
        let input = &node.children[1];
        assert_eq!(input.attrs.get("value").unwrap(), "");
    }

    #[test]
    fn test_render_file_upload() {
        let node = render_file_upload("fu1", "Upload CSV");
        assert!(node
            .attrs
            .get("class")
            .unwrap()
            .contains("rustview-file-upload"));
        assert_eq!(node.children.len(), 2);
        assert_eq!(node.children[0].text.as_deref(), Some("Upload CSV"));
        let input = &node.children[1];
        assert_eq!(input.attrs.get("type").unwrap(), "file");
        assert_eq!(input.attrs.get("data-widget-type").unwrap(), "file_upload");
    }

    #[test]
    fn test_render_image_with_caption() {
        let node = render_image("img1", "https://example.com/pic.png", "Logo");
        assert!(node.attrs.get("class").unwrap().contains("rustview-image"));
        assert_eq!(node.children.len(), 2); // img + caption
        let img = &node.children[0];
        assert_eq!(img.tag, "img");
        assert_eq!(
            img.attrs.get("src").unwrap(),
            "https://example.com/pic.png"
        );
        assert_eq!(img.attrs.get("alt").unwrap(), "Logo");
        let caption = &node.children[1];
        assert_eq!(caption.tag, "small");
        assert_eq!(caption.text.as_deref(), Some("Logo"));
    }

    #[test]
    fn test_render_image_without_caption() {
        let node = render_image("img2", "data:image/png;base64,abc", "");
        assert_eq!(node.children.len(), 1); // img only, no caption
    }

    #[test]
    fn test_render_audio() {
        let node = render_audio("aud1", "https://example.com/clip.mp3", "mp3");
        assert!(node.attrs.get("class").unwrap().contains("rustview-audio"));
        let audio = &node.children[0];
        assert_eq!(audio.tag, "audio");
        assert_eq!(audio.attrs.get("controls").unwrap(), "true");
        let source = &audio.children[0];
        assert_eq!(source.tag, "source");
        assert_eq!(
            source.attrs.get("src").unwrap(),
            "https://example.com/clip.mp3"
        );
        assert_eq!(source.attrs.get("type").unwrap(), "audio/mpeg");
    }

    #[test]
    fn test_render_audio_wav() {
        let node = render_audio("aud2", "clip.wav", "wav");
        let source = &node.children[0].children[0];
        assert_eq!(source.attrs.get("type").unwrap(), "audio/wav");
    }

    #[test]
    fn test_render_audio_ogg() {
        let node = render_audio("aud3", "clip.ogg", "ogg");
        let source = &node.children[0].children[0];
        assert_eq!(source.attrs.get("type").unwrap(), "audio/ogg");
    }

    // ---- New widget/layout tests ----

    #[test]
    fn test_render_row() {
        let node = render_row("row1");
        assert_eq!(node.tag, "div");
        assert!(node.attrs.get("class").unwrap().contains("rustview-row"));
        assert!(node.children.is_empty());
    }

    #[test]
    fn test_render_empty() {
        let node = render_empty("empty1");
        assert_eq!(node.tag, "div");
        assert!(node.attrs.get("class").unwrap().contains("rustview-empty"));
        assert!(node.children.is_empty());
    }

    #[test]
    fn test_render_modal_closed() {
        let node = render_modal("modal1", "Settings", false);
        assert_eq!(node.tag, "div");
        assert!(node
            .attrs
            .get("class")
            .unwrap()
            .contains("rustview-modal-overlay"));
        assert_eq!(node.attrs.get("style").unwrap(), "display:none");
        // dialog > [header, body]
        let dialog = &node.children[0];
        assert!(dialog
            .attrs
            .get("class")
            .unwrap()
            .contains("rustview-modal-dialog"));
        let header = &dialog.children[0];
        assert!(header
            .attrs
            .get("class")
            .unwrap()
            .contains("rustview-modal-header"));
        let title = &header.children[0];
        assert_eq!(title.text.as_deref(), Some("Settings"));
    }

    #[test]
    fn test_render_modal_open() {
        let node = render_modal("modal2", "Info", true);
        assert!(!node.attrs.contains_key("style"));
    }

    #[test]
    fn test_render_line_chart() {
        let data = vec![(0.0, 0.0), (1.0, 1.0), (2.0, 4.0)];
        let node = render_line_chart("lc1", "Linear", &data);
        assert!(node.attrs.get("class").unwrap().contains("rustview-chart"));
        // title + svg
        assert_eq!(node.children.len(), 2);
        let title = &node.children[0];
        assert_eq!(title.text.as_deref(), Some("Linear"));
        let svg_container = &node.children[1];
        let svg_text = svg_container.attrs.get("data-innerHTML").unwrap();
        assert!(svg_text.contains("<svg"));
        assert!(svg_text.contains("<polyline"));
    }

    #[test]
    fn test_render_line_chart_empty() {
        let node = render_line_chart("lc2", "", &[]);
        // no title, just svg
        assert_eq!(node.children.len(), 1);
        let svg_text = node.children[0].attrs.get("data-innerHTML").unwrap();
        assert!(svg_text.contains("No data"));
    }

    #[test]
    fn test_render_bar_chart() {
        let data = vec![("Q1", 100.0), ("Q2", 200.0)];
        let node = render_bar_chart("bc1", "Sales", &data);
        assert!(node.attrs.get("class").unwrap().contains("rustview-chart"));
        let svg_text = node.children[1].attrs.get("data-innerHTML").unwrap();
        assert!(svg_text.contains("<svg"));
        assert!(svg_text.contains("<rect"));
        assert!(svg_text.contains("Q1"));
        assert!(svg_text.contains("Q2"));
    }

    #[test]
    fn test_render_bar_chart_empty() {
        let data: Vec<(&str, f64)> = vec![];
        let node = render_bar_chart("bc2", "", &data);
        let svg_text = node.children[0].attrs.get("data-innerHTML").unwrap();
        assert!(svg_text.contains("No data"));
    }

    #[test]
    fn test_render_scatter_chart() {
        let data = vec![(1.0, 2.0), (3.0, 4.0), (5.0, 1.0)];
        let node = render_scatter_chart("sc1", "Scatter", &data);
        let svg_text = node.children[1].attrs.get("data-innerHTML").unwrap();
        assert!(svg_text.contains("<svg"));
        assert!(svg_text.contains("<circle"));
    }

    #[test]
    fn test_render_histogram() {
        let data = vec![1.0, 2.0, 2.5, 3.0, 3.5, 4.0, 4.5, 5.0];
        let node = render_histogram("hist1", "Distribution", &data, 4);
        let svg_text = node.children[1].attrs.get("data-innerHTML").unwrap();
        assert!(svg_text.contains("<svg"));
        assert!(svg_text.contains("<rect"));
    }

    #[test]
    fn test_render_histogram_empty() {
        let node = render_histogram("hist2", "", &[], 5);
        let svg_text = node.children[0].attrs.get("data-innerHTML").unwrap();
        assert!(svg_text.contains("No data"));
    }

    #[test]
    fn test_render_histogram_single_value() {
        let data = vec![3.0, 3.0, 3.0];
        let node = render_histogram("hist3", "Same", &data, 2);
        let svg_text = node.children[1].attrs.get("data-innerHTML").unwrap();
        assert!(svg_text.contains("<svg"));
        assert!(svg_text.contains("<rect"));
    }

    #[test]
    fn test_line_chart_single_point() {
        let data = vec![(1.0, 1.0)];
        let node = render_line_chart("lc3", "Single", &data);
        let svg_text = node.children[1].attrs.get("data-innerHTML").unwrap();
        assert!(svg_text.contains("<svg"));
        assert!(svg_text.contains("<polyline"));
    }

    #[test]
    fn test_render_toast_success() {
        let node = render_toast("t1", "Saved!", "success");
        assert_eq!(node.tag, "div");
        assert!(node.attrs.get("class").unwrap().contains("rustview-toast-success"));
        assert_eq!(node.attrs.get("data-widget-type").unwrap(), "toast");
        assert_eq!(node.children.len(), 2);
        assert_eq!(node.children[0].text.as_deref(), Some("\u{2713}"));
        assert_eq!(node.children[1].text.as_deref(), Some("Saved!"));
    }

    #[test]
    fn test_render_toast_error() {
        let node = render_toast("t2", "Failed", "error");
        assert!(node.attrs.get("class").unwrap().contains("rustview-toast-error"));
        assert_eq!(node.children[0].text.as_deref(), Some("\u{2717}"));
    }

    #[test]
    fn test_render_toast_warning() {
        let node = render_toast("t3", "Careful", "warning");
        assert!(node.attrs.get("class").unwrap().contains("rustview-toast-warning"));
        assert_eq!(node.children[0].text.as_deref(), Some("\u{26A0}"));
    }

    #[test]
    fn test_render_toast_info() {
        let node = render_toast("t4", "Note", "info");
        assert!(node.attrs.get("class").unwrap().contains("rustview-toast-info"));
        assert_eq!(node.children[0].text.as_deref(), Some("\u{2139}"));
    }

    #[test]
    fn test_render_image_upload_empty() {
        let node = render_image_upload("iu1", "Upload photo", "");
        assert_eq!(node.tag, "div");
        assert!(node.attrs.get("class").unwrap().contains("rustview-image-upload"));
        assert_eq!(node.children.len(), 2); // label + input, no preview
        let input = &node.children[1];
        assert_eq!(input.attrs.get("accept").unwrap(), "image/*");
        assert_eq!(input.attrs.get("data-widget-type").unwrap(), "image_upload");
    }

    #[test]
    fn test_render_image_upload_with_preview() {
        let node = render_image_upload("iu2", "Photo", "data:image/png;base64,abc");
        assert_eq!(node.children.len(), 3); // label + input + preview
        let preview = &node.children[2];
        assert_eq!(preview.tag, "img");
        assert_eq!(preview.attrs.get("src").unwrap(), "data:image/png;base64,abc");
        assert!(preview.attrs.get("class").unwrap().contains("rustview-image-upload-preview"));
    }

    #[test]
    fn test_render_form() {
        let node = render_form("form1");
        assert_eq!(node.tag, "div");
        assert!(node.attrs.get("class").unwrap().contains("rustview-form"));
        assert_eq!(node.attrs.get("data-widget-type").unwrap(), "form");
    }

    #[test]
    fn test_render_form_submit_button() {
        let node = render_form_submit_button("fsb1", "Submit");
        assert_eq!(node.tag, "div");
        assert!(node.attrs.get("class").unwrap().contains("rustview-form-submit"));
        let btn = &node.children[0];
        assert_eq!(btn.tag, "button");
        assert_eq!(btn.text.as_deref(), Some("Submit"));
        assert_eq!(btn.attrs.get("data-widget-type").unwrap(), "form_submit");
    }

    #[test]
    fn test_render_link() {
        let node = render_link("l1", "Click me", "https://example.com");
        assert_eq!(node.tag, "div");
        assert!(node.attrs.get("class").unwrap().contains("rustview-link"));
        let a = &node.children[0];
        assert_eq!(a.tag, "a");
        assert_eq!(a.text.as_deref(), Some("Click me"));
        assert_eq!(a.attrs.get("href").unwrap(), "https://example.com");
        assert_eq!(a.attrs.get("target").unwrap(), "_blank");
        assert_eq!(a.attrs.get("rel").unwrap(), "noopener noreferrer");
    }

    #[test]
    fn test_render_video_mp4() {
        let node = render_video("v1", "test.mp4", "mp4");
        assert_eq!(node.tag, "div");
        assert!(node.attrs.get("class").unwrap().contains("rustview-video"));
        let video = &node.children[0];
        assert_eq!(video.tag, "video");
        assert_eq!(video.attrs.get("controls").unwrap(), "true");
        let source = &video.children[0];
        assert_eq!(source.tag, "source");
        assert_eq!(source.attrs.get("type").unwrap(), "video/mp4");
    }

    #[test]
    fn test_render_video_webm() {
        let node = render_video("v2", "test.webm", "webm");
        let source = &node.children[0].children[0];
        assert_eq!(source.attrs.get("type").unwrap(), "video/webm");
    }

    #[test]
    fn test_render_badge_predefined_color() {
        let node = render_badge("b1", "Active", "green");
        assert_eq!(node.tag, "span");
        assert!(node.attrs.get("class").unwrap().contains("rustview-badge"));
        assert_eq!(node.text.as_deref(), Some("Active"));
        let style = node.attrs.get("style").unwrap();
        assert!(style.contains("#1a3a2a")); // green background
        assert!(style.contains("#50fa7b")); // green foreground
    }

    #[test]
    fn test_render_badge_custom_color() {
        let node = render_badge("b2", "Custom", "#ff00ff");
        let style = node.attrs.get("style").unwrap();
        assert!(style.contains("#ff00ff")); // custom color as foreground
    }

    #[test]
    fn test_render_badge_all_predefined() {
        for color in &["red", "green", "blue", "yellow", "gray", "purple", "orange"] {
            let node = render_badge("b", "Test", color);
            assert_eq!(node.tag, "span");
            assert!(node.attrs.get("style").unwrap().len() > 0);
        }
    }
}
