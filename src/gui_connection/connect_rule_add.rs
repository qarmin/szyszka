use gtk4::prelude::*;

use crate::gui_data_things::gui_data::GuiData;
use crate::help_function::populate_rules_tree_view;
use crate::rule_read::read_rule_from_window;
use crate::update_records::{update_records, UpdateMode};

pub fn connect_rule_add(gui_data: &GuiData) {
    let button_rule_window_add = gui_data.window_rules.button_rule_window_add.clone();
    let window_with_rules = gui_data.window_rules.window_with_rules.clone();
    let window_main = gui_data.window_main.clone();
    let tree_view_results = gui_data.results.tree_view_results.clone();
    let tree_view_window_rules = gui_data.rules_bottom_panel.tree_view_window_rules.clone();
    let shared_result_entries = gui_data.shared_result_entries.clone();
    let rules = gui_data.rules.clone();

    let label_files_folders = gui_data.upper_buttons.label_files_folders.clone();
    let button_save_rules = gui_data.rules_bottom_panel.button_save_rules.clone();

    let window_rules = gui_data.window_rules.clone();

    button_rule_window_add.connect_clicked(move |_e| {
        window_with_rules.hide();
        window_main.set_sensitive(true);
        {
            let mut rule = rules.borrow_mut();
            let rule = &mut *rule;

            let Some(single_rule) = read_rule_from_window(&window_rules, None) else {
                return;
            };

            if let Some(edit_mode) = rule.edit_mode {
                rule.rules[edit_mode].rule_type = single_rule.rule_type;
                rule.rules[edit_mode].rule_place = single_rule.rule_place;
                rule.rules[edit_mode].rule_description = single_rule.rule_description;
                rule.rules[edit_mode].rule_data = single_rule.rule_data;
            } else {
                rule.add_single_rule(single_rule);
            }

            if !rule.rules.is_empty() {
                button_save_rules.set_sensitive(true);
            }
        }

        // Reset TreeView and populate it again

        update_records(&tree_view_results, &shared_result_entries, &rules, &UpdateMode::RuleAdded, &label_files_folders); // TODO Not only RuleAdded but also RuleEdited, but for now there is no difference
        populate_rules_tree_view(&tree_view_window_rules, &rules.borrow().rules);
    });
}
