use crate::class_gui_data::GuiData;
use crate::help_function::{get_list_store_from_tree_view, remove_selected_rows, ColumnsRules};
use crate::rules::{RulePlace, RuleType};
use crate::update_records::{update_records, UpdateMode};
use gtk::prelude::*;
use std::ops::DerefMut;

pub fn connect_rule_modify_add(gui_data: &GuiData) {
    let button_add_rule = gui_data.rules_bottom_panel.button_add_rule.clone();
    let window_with_rules = gui_data.window_rules.window_with_rules.clone();
    let window_main = gui_data.window_main.clone();

    let button_rule_window_add = gui_data.window_rules.button_rule_window_add.clone();

    button_add_rule.connect_clicked(move |_e| {
        // window_rules.set_position(WindowPosition::Center);
        button_rule_window_add.set_label("Add Rule");

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

pub fn connect_rule_modify_edit(gui_data: &GuiData) {
    let button_edit_rule = gui_data.rules_bottom_panel.button_edit_rule.clone();
    let window_rules = gui_data.window_rules.clone();
    let window_main = gui_data.window_main.clone();
    let window_with_rules = gui_data.window_rules.window_with_rules.clone();

    let rules = gui_data.rules.clone();

    let tree_view = gui_data.rules_bottom_panel.tree_view_window_rules.clone();

    let notebook_choose_rule = gui_data.window_rules.notebook_choose_rule.clone();
    let button_rule_window_add = gui_data.window_rules.button_rule_window_add.clone();

    let radio_button_letters_type_uppercase = window_rules.size_letters.radio_button_letters_type_uppercase.clone();
    let radio_button_letters_type_lowercase = window_rules.size_letters.radio_button_letters_type_lowercase.clone();
    let radio_button_letters_usage_name = window_rules.size_letters.radio_button_letters_usage_name.clone();
    let radio_button_letters_usage_extension = window_rules.size_letters.radio_button_letters_usage_extension.clone();
    let radio_button_letters_usage_both = window_rules.size_letters.radio_button_letters_usage_both.clone();

    let radio_button_purge_name = window_rules.purge.radio_button_purge_name.clone();
    let radio_button_purge_extension = window_rules.purge.radio_button_purge_extension.clone();
    let radio_button_purge_both = window_rules.purge.radio_button_purge_both.clone();

    let radio_button_add_text_after_name = window_rules.add_text.radio_button_add_text_after_name.clone();
    let radio_button_add_text_before_name = window_rules.add_text.radio_button_add_text_before_name.clone();
    let entry_add_text_text_to_add = window_rules.add_text.entry_add_text_text_to_add.clone();

    let entry_add_text_text_to_trim = window_rules.trim.entry_add_text_text_to_trim.clone();
    let radio_button_trim_name_start = window_rules.trim.radio_button_trim_name_start.clone();
    let radio_button_trim_name_end = window_rules.trim.radio_button_trim_name_end.clone();
    let radio_button_trim_extension_start = window_rules.trim.radio_button_trim_extension_start.clone();
    let radio_button_trim_extension_end = window_rules.trim.radio_button_trim_extension_end.clone();
    let radio_button_trim_case_insensitive = window_rules.trim.radio_button_trim_case_insensitive.clone();
    let radio_button_trim_case_sensitive = window_rules.trim.radio_button_trim_case_sensitive;

    let entry_custom_text_to_change = window_rules.custom.entry_custom_text_to_change.clone();

    let radio_button_replace_extension = window_rules.replace.radio_button_replace_extension.clone();
    let radio_button_replace_name = window_rules.replace.radio_button_replace_name.clone();
    let radio_button_replace_both = window_rules.replace.radio_button_replace_both.clone();
    let radio_button_replace_case_insensitive = window_rules.replace.radio_button_replace_case_insensitive.clone();
    let radio_button_replace_case_sensitive = window_rules.replace.radio_button_replace_case_sensitive.clone();
    let entry_replace_text_to_remove = window_rules.replace.entry_replace_text_to_remove.clone();
    let entry_replace_text_to_change = window_rules.replace.entry_replace_text_to_change;

    let radio_button_add_number_before_name = window_rules.add_number.radio_button_add_number_before_name.clone();
    let radio_button_add_number_after_name = window_rules.add_number.radio_button_add_number_after_name.clone();
    let entry_add_number_start_number = window_rules.add_number.entry_add_number_start_number.clone();
    let entry_add_number_step = window_rules.add_number.entry_add_number_step.clone();
    let entry_add_number_zeros = window_rules.add_number.entry_add_number_zeros;

    button_edit_rule.connect_clicked(move |_e| {
        let mut rules = rules.borrow_mut();
        let rules = rules.deref_mut();

        let selection = tree_view.selection();

        let (selected_rows, model) = selection.selected_rows();
        if selected_rows.is_empty() {
            return; // Nothing selected
        }
        let selected_item = selected_rows[0].clone();

        let mut item_number: usize = 0;

        let iter = model.iter_first().unwrap();

        loop {
            if model.path(&iter).unwrap() == selected_item {
                break;
            }

            if !model.iter_next(&iter) {
                panic!();
            }
            item_number += 1;
        }

        rules.edit_mode = Some(item_number);

        let rule = rules.rules[item_number].clone();
        let rule_data = rule.rule_data.clone();
        let rule_place = rule.rule_place.clone();
        let rule_type = rule.rule_type.clone();

        notebook_choose_rule.set_current_page(Some(rule.rule_type as u32));
        match rule_type {
            RuleType::CaseSize => {
                if rule_data.to_lowercase {
                    radio_button_letters_type_lowercase.set_active(true);
                } else {
                    radio_button_letters_type_uppercase.set_active(true);
                }

                if rule_place == RulePlace::Extension {
                    radio_button_letters_usage_extension.set_active(true);
                } else if rule_place == RulePlace::ExtensionAndName {
                    radio_button_letters_usage_both.set_active(true);
                } else if rule_place == RulePlace::Name {
                    radio_button_letters_usage_name.set_active(true);
                }
            }
            RuleType::Purge => {
                if rule_place == RulePlace::Extension {
                    radio_button_purge_extension.set_active(true);
                } else if rule_place == RulePlace::ExtensionAndName {
                    radio_button_purge_both.set_active(true);
                } else if rule_place == RulePlace::Name {
                    radio_button_purge_name.set_active(true);
                }
            }
            RuleType::AddText => {
                if rule_place == RulePlace::BeforeName {
                    radio_button_add_text_before_name.set_active(true);
                } else if rule_place == RulePlace::AfterName {
                    radio_button_add_text_after_name.set_active(true);
                }

                entry_add_text_text_to_add.set_text(rule_data.add_text_text.as_str());
            }
            RuleType::Trim => {
                if rule_place == RulePlace::FromNameStart {
                    radio_button_trim_name_start.set_active(true);
                } else if rule_place == RulePlace::FromNameEndReverse {
                    radio_button_trim_name_end.set_active(true);
                } else if rule_place == RulePlace::FromExtensionStart {
                    radio_button_trim_extension_start.set_active(true);
                } else if rule_place == RulePlace::FromExtensionEndReverse {
                    radio_button_trim_extension_end.set_active(true);
                }

                if rule_data.case_sensitive {
                    radio_button_trim_case_sensitive.set_active(true);
                } else {
                    radio_button_trim_case_insensitive.set_active(true);
                }

                entry_add_text_text_to_trim.set_text(rule_data.trim_text.as_str());
            }
            RuleType::Custom => {
                entry_custom_text_to_change.set_text(rule_data.custom_text.as_str());
            }
            RuleType::Replace => {
                if rule_place == RulePlace::ExtensionAndName {
                    radio_button_replace_both.set_active(true);
                } else if rule_place == RulePlace::Name {
                    radio_button_replace_name.set_active(true);
                } else if rule_place == RulePlace::Extension {
                    radio_button_replace_extension.set_active(true);
                }

                if rule_data.case_sensitive {
                    radio_button_replace_case_sensitive.set_active(true);
                } else {
                    radio_button_replace_case_insensitive.set_active(true);
                }

                entry_replace_text_to_remove.set_text(rule_data.text_to_remove.as_str());
                entry_replace_text_to_change.set_text(rule_data.text_to_replace.as_str());
            }
            RuleType::AddNumber => {
                if rule_place == RulePlace::BeforeName {
                    radio_button_add_number_before_name.set_active(true);
                } else if rule_place == RulePlace::AfterName {
                    radio_button_add_number_after_name.set_active(true);
                }

                entry_add_number_zeros.set_text(rule_data.fill_with_zeros.to_string().as_str());
                entry_add_number_step.set_text(rule_data.number_step.to_string().as_str());
                entry_add_number_start_number.set_text(rule_data.number_start.to_string().as_str());
            }
        }

        button_rule_window_add.set_label("Edit Rule");

        window_with_rules.show();
        window_main.set_sensitive(false);
    });
}
