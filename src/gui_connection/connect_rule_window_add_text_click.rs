use crate::gui_connection::common::{connect_examples_check_button, connect_examples_entry_name};
use crate::gui_data_things::gui_data::GuiData;

pub fn connect_rule_window_add_text_click(gui_data: &GuiData) {
    let check_button_add_text_before_name = gui_data.window_rules.add_text.check_button_add_text_before_name.clone();
    let check_button_add_text_after_name = gui_data.window_rules.add_text.check_button_add_text_after_name.clone();

    let entry_add_text_text_to_add = gui_data.window_rules.add_text.entry_add_text_text_to_add.clone();

    connect_examples_check_button(&check_button_add_text_before_name, &gui_data.window_rules);
    connect_examples_check_button(&check_button_add_text_after_name, &gui_data.window_rules);

    connect_examples_entry_name(&entry_add_text_text_to_add, &gui_data.window_rules);
}
