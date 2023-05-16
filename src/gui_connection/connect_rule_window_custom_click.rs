use crate::config::get_custom_text_config_file;
use crate::gui_connection::common::connect_examples_entry_name;
use crate::gui_connection::connect_window_rules_open::create_custom_popovers;
use crate::gui_data_things::gui_data::GuiData;
use gtk4::prelude::*;
use std::cell::RefCell;
use std::fs;
use std::rc::Rc;

pub fn connect_rule_window_custom_click(gui_data: &GuiData) {
    let entry_custom_text_to_change = gui_data.window_rules.custom.entry_custom_text_to_change.clone();
    let window_with_rules = gui_data.window_rules.window_with_rules.clone();

    connect_examples_entry_name(&entry_custom_text_to_change, &gui_data.window_rules);

    let imported_custom_rules = gui_data.window_rules.custom.imported_custom_rules.clone();
    let button_save_custom_rule = gui_data.window_rules.custom.button_save_custom_rule.clone();
    entry_custom_text_to_change.connect_changed(move |e| {
        let imported_custom_rules = imported_custom_rules.clone();
        recalculate_entry_visibility(e, &imported_custom_rules, &button_save_custom_rule);
    });

    let button_save_custom_rule = gui_data.window_rules.custom.button_save_custom_rule.clone();
    let entry_custom_text_to_change = gui_data.window_rules.custom.entry_custom_text_to_change.clone();
    let imported_custom_rules = gui_data.window_rules.custom.imported_custom_rules.clone();
    window_with_rules.connect_show(move |_e| {
        let entry_custom_text_to_change = entry_custom_text_to_change.clone();
        let imported_custom_rules = imported_custom_rules.clone();
        recalculate_entry_visibility(&entry_custom_text_to_change, &imported_custom_rules, &button_save_custom_rule);
    });

    let button_save_custom_rule = gui_data.window_rules.custom.button_save_custom_rule.clone();
    let imported_custom_rules = gui_data.window_rules.custom.imported_custom_rules.clone();
    let entry_custom_text_to_change = gui_data.window_rules.custom.entry_custom_text_to_change.clone();
    let menu_button_load_custom_rule = gui_data.window_rules.custom.menu_button_load_custom_rule.clone();
    button_save_custom_rule.connect_clicked(move |e| {
        let entry_custom_text_to_change_cloned = entry_custom_text_to_change.clone();
        let imported_custom_rules_clone = imported_custom_rules.clone();
        save_custom_rule(e, &entry_custom_text_to_change_cloned, &imported_custom_rules_clone);

        let entry_custom_text_to_change_cloned = entry_custom_text_to_change.clone();
        let imported_custom_rules_clone = imported_custom_rules.clone();
        create_custom_popovers(&menu_button_load_custom_rule, &entry_custom_text_to_change_cloned, &imported_custom_rules_clone.borrow());
    });
}

pub fn save_custom_rule(button_save_custom_rule: &gtk4::Button, entry_custom_text_to_change: &gtk4::Entry, imported_custom_rules: &Rc<RefCell<Vec<String>>>) {
    let new_rule = entry_custom_text_to_change.text().as_str().trim().to_string();
    imported_custom_rules.borrow_mut().push(new_rule);

    if let Some(config_path) = get_custom_text_config_file() {
        let _ = fs::write(config_path, imported_custom_rules.borrow().join("\n"));
    }

    button_save_custom_rule.set_sensitive(false);
}

pub fn recalculate_entry_visibility(entry: &gtk4::Entry, imported_custom_rules: &Rc<RefCell<Vec<String>>>, button_save_custom_rule: &gtk4::Button) {
    let is_unique = imported_custom_rules.borrow().contains(&entry.text().as_str().trim().to_string());

    button_save_custom_rule.set_sensitive(!is_unique);
}
