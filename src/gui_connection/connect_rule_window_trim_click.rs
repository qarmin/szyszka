use crate::gui_connection::common::{connect_examples_check_button, connect_examples_entry_name};
use crate::gui_data_things::gui_data::GuiData;

pub fn connect_rule_window_trim_click(gui_data: &GuiData) {
    let window_rules = gui_data.window_rules.clone();

    let check_button_trim_name_start = gui_data.window_rules.trim.check_button_trim_name_start.clone();
    let check_button_trim_name_end = gui_data.window_rules.trim.check_button_trim_name_end.clone();
    let check_button_trim_extension_start = gui_data.window_rules.trim.check_button_trim_extension_start.clone();
    let check_button_trim_extension_end = gui_data.window_rules.trim.check_button_trim_extension_end.clone();
    let check_button_trim_case_insensitive = gui_data.window_rules.trim.check_button_trim_case_insensitive.clone();
    let check_button_trim_case_sensitive = gui_data.window_rules.trim.check_button_trim_case_sensitive.clone();

    let entry_add_text_text_to_trim = gui_data.window_rules.trim.entry_add_text_text_to_trim.clone();

    connect_examples_check_button(&check_button_trim_name_start, &window_rules);
    connect_examples_check_button(&check_button_trim_name_end, &window_rules);
    connect_examples_check_button(&check_button_trim_extension_start, &window_rules);
    connect_examples_check_button(&check_button_trim_extension_end, &window_rules);
    connect_examples_check_button(&check_button_trim_case_insensitive, &window_rules);
    connect_examples_check_button(&check_button_trim_case_sensitive, &window_rules);
    connect_examples_entry_name(&entry_add_text_text_to_trim, &window_rules);
}
