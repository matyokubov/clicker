use enigo::{Enigo, MouseButton, MouseControllable};
use std::{thread, time::Duration};

fn c(x: i32, y: i32, times: u32, delay: u64, countdown: u32) {
    for sec in (0..countdown).rev() {
        println!("{}", sec);
        thread::sleep(Duration::from_secs(1));
    }
    println!("GO");

    let mut enigo = Enigo::new();

    for _ in 0..times {
        enigo.mouse_move_to(x, y);
        enigo.mouse_down(MouseButton::Left);
        thread::sleep(Duration::from_millis(delay));
        enigo.mouse_up(MouseButton::Left);
    }
}

fn main() {
    c(500, 500, 100, 0, 0);
}
