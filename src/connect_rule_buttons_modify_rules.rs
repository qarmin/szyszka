use crate::class_gui_data::GuiData;
use crate::help_function::{get_list_store_from_tree_view, remove_selected_rows, ColumnsRules};
use crate::update_records::{update_records, UpdateMode};
use gtk::prelude::*;
use std::ops::DerefMut;

pub fn connect_rule_modify_add(gui_data: &GuiData) {
    let button_add_rule = gui_data.rules_bottom_panel.button_add_rule.clone();
    let window_with_rules = gui_data.window_rules.window_with_rules.clone();
    let window_main = gui_data.window_main.clone();

    button_add_rule.connect_clicked(move |_e| {
        // window_rules.set_position(WindowPosition::Center);
        window_with_rules.show();
        window_main.set_sensitive(false);
    });
}

pub fn connect_rule_modify_remove(gui_data: &GuiData) {
    let button_remove_rule = gui_data.rules_bottom_panel.button_remove_rule.clone();
    let tree_view_results = gui_data.results.tree_view_results.clone();
    let tree_view_window_rules = gui_data.rules_bottom_panel.tree_view_window_rules.clone();
    let shared_result_entries = gui_data.shared_result_entries.clone();
    let rules = gui_data.rules.clone();

    let label_files_folders = gui_data.upper_buttons.label_files_folders.clone();

    // Multiselection Ready
    button_remove_rule.connect_clicked(move |_e| {
        let vec_rule_to_delete = remove_selected_rows(&tree_view_window_rules);

        {
            let mut rules = rules.borrow_mut();
            let rules = rules.deref_mut();

            for rule_to_delete in vec_rule_to_delete.iter().rev() {
                rules.remove_rule(*rule_to_delete);
            }
        }

        update_records(&tree_view_results, shared_result_entries.clone(), rules.clone(), UpdateMode::RuleRemoved, &label_files_folders);
    });
}

