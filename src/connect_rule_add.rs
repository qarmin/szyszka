use crate::class_gui_data::GuiData;
use crate::help_function::{populate_rules_tree_view, read_rule_from_window};
use crate::update_records::{update_records, UpdateMode};
use gtk::prelude::*;
use std::ops::DerefMut;

pub fn connect_rule_add(gui_data: &GuiData) {
    let button_rule_window_add = gui_data.window_rules.button_rule_window_add.clone();
    let window_with_rules = gui_data.window_rules.window_with_rules.clone();
    let window_main = gui_data.window_main.clone();
    let tree_view_results = gui_data.results.tree_view_results.clone();
    let tree_view_window_rules = gui_data.rules_bottom_panel.tree_view_window_rules.clone();
    let shared_result_entries = gui_data.shared_result_entries.clone();
    let rules = gui_data.rules.clone();

    let label_files_folders = gui_data.upper_buttons.label_files_folders.clone();

    let window_rules = gui_data.window_rules.clone();

    button_rule_window_add.connect_clicked(move |_e| {
        window_with_rules.hide();
        window_main.set_sensitive(true);
        {
            let mut rule = rules.borrow_mut();
            let rule = rule.deref_mut();

            let single_rule = read_rule_from_window(&window_rules, None);

            if let Some(edit_mode) = rule.edit_mode {
                rule.rules[edit_mode].rule_type = single_rule.rule_type;
                rule.rules[edit_mode].rule_place = single_rule.rule_place;
                rule.rules[edit_mode].rule_description = single_rule.rule_description;
                rule.rules[edit_mode].rule_data = single_rule.rule_data;
            } else {
                rule.add_single_rule(single_rule);
            }
        }

        // Reset TreeView and populate it again

        update_records(&tree_view_results, shared_result_entries.clone(), rules.clone(), UpdateMode::RuleAdded, &label_files_folders); // TODO Not only RuleAdded but also RuleEdited, but for now there is no difference
        populate_rules_tree_view(&tree_view_window_rules, rules.clone());
    });
}
