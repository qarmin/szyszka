use gtk4::prelude::*;

use crate::example_fields::update_examples;
use crate::gui_data::GuiData;

pub fn connect_rule_window_size_letters_click(gui_data: &GuiData) {
    let window_rules = gui_data.window_rules.clone();
    let check_button_letters_usage_both = gui_data.window_rules.size_letters.check_button_letters_usage_both.clone();
    let check_button_letters_usage_name = gui_data.window_rules.size_letters.check_button_letters_usage_name.clone();
    let check_button_letters_usage_extension = gui_data.window_rules.size_letters.check_button_letters_usage_extension.clone();
    let check_button_letters_type_lowercase = gui_data.window_rules.size_letters.check_button_letters_type_lowercase.clone();
    let check_button_letters_type_uppercase = gui_data.window_rules.size_letters.check_button_letters_type_uppercase.clone();

    check_button_letters_type_lowercase.connect_toggled(move |_e| {
        update_examples(&window_rules, None);
    });
    let window_rules = gui_data.window_rules.clone();
    check_button_letters_type_uppercase.connect_toggled(move |_e| {
        update_examples(&window_rules, None);
    });
    let window_rules = gui_data.window_rules.clone();
    check_button_letters_usage_extension.connect_toggled(move |_e| {
        update_examples(&window_rules, None);
    });
    let window_rules = gui_data.window_rules.clone();
    check_button_letters_usage_name.connect_toggled(move |_e| {
        update_examples(&window_rules, None);
    });
    let window_rules = gui_data.window_rules.clone();
    check_button_letters_usage_both.connect_toggled(move |_e| {
        update_examples(&window_rules, None);
    });
}
