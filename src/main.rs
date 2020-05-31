use std::time::Duration;
use std::thread;
extern crate copypasta;
use copypasta::{ClipboardContext, ClipboardProvider};

mod config;

fn clear_clipboard() {
    let mut ctx = ClipboardContext::new().unwrap();
    let content = ctx.get_contents();
    if content.is_ok() {
        println!("{:?}", content);
        //ctx.set_contents("".to_owned()).unwrap();
    }
    println!("every three seconds");
}

fn main() {
    let cfg = config::config_get().ok().unwrap();
    // println!("{:#?}", cfg.ok().unwrap());

    let mut planner = periodic::Planner::new();
    planner.add(
        clear_clipboard,
        periodic::Every::new(Duration::from_secs(cfg.interval)),
    );
    planner.start();

    let done = false; // mut done: bool
    while !done {
        thread::sleep(::std::time::Duration::new(1, 0));
    }

    drop(planner)
}
