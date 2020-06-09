use std::thread;
use crossbeam::channel::{unbounded, Sender};
use std::sync::{Arc};

pub type ClipboardSenderType = Sender<String>;

pub type CallbackType<'a> = Arc<dyn Fn(&String) + Send + Sync+ 'a>;

pub struct ClipboardHolder {
    history: Vec<String>,
    pub history_sender: ClipboardSenderType
}
impl ClipboardHolder {
    pub fn new() -> Self {
        let (history_sender, history_receiver) = unbounded();
        // Managing clipboard history and state
        thread::spawn(move || {
            loop {
                println!("{:#?}", history_receiver.recv().unwrap());
            }
        });

        ClipboardHolder {
            history: Vec::new(),
            history_sender
        }
    }

    pub fn safe_history_add(&self, content: Box<&String>) {
        self.history_sender.send(content.to_string());
    }

    pub fn history_add(&mut self) {

    }

    pub fn history_get_last_remove(&mut self) {

    }

    pub fn history_get_total(&mut self) {

    }

    pub fn sender_callback_create(&self) -> CallbackType<'_> {
        Arc::new(| text: &String | { self.safe_history_add(Box::new(text)); })
    }
}