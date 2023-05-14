use crate::gui_connection::common::{connect_examples_check_button, connect_examples_entry_name};
use crate::gui_data_things::gui_data::GuiData;

pub fn connect_rule_window_replace_click(gui_data: &GuiData) {
    let check_button_replace_name = gui_data.window_rules.replace.check_button_replace_name.clone();
    let check_button_replace_extension = gui_data.window_rules.replace.check_button_replace_extension.clone();
    let check_button_replace_both = gui_data.window_rules.replace.check_button_replace_both.clone();

    let check_button_replace_case_insensitive = gui_data.window_rules.replace.check_button_replace_case_insensitive.clone();
    let check_button_replace_case_sensitive = gui_data.window_rules.replace.check_button_replace_case_sensitive.clone();

    let entry_replace_text_to_change = gui_data.window_rules.replace.entry_replace_text_to_change.clone();
    let entry_replace_text_to_remove = gui_data.window_rules.replace.entry_replace_text_to_remove.clone();

    connect_examples_check_button(&check_button_replace_name, &gui_data.window_rules);
    connect_examples_check_button(&check_button_replace_extension, &gui_data.window_rules);
    connect_examples_check_button(&check_button_replace_both, &gui_data.window_rules);
    connect_examples_check_button(&check_button_replace_case_sensitive, &gui_data.window_rules);
    connect_examples_check_button(&check_button_replace_case_insensitive, &gui_data.window_rules);

    connect_examples_entry_name(&entry_replace_text_to_remove, &gui_data.window_rules);
    connect_examples_entry_name(&entry_replace_text_to_change, &gui_data.window_rules);
}
