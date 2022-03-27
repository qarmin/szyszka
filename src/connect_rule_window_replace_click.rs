use crate::example_fields::update_examples;
use crate::gui_data::GuiData;
use crate::help_function::validate_name;
use gtk::prelude::*;

pub fn connect_rule_window_replace_click(gui_data: &GuiData) {
    let window_rules = gui_data.window_rules.clone();

    let radio_button_replace_name = gui_data.window_rules.replace.radio_button_replace_name.clone();
    let radio_button_replace_extension = gui_data.window_rules.replace.radio_button_replace_extension.clone();
    let radio_button_replace_both = gui_data.window_rules.replace.radio_button_replace_both.clone();

    let radio_button_replace_case_insensitive = gui_data.window_rules.replace.radio_button_replace_case_insensitive.clone();
    let radio_button_replace_case_sensitive = gui_data.window_rules.replace.radio_button_replace_case_sensitive.clone();

    let entry_replace_text_to_change = gui_data.window_rules.replace.entry_replace_text_to_change.clone();
    let entry_replace_text_to_remove = gui_data.window_rules.replace.entry_replace_text_to_remove.clone();

    radio_button_replace_name.connect_clicked(move |_e| {
        update_examples(&window_rules, None);
    });
    let window_rules = gui_data.window_rules.clone();
    radio_button_replace_extension.connect_clicked(move |_e| {
        update_examples(&window_rules, None);
    });
    let window_rules = gui_data.window_rules.clone();
    radio_button_replace_both.connect_clicked(move |_e| {
        update_examples(&window_rules, None);
    });

    let window_rules = gui_data.window_rules.clone();
    radio_button_replace_case_sensitive.connect_clicked(move |_e| {
        update_examples(&window_rules, None);
    });
    let window_rules = gui_data.window_rules.clone();
    radio_button_replace_case_insensitive.connect_clicked(move |_e| {
        update_examples(&window_rules, None);
    });

    let window_rules = gui_data.window_rules.clone();
    entry_replace_text_to_remove.connect_changed(move |e| {
        e.set_text(validate_name(e.text().to_string()).as_str());
        update_examples(&window_rules, None);
    });
    let window_rules = gui_data.window_rules.clone();
    entry_replace_text_to_change.connect_changed(move |e| {
        e.set_text(validate_name(e.text().to_string()).as_str());
        update_examples(&window_rules, None);
    });
}
