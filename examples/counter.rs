/// Counter example with session state — matches the design document §8.3.
use rustview::prelude::*;

fn counter_app(ui: &mut Ui) {
    let count = ui.get_state::<i64>("counter", 0);
    if ui.button("Increment") {
        ui.set_state("counter", count + 1);
    }
    if ui.button("Reset") {
        ui.set_state("counter", 0i64);
    }
    ui.write(format!("Count: {}", ui.get_state::<i64>("counter", 0)));
}

fn main() {
    rustview::run(counter_app);
}
