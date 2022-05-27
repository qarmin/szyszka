use crate::example_fields::update_examples;
use crate::gui_data::GuiData;
use crate::help_function::validate_number;
use gtk4::prelude::*;

pub fn connect_rule_window_add_number_click(gui_data: &GuiData) {
    let window_rules = gui_data.window_rules.clone();

    let check_button_add_number_before_name = gui_data.window_rules.add_number.check_button_add_number_before_name.clone();
    let check_button_add_number_after_name = gui_data.window_rules.add_number.check_button_add_number_after_name.clone();

    let entry_add_number_start_number = gui_data.window_rules.add_number.entry_add_number_start_number.clone();
    let entry_add_number_step = gui_data.window_rules.add_number.entry_add_number_step.clone();
    let entry_add_number_zeros = gui_data.window_rules.add_number.entry_add_number_zeros.clone();

    // TODO GTK 4
    // check_button_add_number_before_name.connect_clicked(move |_e| {
    //     update_examples(&window_rules, None);
    // });
    // let window_rules = gui_data.window_rules.clone();
    // check_button_add_number_after_name.connect_clicked(move |_e| {
    //     update_examples(&window_rules, None);
    // });

    let window_rules = gui_data.window_rules.clone();
    entry_add_number_start_number.connect_changed(move |e| {
        e.set_text(validate_number(e.text().to_string()).as_str());
        update_examples(&window_rules, None);
    });
    let window_rules = gui_data.window_rules.clone();
    entry_add_number_step.connect_changed(move |e| {
        e.set_text(validate_number(e.text().to_string()).as_str());
        update_examples(&window_rules, None);
    });
    let window_rules = gui_data.window_rules.clone();
    entry_add_number_zeros.connect_changed(move |e| {
        e.set_text(validate_number(e.text().to_string()).as_str());
        update_examples(&window_rules, None);
    });
}
