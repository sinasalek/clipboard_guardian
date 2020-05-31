extern crate copypasta;
use copypasta::{ClipboardContext, ClipboardProvider};
use clipboard_master::{Master, ClipboardHandler, CallbackResult};
use std::time::Duration;
use std::io;

struct Handler;

impl ClipboardHandler for Handler {
    fn on_clipboard_change(&mut self) -> CallbackResult {
        println!("Clipboard change happened!");
        clipboard_clear_timer();
        CallbackResult::Next
    }

    fn on_clipboard_error(&mut self, error: io::Error) -> CallbackResult {
        eprintln!("Error: {}", error);
        CallbackResult::Next
    }
}

pub fn clipboard_clear() {
    let mut ctx = ClipboardContext::new().unwrap();
    let content = ctx.get_contents();
    if content.is_ok() {
        println!("{:?}", content);
        println!("cleared");
        //ctx.set_contents("".to_owned()).unwrap();
    }
}

pub fn clipboard_clear_timer() {
    let mut planner = periodic::Planner::new();
    planner.add(
        clipboard_clear,
        periodic::After::new(Duration::from_secs(5)),
    );
    planner.start();
    drop(planner)
}

pub fn clipboard_change_monitor(_interval:u64) {
    let _ = Master::new(Handler).run();
}

