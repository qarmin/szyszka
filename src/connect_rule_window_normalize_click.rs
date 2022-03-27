use crate::example_fields::update_examples;
use crate::gui_data::GuiData;
use gtk::prelude::*;

pub fn connect_rule_window_normalize_click(gui_data: &GuiData) {
    let window_rules = gui_data.window_rules.clone();

    let radio_button_normalize_everything = gui_data.window_rules.normalize.radio_button_normalize_everything.clone();
    let radio_button_normalize_partial = gui_data.window_rules.normalize.radio_button_normalize_partial.clone();

    radio_button_normalize_everything.connect_clicked(move |_e| {
        update_examples(&window_rules, None);
    });
    let window_rules = gui_data.window_rules.clone();
    radio_button_normalize_partial.connect_clicked(move |_e| {
        update_examples(&window_rules, None);
    });
}
