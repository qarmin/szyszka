use crate::class_dialog_rule_remove::GUIRemove;
use crate::class_gui_data::GuiData;
use crate::help_function::*;
use gtk::{ButtonExt, LabelExt, ToggleButtonExt};

pub fn connect_dialog_remove_click(gui_data: &GuiData) {
    let remove = gui_data.dialog_rules.remove.clone();
    let radio_button_remove_both = gui_data.dialog_rules.remove.radio_button_remove_both.clone();
    let radio_button_remove_name = gui_data.dialog_rules.remove.radio_button_remove_name.clone();
    let radio_button_remove_extension = gui_data.dialog_rules.remove.radio_button_remove_extension.clone();

    // Initial Update
    check_how_should_look_example(&remove);

    radio_button_remove_extension.connect_clicked(move |_e| {
        check_how_should_look_example(&remove);
    });
    let remove = gui_data.dialog_rules.remove.clone();
    radio_button_remove_name.connect_clicked(move |_e| {
        check_how_should_look_example(&remove);
    });
    let remove = gui_data.dialog_rules.remove.clone();
    radio_button_remove_both.connect_clicked(move |_e| {
        check_how_should_look_example(&remove);
    });
}

fn check_how_should_look_example(remove: &GUIRemove) {
    let labels_remove_example_before = remove.labels_remove_example_before.clone();
    let labels_remove_example_after = remove.labels_remove_example_after.clone();

    let radio_button_remove_both = remove.radio_button_remove_both.clone();
    let radio_button_remove_name = remove.radio_button_remove_name.clone();
    let radio_button_remove_extension = remove.radio_button_remove_extension.clone();

    labels_remove_example_before.set_text(EXAMPLE_NAME);

    let split = EXAMPLE_NAME.split('.').map(|e| e.to_string()).collect::<Vec<String>>();
    let name = split[0].clone();
    let extension = if split.len() > 1 { split[1].clone() } else { "".to_string() };
    let return_name;

    if radio_button_remove_both.get_active() {
        return_name = "".to_string();
    } else if radio_button_remove_name.get_active() {
        return_name = name;
    } else if radio_button_remove_extension.get_active() {
        return_name = extension;
    } else {
        panic!("Invalid Rule Type for remove rule");
    }

    labels_remove_example_after.set_text(return_name.as_str());
}
