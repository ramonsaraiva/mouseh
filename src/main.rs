use inputbot::{KeybdKey::*, MouseCursor};
use std::{thread::sleep, time::Duration};

fn main() {
    let speed : i32 = 10;

    HKey.bind(move || {
        MouseCursor::move_rel(-1 * speed, 0);
        sleep(Duration::from_millis(1));
    });

    inputbot::handle_input_events();
}
