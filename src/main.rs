use data::TodoState;
use druid::{AppLauncher, WindowDesc};
use im::Vector;
use saver::read_stored;
use ui::ui_builder;

mod data;
mod saver;
mod ui;
fn main() {
    let main_window = WindowDesc::new(ui_builder())
        .title("todo")
        .window_size((400., 400.));
    let stored = read_stored();
    let default_state = TodoState {
        todos: Vector::from(stored.tasks),
        ..Default::default()
    };

    AppLauncher::with_window(main_window)
        .launch(default_state)
        .expect("failed")
}
