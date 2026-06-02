/// Reactive Charts Demo
///
/// Demonstrates all four chart types (line, scatter, bar, histogram) with
/// live data updates on user interaction. Add and remove data points to
/// observe how charts re-render in real time.
///
/// Run: cargo run --example reactive_charts
use rustview::prelude::*;
use rustview::server::RustViewConfig;

const POINTS_KEY: &str = "chart_points";
const SEQ_KEY: &str = "seq";

fn reactive_charts(ui: &mut Ui) {
    ui.heading("Reactive Charts");
    ui.caption("Interactively add and remove data points. All four chart types \
                (line, scatter, bar, histogram) update live on each interaction.");

    ui.divider();

    let points = ui.get_state::<Vec<(f64, f64)>>(POINTS_KEY, vec![]);
    let seq = ui.get_state::<f64>(SEQ_KEY, 0.0);

    // --- Metrics row ---
    ui.subheading("Statistics");
    let last_y = points.last().map(|&(_, y)| y).unwrap_or(0.0);
    let avg_y = if points.is_empty() {
        0.0
    } else {
        points.iter().map(|&(_, y)| y).sum::<f64>() / points.len() as f64
    };
    let range = if points.is_empty() {
        (0.0, 0.0)
    } else {
        let ys: Vec<f64> = points.iter().map(|&(_, y)| y).collect();
        let min = ys.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = ys.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        (min, max)
    };

    ui.columns(
        &[1, 1, 1, 1],
        &[
            &|c: &mut Ui| c.metric("Point count", points.len() as i64, None),
            &|c: &mut Ui| c.metric("Last Y", format!("{:.2}", last_y), None),
            &|c: &mut Ui| c.metric("Avg Y", format!("{:.2}", avg_y), None),
            &|c: &mut Ui| {
                c.metric(
                    "Y range",
                    format!("{:.1} – {:.1}", range.0, range.1),
                    None,
                )
            },
        ],
    );

    ui.divider();

    // --- Controls ---
    ui.subheading("Controls");

    ui.columns(
        &[1, 1, 1, 1, 1, 1],
        &[
            &|c: &mut Ui| {
                if c.button("Add sine") {
                    let mut pts = c.get_state::<Vec<(f64, f64)>>(POINTS_KEY, vec![]);
                    let mut s = c.get_state::<f64>(SEQ_KEY, 0.0);
                    pts.push((s, (s * 0.5).sin() * 10.0));
                    s += 1.0;
                    c.set_state(POINTS_KEY, pts);
                    c.set_state(SEQ_KEY, s);
                }
            },
            &|c: &mut Ui| {
                if c.button("Add linear") {
                    let mut pts = c.get_state::<Vec<(f64, f64)>>(POINTS_KEY, vec![]);
                    let mut s = c.get_state::<f64>(SEQ_KEY, 0.0);
                    pts.push((s, s * 0.5));
                    s += 1.0;
                    c.set_state(POINTS_KEY, pts);
                    c.set_state(SEQ_KEY, s);
                }
            },
            &|c: &mut Ui| {
                if c.button("Bulk +10") {
                    let mut pts = c.get_state::<Vec<(f64, f64)>>(POINTS_KEY, vec![]);
                    let mut s = c.get_state::<f64>(SEQ_KEY, 0.0);
                    for _ in 0..10 {
                        pts.push((s, (s * 0.3).sin() * 8.0 + (s * 0.1) * 2.0));
                        s += 1.0;
                    }
                    c.set_state(POINTS_KEY, pts);
                    c.set_state(SEQ_KEY, s);
                }
            },
            &|c: &mut Ui| {
                if c.button("Remove last") {
                    let mut pts = c.get_state::<Vec<(f64, f64)>>(POINTS_KEY, vec![]);
                    let mut s = c.get_state::<f64>(SEQ_KEY, 0.0);
                    pts.pop();
                    if s > 0.0 {
                        s -= 1.0;
                    }
                    c.set_state(POINTS_KEY, pts);
                    c.set_state(SEQ_KEY, s);
                }
            },
            &|c: &mut Ui| {
                if c.button("Clear") {
                    let empty: Vec<(f64, f64)> = vec![];
                    c.set_state(POINTS_KEY, empty);
                    c.set_state(SEQ_KEY, 0.0_f64);
                }
            },
            &|c: &mut Ui| {
                if c.button("Randomize") {
                    let mut s = c.get_state::<f64>(SEQ_KEY, 0.0);
                    let mut pts: Vec<(f64, f64)> = Vec::new();
                    for i in 0..20 {
                        let x = s + i as f64;
                        let y = (x * 0.7).sin() * 12.0 + (x * 0.3).cos() * 5.0;
                        pts.push((x, y));
                    }
                    s += 20.0;
                    c.set_state(POINTS_KEY, pts);
                    c.set_state(SEQ_KEY, s);
                }
            },
        ],
    );

    ui.divider();

    // --- Raw data ---
    ui.expander("Raw data", |inner| {
        inner.json(&serde_json::json!({
            "total_points": points.len(),
            "sequence": seq,
            "points": points.iter().map(|&(x, y)| {
                serde_json::json!({"x": x, "y": format!("{:.2}", y)})
            }).collect::<Vec<_>>(),
        }));
    });

    ui.divider();

    // --- Charts ---
    ui.subheading("Charts");

    ui.columns(
        &[1, 1],
        &[
            &|col: &mut Ui| {
                let pts = col.get_state::<Vec<(f64, f64)>>(POINTS_KEY, vec![]);
                col.line_chart("Line Chart", &pts);
            },
            &|col: &mut Ui| {
                let pts = col.get_state::<Vec<(f64, f64)>>(POINTS_KEY, vec![]);
                col.scatter_chart("Scatter Chart", &pts);
            },
        ],
    );

    // Bar chart: last 5 Y values as categories A-E
    let bar_data: Vec<(&str, f64)> = {
        let recent: Vec<f64> = points.iter().rev().take(5).map(|&(_, y)| y).collect();
        let labels = ["A", "B", "C", "D", "E"];
        labels
            .iter()
            .enumerate()
            .map(|(i, label)| (*label, *recent.get(i).unwrap_or(&0.0)))
            .collect()
    };
    let bar_title = format!("Bar Chart - last 5 Y values ({} total)", points.len());

    // Histogram: Y distribution
    let hist_data: Vec<f64> = points.iter().map(|&(_, y)| y).collect();
    let hist_title = format!("Histogram - Y distribution ({} points)", points.len());

    ui.columns(
        &[1, 1],
        &[
            &|col: &mut Ui| {
                col.bar_chart(&bar_title, &bar_data);
            },
            &|col: &mut Ui| {
                col.histogram(&hist_title, &hist_data, 6);
            },
        ],
    );

    ui.divider();

    // --- Footer ---
    ui.caption("Built with RustView - reactive charts driven by session state and automatic re-rendering.");
}

fn main() {
    let config = RustViewConfig {
        open_browser: true,
        ..Default::default()
    };
    rustview::run_with_config(reactive_charts, config);
}
