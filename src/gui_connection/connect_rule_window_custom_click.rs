use crate::gui_connection::common::connect_examples_entry_name;
use crate::gui_data_things::gui_data::GuiData;

pub fn connect_rule_window_custom_click(gui_data: &GuiData) {
    let entry_custom_text_to_change = gui_data.window_rules.custom.entry_custom_text_to_change.clone();

    connect_examples_entry_name(&entry_custom_text_to_change, &gui_data.window_rules);
}
