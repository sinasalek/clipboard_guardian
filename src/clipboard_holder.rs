use std::thread;
use std::sync::mpsc::Sender;

pub struct ClipboardHolder {
    history: Vec<String>,
    pub history_sender: Sender<String>
}
impl ClipboardHolder {
    pub fn new() -> Self {
        let (history_sender, history_receiver) = std::sync::mpsc::channel();
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

    pub fn safe_history_add(&mut self, content: String) {
        self.history_sender.send(content);
    }

    pub fn history_add(&mut self) {

    }

    pub fn history_get_last_remove(&mut self) {

    }

    pub fn history_get_total(&mut self) {

    }
}