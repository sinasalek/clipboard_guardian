//#![windows_subsystem = "windows"]
use hotkey;
use std::thread;
use std::sync::{Arc};

mod config;
mod clipboard;
mod clipboard_holder;
mod gui;

fn main() {
    let clipboard_holder = clipboard_holder::ClipboardHolder::new();
    let callback = clipboard_holder.sender_callback_create();
    // Load the config
    let cfg = Arc::new(config::config_get().ok().unwrap());
    let thread_cfg = cfg.clone();
    // println!("{:#?}", cfg.ok().unwrap());

    // Setup a separate thread to monitor clipboard changes
    thread::spawn(move || {
        clipboard::clipboard_change_monitor(thread_cfg.interval, callback.clone());
    });

    // Setup a separate thread to monitor clipboard changes
    thread::spawn(|| {
        gui::show_settings_ui()
    });

    // Setup hot-keys for certain actions
    let mut hk = hotkey::Listener::new();
    hk.register_hotkey(
        hotkey::modifiers::CONTROL | hotkey::modifiers::SHIFT,
        'J' as u32,
        || println!("Ctrl-Shift-J pressed!"),
    )
        .unwrap();

    hk.listen();
}
