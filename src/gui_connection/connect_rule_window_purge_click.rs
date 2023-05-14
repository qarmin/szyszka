use crate::gui_connection::common::connect_examples_check_button;
use crate::gui_data_things::gui_data::GuiData;

pub fn connect_rule_window_purge_click(gui_data: &GuiData) {
    let check_button_purge_both = gui_data.window_rules.purge.check_button_purge_both.clone();
    let check_button_purge_name = gui_data.window_rules.purge.check_button_purge_name.clone();
    let check_button_purge_extension = gui_data.window_rules.purge.check_button_purge_extension.clone();

    connect_examples_check_button(&check_button_purge_extension, &gui_data.window_rules);
    connect_examples_check_button(&check_button_purge_name, &gui_data.window_rules);
    connect_examples_check_button(&check_button_purge_both, &gui_data.window_rules);
}
