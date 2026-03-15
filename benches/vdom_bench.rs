use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rustview::vdom::{self, VNode};

fn build_widget_tree(n: usize) -> VNode {
    let mut root = VNode::new("rustview-root", "div");
    root.attrs
        .insert("class".to_string(), "rustview-app".to_string());
    for i in 0..n {
        let widget = VNode::new(format!("w-{i}"), "div")
            .with_attr("class", "rustview-widget rustview-write")
            .with_child(VNode::new(format!("w-{i}-text"), "p").with_text(format!("Widget {i}")));
        root.children.push(widget);
    }
    root
}

fn bench_diff_no_change(c: &mut Criterion) {
    let tree = build_widget_tree(100);
    c.bench_function("diff_100_no_change", |b| {
        b.iter(|| {
            let patches = vdom::diff(black_box(&tree), black_box(&tree));
            assert!(patches.is_empty());
        });
    });
}

fn bench_diff_5_changed(c: &mut Criterion) {
    let old = build_widget_tree(100);
    let mut new = old.clone();
    // Change 5 widgets
    for i in [10, 30, 50, 70, 90] {
        if let Some(child) = new.children[i].children.first_mut() {
            child.text = Some(format!("Changed Widget {i}"));
        }
    }

    c.bench_function("diff_100_5_changed", |b| {
        b.iter(|| {
            let patches = vdom::diff(black_box(&old), black_box(&new));
            assert_eq!(patches.len(), 5);
        });
    });
}

fn bench_widget_rerun_10(c: &mut Criterion) {
    c.bench_function("widget_rerun_10", |b| {
        b.iter(|| {
            let tree = black_box(build_widget_tree(10));
            black_box(tree);
        });
    });
}

fn bench_widget_rerun_50(c: &mut Criterion) {
    c.bench_function("widget_rerun_50", |b| {
        b.iter(|| {
            let tree = black_box(build_widget_tree(50));
            black_box(tree);
        });
    });
}

fn bench_widget_rerun_100(c: &mut Criterion) {
    c.bench_function("widget_rerun_100", |b| {
        b.iter(|| {
            let tree = black_box(build_widget_tree(100));
            black_box(tree);
        });
    });
}

fn bench_cache_hit(c: &mut Criterion) {
    // Pre-populate cache
    rustview::cache::insert_cached("bench_fn", 42, 12345i64);

    c.bench_function("cache_hit", |b| {
        b.iter(|| {
            let result = rustview::cache::get_cached::<i64>(black_box("bench_fn"), black_box(42));
            assert_eq!(result, Some(12345));
        });
    });
}

criterion_group!(
    benches,
    bench_diff_no_change,
    bench_diff_5_changed,
    bench_widget_rerun_10,
    bench_widget_rerun_50,
    bench_widget_rerun_100,
    bench_cache_hit
);
criterion_main!(benches);
