use gtk4::prelude::*;

use crate::example_fields::update_examples;
use crate::gui_data::GuiData;

pub fn connect_rule_window_purge_click(gui_data: &GuiData) {
    let window_rules = gui_data.window_rules.clone();

    let check_button_purge_both = gui_data.window_rules.purge.check_button_purge_both.clone();
    let check_button_purge_name = gui_data.window_rules.purge.check_button_purge_name.clone();
    let check_button_purge_extension = gui_data.window_rules.purge.check_button_purge_extension.clone();

    check_button_purge_extension.connect_toggled(move |_e| {
        update_examples(&window_rules, None);
    });
    let window_rules = gui_data.window_rules.clone();
    check_button_purge_name.connect_toggled(move |_e| {
        update_examples(&window_rules, None);
    });
    let window_rules = gui_data.window_rules.clone();
    check_button_purge_both.connect_toggled(move |_e| {
        update_examples(&window_rules, None);
    });
}
