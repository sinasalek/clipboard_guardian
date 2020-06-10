//#![windows_subsystem = "windows"]
//#![allow(unused_variables)]
use hotkey;
use std::{thread};
use std::sync::{Arc};

mod config;
mod clipboard;
mod clipboard_holder;
mod gui;
mod system_tray;
mod helpers;

fn main() {
    let clipboard_holder: clipboard_holder::ClipboardHolderContainer = clipboard_holder::ClipboardHolderContainer::new();
    let callback = clipboard_holder.sender_callback_create();
    // Load the config
    let cfg = Arc::new(config::config_get().ok().unwrap());
    let thread_cfg = cfg.clone();
    let mut handles = Vec::new();
    // println!("{:#?}", cfg.ok().unwrap());

    // Setup a separate thread to monitor clipboard changes
    handles.push(thread::spawn(move || {
        clipboard::clipboard_change_monitor(thread_cfg.interval, callback);
    }));

    // Setup a separate thread to monitor clipboard changes
    handles.push(thread::spawn(|| {
        gui::show_settings_ui()
    }));

    // system tray for the app
    handles.push(thread::spawn(|| {
        let p1 = helpers::get_target_dir();
        let app_path = helpers::get_top_dir(&p1);
        system_tray::start(app_path.to_str().unwrap());
    }));

    // Setup hot-keys for certain actions
    let mut hk = hotkey::Listener::new();
    hk.register_hotkey(
        hotkey::modifiers::CONTROL | hotkey::modifiers::SHIFT,
        'J' as u32,
        || println!("Ctrl-Shift-J pressed!"),
    )
        .unwrap();

    hk.listen();

    // Wait for other threads to finish.
    for handle in handles {
        handle.join().unwrap();
    }
}
