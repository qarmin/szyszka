use crate::example_fields::update_examples;
use crate::gui_data_things::class_dialog_rules::GuiDialogRules;
use crate::help_function::{validate_name, validate_number};
use gtk4::prelude::*;
use gtk4::{CheckButton, Entry};

pub fn connect_examples_entry_number(entry: &Entry, window_rules: &GuiDialogRules) {
    let window_rules = window_rules.clone();
    entry.connect_changed(move |entry| {
        let old_name = entry.text().to_string();
        let validate_number = validate_number(&old_name);
        if validate_number != old_name {
            entry.set_text(&validate_number);
        }
        update_examples(&window_rules, None);
    });
}

pub fn connect_examples_entry_name(entry: &Entry, window_rules: &GuiDialogRules) {
    let window_rules = window_rules.clone();
    entry.connect_changed(move |entry| {
        let old_name = entry.text().to_string();
        let validate_number = validate_name(&entry.text());
        if validate_number != old_name {
            entry.set_text(&validate_number);
        }
        update_examples(&window_rules, None);
    });
}
pub fn connect_examples_check_button(check_button: &CheckButton, window_rules: &GuiDialogRules) {
    let window_rules = window_rules.clone();
    check_button.connect_toggled(move |_e| {
        update_examples(&window_rules, None);
    });
}