pub fn connect_rule_modify_one_up(gui_data: &GuiData) {
    let button_rule_one_up = gui_data.rules_bottom_panel.button_rule_one_up.clone();
    let tree_view_results = gui_data.results.tree_view_results.clone();
    let tree_view_window_rules = gui_data.rules_bottom_panel.tree_view_window_rules.clone();
    let shared_result_entries = gui_data.shared_result_entries.clone();
    let rules = gui_data.rules.clone();

    let label_files_folders = gui_data.upper_buttons.label_files_folders.clone();

    // TODO only works with single selection
    button_rule_one_up.connect_clicked(move |_e| {
        let selection = tree_view_window_rules.selection();
        if selection.selected_rows().0.is_empty() {
            return;
        }

        {
            let mut rules = rules.borrow_mut();
            let rules = rules.deref_mut();
            let list_store = get_list_store_from_tree_view(&tree_view_window_rules);

            let (selected_rows, _tree_model) = selection.selected_rows();

            let first_iter = list_store.iter_first().unwrap();
            let first_path = list_store.path(&first_iter).unwrap();

            if selected_rows.iter().any(|selected_path| *selected_path == first_path) {
                return; // First thing is selected - this works only in single selection mode
            }

            let current_iter = first_iter.clone();
            let mut previous_iter = first_iter;

            if list_store.iter_next(&current_iter) {
                let mut current_index = 0;
                loop {
                    current_index += 1;
                    let current_path = list_store.path(&current_iter).unwrap();

                    let found = selected_rows.iter().any(|selected_path| *selected_path == current_path);

                    if found {
                        break;
                    }

                    previous_iter = current_iter.clone();
                    if !list_store.iter_next(&current_iter) {
                        // break;
                        panic!("");
                    }
                }
                // Swap rules
                {
                    rules.rules.swap(current_index, current_index - 1);
                    let previous_type = list_store.value(&previous_iter, ColumnsRules::RuleType as i32).get::<String>().unwrap();
                    let previous_usage = list_store.value(&previous_iter, ColumnsRules::UsageType as i32).get::<String>().unwrap();
                    let previous_description = list_store.value(&previous_iter, ColumnsRules::Description as i32).get::<String>().unwrap();

                    let next_type = list_store.value(&current_iter, ColumnsRules::RuleType as i32).get::<String>().unwrap();
                    let next_usage = list_store.value(&current_iter, ColumnsRules::UsageType as i32).get::<String>().unwrap();
                    let next_description = list_store.value(&current_iter, ColumnsRules::Description as i32).get::<String>().unwrap();

                    list_store.set_value(&current_iter, ColumnsRules::RuleType as u32, &previous_type.to_value());
                    list_store.set_value(&current_iter, ColumnsRules::UsageType as u32, &previous_usage.to_value());
                    list_store.set_value(&current_iter, ColumnsRules::Description as u32, &previous_description.to_value());

                    list_store.set_value(&previous_iter, ColumnsRules::RuleType as u32, &next_type.to_value());
                    list_store.set_value(&previous_iter, ColumnsRules::UsageType as u32, &next_usage.to_value());
                    list_store.set_value(&previous_iter, ColumnsRules::Description as u32, &next_description.to_value());
                }
                selection.select_iter(&previous_iter);
            } else {
                return;
            }
        }
        update_records(&tree_view_results, shared_result_entries.clone(), rules.clone(), UpdateMode::RuleRemoved, &label_files_folders);
    });
}
pub fn connect_rule_modify_one_down(gui_data: &GuiData) {
    let button_rule_one_down = gui_data.rules_bottom_panel.button_rule_one_down.clone();
    let tree_view_results = gui_data.results.tree_view_results.clone();
    let tree_view_window_rules = gui_data.rules_bottom_panel.tree_view_window_rules.clone();
    let shared_result_entries = gui_data.shared_result_entries.clone();
    let rules = gui_data.rules.clone();

    let label_files_folders = gui_data.upper_buttons.label_files_folders.clone();

    // TODO only works with single selection
    button_rule_one_down.connect_clicked(move |_e| {
        let selection = tree_view_window_rules.selection();
        if selection.selected_rows().0.is_empty() {
            return;
        }

        {
            let mut rules = rules.borrow_mut();
            let rules = rules.deref_mut();
            let list_store = get_list_store_from_tree_view(&tree_view_window_rules);

            let (selected_rows, _tree_model) = selection.selected_rows();

            let first_iter = list_store.iter_first().unwrap();

            let current_iter;
            let previous_iter = first_iter;

            let mut current_index = 0;
            loop {
                current_index += 1;
                let current_path = list_store.path(&previous_iter).unwrap();

                let found = selected_rows.iter().any(|selected_path| *selected_path == current_path);

                if found {
                    break;
                }

                if !list_store.iter_next(&previous_iter) {
                    panic!();
                }
            }

            current_iter = previous_iter.clone();
            if !list_store.iter_next(&current_iter) {
                return;
                // Latest element
            }

            // Swap rules
            {
                rules.rules.swap(current_index, current_index - 1);
                let previous_type = list_store.value(&previous_iter, ColumnsRules::RuleType as i32).get::<String>().unwrap();
                let previous_usage = list_store.value(&previous_iter, ColumnsRules::UsageType as i32).get::<String>().unwrap();
                let previous_description = list_store.value(&previous_iter, ColumnsRules::Description as i32).get::<String>().unwrap();

                let next_type = list_store.value(&current_iter, ColumnsRules::RuleType as i32).get::<String>().unwrap();
                let next_usage = list_store.value(&current_iter, ColumnsRules::UsageType as i32).get::<String>().unwrap();
                let next_description = list_store.value(&current_iter, ColumnsRules::Description as i32).get::<String>().unwrap();

                list_store.set_value(&current_iter, ColumnsRules::RuleType as u32, &previous_type.to_value());
                list_store.set_value(&current_iter, ColumnsRules::UsageType as u32, &previous_usage.to_value());
                list_store.set_value(&current_iter, ColumnsRules::Description as u32, &previous_description.to_value());

                list_store.set_value(&previous_iter, ColumnsRules::RuleType as u32, &next_type.to_value());
                list_store.set_value(&previous_iter, ColumnsRules::UsageType as u32, &next_usage.to_value());
                list_store.set_value(&previous_iter, ColumnsRules::Description as u32, &next_description.to_value());
            }
            selection.select_iter(&current_iter);
        }
        update_records(&tree_view_results, shared_result_entries.clone(), rules.clone(), UpdateMode::RuleRemoved, &label_files_folders);
    });
}
