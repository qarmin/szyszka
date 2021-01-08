use crate::class_dialog_rule_purge::GUIPurge;
use crate::class_gui_data::GuiData;
use crate::help_function::*;
use gtk::{ButtonExt, LabelExt, ToggleButtonExt};

pub fn connect_dialog_purge_click(gui_data: &GuiData) {
    let purge = gui_data.dialog_rules.purge.clone();
    let radio_button_purge_both = gui_data.dialog_rules.purge.radio_button_purge_both.clone();
    let radio_button_purge_name = gui_data.dialog_rules.purge.radio_button_purge_name.clone();
    let radio_button_purge_extension = gui_data.dialog_rules.purge.radio_button_purge_extension.clone();

    // Initial Update
    check_how_should_look_example(&purge);

    radio_button_purge_extension.connect_clicked(move |_e| {
        check_how_should_look_example(&purge);
    });
    let purge = gui_data.dialog_rules.purge.clone();
    radio_button_purge_name.connect_clicked(move |_e| {
        check_how_should_look_example(&purge);
    });
    let purge = gui_data.dialog_rules.purge.clone();
    radio_button_purge_both.connect_clicked(move |_e| {
        check_how_should_look_example(&purge);
    });
}

fn check_how_should_look_example(purge: &GUIPurge) {
    let labels_purge_example_before = purge.labels_purge_example_before.clone();
    let labels_purge_example_after = purge.labels_purge_example_after.clone();

    let radio_button_purge_both = purge.radio_button_purge_both.clone();
    let radio_button_purge_name = purge.radio_button_purge_name.clone();
    let radio_button_purge_extension = purge.radio_button_purge_extension.clone();

    labels_purge_example_before.set_text(EXAMPLE_NAME);

    let split = EXAMPLE_NAME.split('.').map(|e| e.to_string()).collect::<Vec<String>>();
    let name = split[0].clone();
    let extension = if split.len() > 1 { split[1].clone() } else { "".to_string() };
    let return_name;

    if radio_button_purge_both.get_active() {
        return_name = "".to_string();
    } else if radio_button_purge_name.get_active() {
        return_name = name;
    } else if radio_button_purge_extension.get_active() {
        return_name = extension;
    } else {
        panic!("Invalid Rule Type for purge rule");
    }

    labels_purge_example_after.set_text(return_name.as_str());
}
