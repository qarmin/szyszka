use crate::gui_connection::common::connect_examples_check_button;
use crate::gui_data_things::gui_data::GuiData;

pub fn connect_rule_window_normalize_click(gui_data: &GuiData) {
    let check_button_normalize_everything = gui_data.window_rules.normalize.check_button_normalize_everything.clone();
    let check_button_normalize_partial = gui_data.window_rules.normalize.check_button_normalize_partial.clone();

    connect_examples_check_button(&check_button_normalize_everything, &gui_data.window_rules);
    connect_examples_check_button(&check_button_normalize_partial, &gui_data.window_rules);
}
