use crate::class_gui_data::GuiData;
use crate::example_fields::update_examples;
use gtk::ButtonExt;

pub fn connect_rule_window_size_letters_click(gui_data: &GuiData) {
    let window_rules = gui_data.window_rules.clone();
    let radio_button_letters_usage_both = gui_data.window_rules.size_letters.radio_button_letters_usage_both.clone();
    let radio_button_letters_usage_name = gui_data.window_rules.size_letters.radio_button_letters_usage_name.clone();
    let radio_button_letters_usage_extension = gui_data.window_rules.size_letters.radio_button_letters_usage_extension.clone();
    let radio_button_letters_type_lowercase = gui_data.window_rules.size_letters.radio_button_letters_type_lowercase.clone();
    let radio_button_letters_type_uppercase = gui_data.window_rules.size_letters.radio_button_letters_type_uppercase.clone();

    radio_button_letters_type_lowercase.connect_clicked(move |_e| {
        update_examples(&window_rules, None);
    });
    let window_rules = gui_data.window_rules.clone();
    radio_button_letters_type_uppercase.connect_clicked(move |_e| {
        update_examples(&window_rules, None);
    });
    let window_rules = gui_data.window_rules.clone();
    radio_button_letters_usage_extension.connect_clicked(move |_e| {
        update_examples(&window_rules, None);
    });
    let window_rules = gui_data.window_rules.clone();
    radio_button_letters_usage_name.connect_clicked(move |_e| {
        update_examples(&window_rules, None);
    });
    let window_rules = gui_data.window_rules.clone();
    radio_button_letters_usage_both.connect_clicked(move |_e| {
        update_examples(&window_rules, None);
    });
}
