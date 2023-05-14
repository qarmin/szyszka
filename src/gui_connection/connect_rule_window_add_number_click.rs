use crate::gui_connection::common::{connect_examples_check_button, connect_examples_entry_number};
use crate::gui_data_things::gui_data::GuiData;

pub fn connect_rule_window_add_number_click(gui_data: &GuiData) {
    let check_button_add_number_before_name = gui_data.window_rules.add_number.check_button_add_number_before_name.clone();
    let check_button_add_number_after_name = gui_data.window_rules.add_number.check_button_add_number_after_name.clone();

    let entry_add_number_start_number = gui_data.window_rules.add_number.entry_add_number_start_number.clone();
    let entry_add_number_step = gui_data.window_rules.add_number.entry_add_number_step.clone();
    let entry_add_number_zeros = gui_data.window_rules.add_number.entry_add_number_zeros.clone();

    connect_examples_check_button(&check_button_add_number_before_name, &gui_data.window_rules);
    connect_examples_check_button(&check_button_add_number_after_name, &gui_data.window_rules);

    connect_examples_entry_number(&entry_add_number_start_number, &gui_data.window_rules);
    connect_examples_entry_number(&entry_add_number_step, &gui_data.window_rules);
    connect_examples_entry_number(&entry_add_number_zeros, &gui_data.window_rules);
}
