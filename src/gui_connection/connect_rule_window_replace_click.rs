use crate::gui_connection::common::{connect_examples_check_button, connect_examples_entry_name};
use crate::gui_data_things::gui_data::GuiData;
use gtk4::prelude::*;

pub fn connect_rule_window_replace_click(gui_data: &GuiData) {
    let check_button_replace_name = gui_data.window_rules.replace.check_button_replace_name.clone();
    let check_button_replace_extension = gui_data.window_rules.replace.check_button_replace_extension.clone();
    let check_button_replace_both = gui_data.window_rules.replace.check_button_replace_both.clone();

    let check_button_replace_case_insensitive = gui_data.window_rules.replace.check_button_replace_case_insensitive.clone();
    let check_button_replace_case_sensitive = gui_data.window_rules.replace.check_button_replace_case_sensitive.clone();

    let check_button_replace_regex = gui_data.window_rules.replace.check_button_replace_regex.clone();

    let entry_replace_text_to_change = gui_data.window_rules.replace.entry_replace_text_to_change.clone();
    let entry_replace_text_to_find = gui_data.window_rules.replace.entry_replace_text_to_find.clone();

    connect_examples_check_button(&check_button_replace_name, &gui_data.window_rules);
    connect_examples_check_button(&check_button_replace_extension, &gui_data.window_rules);
    connect_examples_check_button(&check_button_replace_both, &gui_data.window_rules);
    connect_examples_check_button(&check_button_replace_case_sensitive, &gui_data.window_rules);
    connect_examples_check_button(&check_button_replace_case_insensitive, &gui_data.window_rules);
    connect_examples_check_button(&check_button_replace_regex, &gui_data.window_rules);

    connect_examples_entry_name(&entry_replace_text_to_find, &gui_data.window_rules);
    connect_examples_entry_name(&entry_replace_text_to_change, &gui_data.window_rules);

    check_button_replace_regex.connect_toggled(move |e| {
        let active = e.is_active();
        check_button_replace_case_sensitive.set_sensitive(!active);
        check_button_replace_case_insensitive.set_sensitive(!active);
        check_button_replace_name.set_sensitive(!active);
        check_button_replace_both.set_sensitive(!active);
        check_button_replace_extension.set_sensitive(!active);
    });
}
