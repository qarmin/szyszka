use crate::class_gui_data::GuiData;
use crate::example_fields::update_examples;
use crate::help_function::validate_name;
use gtk::{ButtonExt, EditableSignals, EntryExt};

pub fn connect_rule_window_trim_click(gui_data: &GuiData) {
    let window_rules = gui_data.window_rules.clone();

    let radio_button_trim_name_start = gui_data.window_rules.trim.radio_button_trim_name_start.clone();
    let radio_button_trim_name_end = gui_data.window_rules.trim.radio_button_trim_name_end.clone();
    let radio_button_trim_extension_start = gui_data.window_rules.trim.radio_button_trim_extension_start.clone();
    let radio_button_trim_extension_end = gui_data.window_rules.trim.radio_button_trim_extension_end.clone();
    let radio_button_trim_case_insensitive = gui_data.window_rules.trim.radio_button_trim_case_insensitive.clone();
    let radio_button_trim_case_sensitive = gui_data.window_rules.trim.radio_button_trim_case_sensitive.clone();

    let entry_add_text_text_to_trim = gui_data.window_rules.trim.entry_add_text_text_to_trim.clone();

    radio_button_trim_name_start.connect_clicked(move |_e| {
        update_examples(&window_rules, None);
    });
    let window_rules = gui_data.window_rules.clone();
    radio_button_trim_name_end.connect_clicked(move |_e| {
        update_examples(&window_rules, None);
    });
    let window_rules = gui_data.window_rules.clone();
    radio_button_trim_extension_start.connect_clicked(move |_e| {
        update_examples(&window_rules, None);
    });
    let window_rules = gui_data.window_rules.clone();
    radio_button_trim_extension_end.connect_clicked(move |_e| {
        update_examples(&window_rules, None);
    });
    let window_rules = gui_data.window_rules.clone();
    radio_button_trim_case_insensitive.connect_clicked(move |_e| {
        update_examples(&window_rules, None);
    });
    let window_rules = gui_data.window_rules.clone();
    radio_button_trim_case_sensitive.connect_clicked(move |_e| {
        update_examples(&window_rules, None);
    });
    let window_rules = gui_data.window_rules.clone();
    entry_add_text_text_to_trim.connect_changed(move |e| {
        e.set_text(validate_name(e.get_text().to_string()).as_str());
        update_examples(&window_rules, None);
    });
}
