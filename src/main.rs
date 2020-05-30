use std::time::Duration;
use std::thread;
extern crate clipboard;
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

fn clear_clipboard() {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents("".to_owned()).unwrap();
    println!("every three seconds");
}

fn main() {
    let mut planner = periodic::Planner::new();
    planner.add(
        clear_clipboard,
        periodic::Every::new(Duration::from_secs(3)),
    );
    planner.start();

    let mut done = false; // mut done: bool
    while !done {
        thread::sleep(::std::time::Duration::new(1, 0));
    }

    drop(planner)
}
