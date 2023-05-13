use gtk4::prelude::*;

use crate::example_fields::update_examples;
use crate::gui_data_things::gui_data::GuiData;

pub fn connect_rule_window_normalize_click(gui_data: &GuiData) {
    let window_rules = gui_data.window_rules.clone();

    let check_button_normalize_everything = gui_data.window_rules.normalize.check_button_normalize_everything.clone();
    let check_button_normalize_partial = gui_data.window_rules.normalize.check_button_normalize_partial.clone();

    check_button_normalize_everything.connect_toggled(move |_e| {
        update_examples(&window_rules, None);
    });
    let window_rules = gui_data.window_rules.clone();
    check_button_normalize_partial.connect_toggled(move |_e| {
        update_examples(&window_rules, None);
    });
}
