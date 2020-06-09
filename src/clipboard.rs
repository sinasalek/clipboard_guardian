extern crate copypasta;
use copypasta::{ClipboardContext, ClipboardProvider};
use clipboard_master::{Master, ClipboardHandler, CallbackResult};
use std::time::Duration;
use std::io;
use std::ops::Fn;
use std::sync::{Arc};
use super::clipboard_holder::{CallbackType};

struct Handler {
    clipboard_sender: CallbackType
}

impl ClipboardHandler for Handler {
    fn on_clipboard_change(&mut self) -> CallbackResult {
        println!("Clipboard change happened!");
        let clipboard_sender = self.clipboard_sender.clone();
        clipboard_clear_timer(clipboard_sender);
        CallbackResult::Next
    }

    fn on_clipboard_error(&mut self, error: io::Error) -> CallbackResult {
        eprintln!("Error: {}", error);
        CallbackResult::Next
    }
}

fn clipboard_clear(clipboard_sender: CallbackType) {
    let mut ctx = ClipboardContext::new().unwrap();
    let content = ctx.get_contents();
    if content.is_ok() {
        println!("{:?}", content);
        println!("cleared");
        clipboard_sender(&content.unwrap());
        //ctx.set_contents("".to_owned()).unwrap();
    }
}

pub fn clipboard_clear_timer(callback: CallbackType) {
    let mut planner = periodic::Planner::new();
    //Box::new(move || clipboard_clear(callback)),
    planner.add(
        move || clipboard_clear(callback.clone()),
        periodic::After::new(Duration::from_secs(5)),
    );
    planner.start();
    drop(planner)
}

pub fn clipboard_change_monitor(_interval:u64, callback: CallbackType) {
    let _ = Master::new(Handler{ clipboard_sender: callback }).run();
}