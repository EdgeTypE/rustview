/// Minimal RustView example — matches the design document §8.1.
use rustview::prelude::*;

fn app(ui: &mut Ui) {
    let name = ui.text_input("Your name", "World");
    let times = ui.int_slider("Repeat", 1..=10, 3);
    for _ in 0..times {
        ui.write(format!("Hello, {}!", name));
    }
}

fn main() {
    rustview::run(app);
}
