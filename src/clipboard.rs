extern crate copypasta;
use copypasta::{ClipboardContext, ClipboardProvider};
use clipboard_master::{Master, ClipboardHandler, CallbackResult};
use std::time::Duration;
use std::io;
use std::ops::Fn;

struct Handler {
    clipboard_sender: dyn Fn(String)
}

impl ClipboardHandler for Handler {

    fn on_clipboard_change(&mut self) -> CallbackResult {
        println!("Clipboard change happened!");
        clipboard_clear_timer(self.clipboard_sender);
        CallbackResult::Next
    }

    fn on_clipboard_error(&mut self, error: io::Error) -> CallbackResult {
        eprintln!("Error: {}", error);
        CallbackResult::Next
    }
}

fn clipboard_clear(clipboard_sender: dyn Fn(String)) {
    let mut ctx = ClipboardContext::new().unwrap();
    let content = ctx.get_contents();
    if content.is_ok() {
        println!("{:?}", content);
        println!("cleared");
        clipboard_sender(content.unwrap());
        //ctx.set_contents("".to_owned()).unwrap();
    }
}

fn clipboard_clear_timer(clipboard_sender: dyn Fn(String)) {
    let mut planner = periodic::Planner::new();
    //let clipboard_sender2 = clipboard_sender.clone();
    planner.add(
        move || clipboard_clear(&clipboard_sender),
        periodic::After::new(Duration::from_secs(5)),
    );
    planner.start();
    drop(planner)
}

pub fn clipboard_change_monitor(_interval:u64, clipboard_sender: dyn Fn(String)) {
    let _ = Master::new(Handler{clipboard_sender}).run();
}

