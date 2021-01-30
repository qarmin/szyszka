use crate::class_gui_data::GuiData;
use crate::example_fields::update_examples;
use gtk::{ButtonExt, EditableSignals};

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
    entry_add_text_text_to_add.connect_changed(move |_e| {
        update_examples(&window_rules, None);
    });
}
