use enigo::{Enigo,MouseButton, MouseControllable};
use std::thread;
use std::time::Duration;

fn main() {
    let wait_time = Duration::from_millis(150);
    let mut enigo = Enigo::new();

    thread::sleep(wait_time);

    // enigo.mouse_move_to(200, 1300);
    // thread::sleep(wait_time);
    let mut index =0;
    while index < 5*60*60*10 {
        enigo.mouse_click(MouseButton::Left);
        thread::sleep(wait_time);
        index+=1;
        if index % 600 ==0{
            println!("index: {}", index/600)
        }
    }
}
