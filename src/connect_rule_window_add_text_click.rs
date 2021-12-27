use crate::example_fields::update_examples;
use crate::gui_data::GuiData;
use crate::help_function::validate_name;
use gtk::prelude::*;

pub fn connect_rule_window_add_text_click(gui_data: &GuiData) {
    let window_rules = gui_data.window_rules.clone();

    let radio_button_add_text_before_name = gui_data.window_rules.add_text.radio_button_add_text_before_name.clone();
    let radio_button_add_text_after_name = gui_data.window_rules.add_text.radio_button_add_text_after_name.clone();

    let entry_add_text_text_to_add = gui_data.window_rules.add_text.entry_add_text_text_to_add.clone();

    radio_button_add_text_before_name.connect_clicked(move |_e| {
        update_examples(&window_rules, None);
    });
    let window_rules = gui_data.window_rules.clone();
    radio_button_add_text_after_name.connect_clicked(move |_e| {
        update_examples(&window_rules, None);
    });
    let window_rules = gui_data.window_rules.clone();
    entry_add_text_text_to_add.connect_changed(move |e| {
        e.set_text(validate_name(e.text().to_string()).as_str());
        update_examples(&window_rules, None);
    });
}
