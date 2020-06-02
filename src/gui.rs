extern crate iui;
extern crate exitcode;
use iui::prelude::*;
use iui::controls::{Button, Label, Entry, VerticalBox, Group, HorizontalBox};
use std::convert::Into;
use std::rc::Rc;
use std::cell::RefCell;

/// This struct will hold the values that multiple callbacks will need to access.
struct State {
    interval_val: String,
    history_total_val: String,
    restore_hot_key_val: String
}

pub fn show_settings_ui() {
    // Initialize the UI library
    let ui = UI::init().expect("Couldn't initialize UI library");
    // Create a window into which controls can be placed
    let mut win = Window::new(&ui, "Clipboard Guardian", 300, 400, WindowType::NoMenubar);

    // Initialize the state of the application.
    let state = Rc::new(RefCell::new(State { interval_val: "".into(), history_total_val: "".into(), restore_hot_key_val: "".into()}));

    //--(Begin)--> Create inputs for settings
    let (input_vbox, mut interval, mut history_total, mut restore_hot_key) = {
        // The group will hold all the inputs
        let mut input_group = Group::new(&ui, "Settings");
        // The vertical box arranges the inputs within the groups
        let mut input_vbox = VerticalBox::new(&ui);
        input_vbox.set_padded(&ui, true);
        // Numerical inputs
        let interval = Entry::new(&ui);
        let history_total = Entry::new(&ui);
        let restore_hot_key = Entry::new(&ui);
        // Add everything in hierarchy
        // Note the reverse order here. Again, it's not necessary, but it improves
        // readability.
        input_vbox.append(&ui, interval.clone(), LayoutStrategy::Compact);
        input_vbox.append(&ui, history_total.clone(), LayoutStrategy::Compact);
        input_vbox.append(&ui, restore_hot_key.clone(), LayoutStrategy::Compact);

        //input_group.set_child(&ui, input_vbox);
        (input_vbox, interval, history_total, restore_hot_key)
    };
    //--(End)--> Create inputs for settings

    //--(Begin)--> Create inputs labels for settings
    let (label_vbox, interval_label, history_total_label, restore_hot_key_label) = {
        let mut label_group = Group::new(&ui, "Outputs");
        let mut label_vbox = VerticalBox::new(&ui);
        let interval_label = Label::new(&ui, "SS");
        let history_total_label = Label::new(&ui, "ff");
        let restore_hot_key_label = Label::new(&ui, "fff");
        label_vbox.append(&ui, interval_label.clone(), LayoutStrategy::Compact);
        label_vbox.append(&ui, history_total_label.clone(), LayoutStrategy::Compact);
        label_vbox.append(&ui, restore_hot_key_label.clone(), LayoutStrategy::Compact);
        (label_vbox, interval_label, history_total_label, restore_hot_key_label)
    };
    //--(End)--> Create inputs labels for settings

    // This horizontal box will arrange the two groups of controls.
    let mut settings_box = HorizontalBox::new(&ui);
    settings_box.append(&ui, label_vbox, LayoutStrategy::Stretchy);
    settings_box.append(&ui, input_vbox, LayoutStrategy::Stretchy);
    let mut settings_group = Group::new(&ui, "Settings");
    settings_group.set_child(&ui, settings_box);

    //--(Begin)--> Set the events
    win.on_closing(&ui, {
        let ui = ui.clone();
        move |_| {
            ui.quit();
            std::process::exit(exitcode::OK);
        }
    });

    /*
    let mut button = Button::new(&ui, "Close");
    button.on_clicked(&ui, {
        let ui = ui.clone();
        move |btn| {
            btn.set_text(&ui, "Clicked!");
        }
    });
    */

    interval.on_changed(&ui, {
        let state = state.clone();
        move |val| { state.borrow_mut().interval_val = val; }
    });

    let mut quit_button = Button::new(&ui, "Stop and Exit");
    quit_button.on_clicked(&ui, {
        let ui = ui.clone();
        move |_| {
            ui.quit();
            std::process::exit(exitcode::OK);
        }
    });

    let mut save_button = Button::new(&ui, "Save");
    save_button.on_clicked(&ui, {
        let ui = ui.clone();
        move |_| {
            ui.quit();
            std::process::exit(exitcode::OK);
        }
    });
    //--(End)--> Set the events

    // Create a vertical layout to hold the controls
    let mut vbox = VerticalBox::new(&ui);
    vbox.set_padded(&ui, true);

    // Create a new label. Note that labels don't auto-wrap!
    let mut label_text = String::new();
    label_text.push_str("There is a ton of text in this label.\n");
    let label = Label::new(&ui, &label_text);

    vbox.append(&ui, label, LayoutStrategy::Stretchy);
    vbox.append(&ui, settings_group, LayoutStrategy::Stretchy);


    let mut buttons_box = HorizontalBox::new(&ui);
    buttons_box.append(&ui, quit_button, LayoutStrategy::Stretchy);
    buttons_box.append(&ui, save_button, LayoutStrategy::Stretchy);
    vbox.append(&ui, buttons_box, LayoutStrategy::Compact);

    // Actually put the button in the window
    win.set_child(&ui, vbox);
    // Show the window
    win.show(&ui);

    // Run the application
    ui.main();
}