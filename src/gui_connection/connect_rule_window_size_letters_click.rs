use crate::gui_connection::common::connect_examples_check_button;
use crate::gui_data_things::gui_data::GuiData;

pub fn connect_rule_window_size_letters_click(gui_data: &GuiData) {
    let check_button_letters_usage_both = gui_data.window_rules.size_letters.check_button_letters_usage_both.clone();
    let check_button_letters_usage_name = gui_data.window_rules.size_letters.check_button_letters_usage_name.clone();
    let check_button_letters_usage_extension = gui_data.window_rules.size_letters.check_button_letters_usage_extension.clone();
    let check_button_letters_type_lowercase = gui_data.window_rules.size_letters.check_button_letters_type_lowercase.clone();
    let check_button_letters_type_uppercase = gui_data.window_rules.size_letters.check_button_letters_type_uppercase.clone();

    connect_examples_check_button(&check_button_letters_type_lowercase, &gui_data.window_rules);
    connect_examples_check_button(&check_button_letters_type_uppercase, &gui_data.window_rules);
    connect_examples_check_button(&check_button_letters_usage_extension, &gui_data.window_rules);
    connect_examples_check_button(&check_button_letters_usage_name, &gui_data.window_rules);
    connect_examples_check_button(&check_button_letters_usage_both, &gui_data.window_rules);
}
