use crate::class_dialog_rules::DialogRules;
use crate::class_gui_data::GuiData;
use crate::help_function::*;
use gtk::{ButtonExt, LabelExt, ToggleButtonExt};

pub fn connect_dialog_size_letters_click(gui_data: &GuiData) {
    let dialog_rules = gui_data.dialog_rules.clone();
    let radio_button_letters_usage_both = gui_data.dialog_rules.radio_button_letters_usage_both.clone();
    let radio_button_letters_usage_name = gui_data.dialog_rules.radio_button_letters_usage_name.clone();
    let radio_button_letters_usage_extension = gui_data.dialog_rules.radio_button_letters_usage_extension.clone();
    let radio_button_letters_type_lowercase = gui_data.dialog_rules.radio_button_letters_type_lowercase.clone();
    let radio_button_letters_type_uppercase = gui_data.dialog_rules.radio_button_letters_type_uppercase.clone();

    // Initial Update
    check_how_should_look_example(&dialog_rules);

    radio_button_letters_type_lowercase.connect_clicked(move |_e| {
        check_how_should_look_example(&dialog_rules);
    });
    let dialog_rules = gui_data.dialog_rules.clone();
    radio_button_letters_type_uppercase.connect_clicked(move |_e| {
        check_how_should_look_example(&dialog_rules);
    });
    let dialog_rules = gui_data.dialog_rules.clone();
    radio_button_letters_usage_extension.connect_clicked(move |_e| {
        check_how_should_look_example(&dialog_rules);
    });
    let dialog_rules = gui_data.dialog_rules.clone();
    radio_button_letters_usage_name.connect_clicked(move |_e| {
        check_how_should_look_example(&dialog_rules);
    });
    let dialog_rules = gui_data.dialog_rules.clone();
    radio_button_letters_usage_both.connect_clicked(move |_e| {
        check_how_should_look_example(&dialog_rules);
    });
}

fn check_how_should_look_example(dialog_rules: &DialogRules) {
    let labels_letters_example_before = dialog_rules.labels_letters_example_before.clone();
    let labels_letters_example_after = dialog_rules.labels_letters_example_after.clone();

    let radio_button_letters_usage_both = dialog_rules.radio_button_letters_usage_both.clone();
    let radio_button_letters_usage_name = dialog_rules.radio_button_letters_usage_name.clone();
    let radio_button_letters_usage_extension = dialog_rules.radio_button_letters_usage_extension.clone();
    let radio_button_letters_type_lowercase = dialog_rules.radio_button_letters_type_lowercase.clone();
    let radio_button_letters_type_uppercase = dialog_rules.radio_button_letters_type_uppercase.clone();

    labels_letters_example_before.set_text(EXAMPLE_NAME);

    let split = EXAMPLE_NAME.split('.').map(|e| e.to_string()).collect::<Vec<String>>();
    let mut name = split[0].clone();
    let mut extension = split[1].clone();

    if radio_button_letters_type_uppercase.get_active() {
        if radio_button_letters_usage_both.get_active() {
            name = name.to_uppercase();
            extension = extension.to_uppercase();
        } else if radio_button_letters_usage_name.get_active() {
            name = name.to_uppercase();
        } else if radio_button_letters_usage_extension.get_active() {
            extension = extension.to_uppercase();
        } else {
            panic!("Missing radio button");
        }
    } else if radio_button_letters_type_lowercase.get_active() {
        if radio_button_letters_usage_both.get_active() {
            name = name.to_lowercase();
            extension = extension.to_lowercase();
        } else if radio_button_letters_usage_name.get_active() {
            name = name.to_lowercase();
        } else if radio_button_letters_usage_extension.get_active() {
            extension = extension.to_lowercase();
        } else {
            panic!("Missing radio button");
        }
    } else {
        panic!("Missing radio button");
    }
    labels_letters_example_after.set_text(format!("{}.{}", name, extension).as_str());
}
