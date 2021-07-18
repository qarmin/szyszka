use crate::class_gui_data::GuiData;
use crate::help_function::populate_rules_tree_view;
use crate::notebook_enum::{to_notebook_enum, NotebookEnum};
use crate::rules::{RuleData, RulePlace, RuleType};
use crate::update_records::{update_records, UpdateMode};
use gtk::prelude::*;
use std::ops::DerefMut;

pub fn connect_rule_add(gui_data: &GuiData) {
    let button_rule_window_add = gui_data.window_rules.button_rule_window_add.clone();
    let window_with_rules = gui_data.window_rules.window_with_rules.clone();
    let window_main = gui_data.window_main.clone();
    let notebook_choose_rule = gui_data.window_rules.notebook_choose_rule.clone();
    let tree_view_results = gui_data.results.tree_view_results.clone();
    let tree_view_window_rules = gui_data.rules_bottom_panel.tree_view_window_rules.clone();
    let shared_result_entries = gui_data.shared_result_entries.clone();
    let rules = gui_data.rules.clone();

    let label_files_folders = gui_data.upper_buttons.label_files_folders.clone();

    let window_rules = gui_data.window_rules.clone();

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

    button_rule_window_add.connect_clicked(move |_e| {
        window_with_rules.hide();
        window_main.set_sensitive(true);
        {
            let mut rule = rules.borrow_mut();
            let rule = rule.deref_mut();

            let rule_type: RuleType;
            let rule_place: RulePlace;
            let mut rule_data: RuleData = RuleData::new();
            let rule_description: String;

            match to_notebook_enum(notebook_choose_rule.current_page().unwrap()) {
                NotebookEnum::CaseSize => {
                    rule_type = RuleType::CaseSize;

                    rule_data.to_lowercase = true;
                    if radio_button_letters_type_uppercase.is_active() {
                        rule_data.to_lowercase = false;
                    } else if radio_button_letters_type_lowercase.is_active() {
                        rule_data.to_lowercase = true;
                    } else {
                        panic!("Button not available");
                    }
                    if radio_button_letters_usage_extension.is_active() {
                        rule_place = RulePlace::Extension;
                    } else if radio_button_letters_usage_both.is_active() {
                        rule_place = RulePlace::ExtensionAndName;
                    } else if radio_button_letters_usage_name.is_active() {
                        rule_place = RulePlace::Name;
                    } else {
                        panic!("Invalid Button Clicked");
                    }

                    let mut text = if rule_data.to_lowercase { "Lowercase".to_string() } else { "Uppercase".to_string() };
                    text.push_str(" text");
                    rule_description = text;
                }
                NotebookEnum::Purge => {
                    rule_type = RuleType::Purge;
                    if radio_button_purge_extension.is_active() {
                        rule_place = RulePlace::Extension;
                    } else if radio_button_purge_both.is_active() {
                        rule_place = RulePlace::ExtensionAndName;
                    } else if radio_button_purge_name.is_active() {
                        rule_place = RulePlace::Name;
                    } else {
                        panic!("Invalid Button Clicked");
                    }
                    rule_description = "".to_string();
                }
                NotebookEnum::AddText => {
                    rule_type = RuleType::AddText;
                    if radio_button_add_text_before_name.is_active() {
                        rule_place = RulePlace::BeforeName;
                    } else if radio_button_add_text_after_name.is_active() {
                        rule_place = RulePlace::AfterName;
                    } else {
                        panic!("Invalid Button Clicked");
                    }
                    rule_data.add_text_text = entry_add_text_text_to_add.text().to_string();
                    rule_description = format!("Added text: {}", rule_data.add_text_text);
                }
                NotebookEnum::Trim => {
                    rule_type = RuleType::Trim;

                    if radio_button_trim_case_sensitive.is_active() {
                        rule_data.case_sensitive = true;
                    } else if radio_button_trim_case_insensitive.is_active() {
                        rule_data.case_sensitive = false;
                    } else {
                        panic!("Invalid Button Clicked");
                    }

                    let where_remove;

                    if radio_button_trim_name_start.is_active() {
                        rule_place = RulePlace::FromNameStart;
                        where_remove = "start";
                    } else if radio_button_trim_name_end.is_active() {
                        rule_place = RulePlace::FromNameEndReverse;
                        where_remove = "end of name";
                    } else if radio_button_trim_extension_start.is_active() {
                        rule_place = RulePlace::FromExtensionStart;
                        where_remove = "extension";
                    } else if radio_button_trim_extension_end.is_active() {
                        rule_place = RulePlace::FromExtensionEndReverse;
                        where_remove = "end of extension";
                    } else {
                        panic!("Invalid Button Clicked");
                    }
                    rule_data.trim_text = entry_add_text_text_to_trim.text().to_string();
                    rule_description = format!("Trimming \"{}\" from {}", rule_data.trim_text, where_remove);
                }
                NotebookEnum::Custom => {
                    rule_type = RuleType::Custom;
                    rule_place = RulePlace::None;

                    rule_data.custom_text = entry_custom_text_to_change.text().to_string();
                    rule_description = format!("Custom rule: {}", rule_data.custom_text);
                }
                NotebookEnum::Replace => {
                    rule_type = RuleType::Replace;

                    if radio_button_replace_both.is_active() {
                        rule_place = RulePlace::ExtensionAndName;
                    } else if radio_button_replace_name.is_active() {
                        rule_place = RulePlace::Name;
                    } else if radio_button_replace_extension.is_active() {
                        rule_place = RulePlace::Extension;
                    } else {
                        panic!("Invalid Rule Type for purge rule");
                    }

                    if radio_button_replace_case_sensitive.is_active() {
                        rule_data.case_sensitive = true;
                    } else if radio_button_replace_case_insensitive.is_active() {
                        rule_data.case_sensitive = false;
                    } else {
                        panic!("Invalid Button Clicked");
                    }

                    rule_data.text_to_remove = entry_replace_text_to_remove.text().to_string();
                    rule_data.text_to_replace = entry_replace_text_to_change.text().to_string();
                    rule_description = format!("Replacing \"{}\" with \"{}\"", rule_data.text_to_remove, rule_data.text_to_replace);
                }
                NotebookEnum::AddNumber => {
                    rule_type = RuleType::AddNumber;

                    if radio_button_add_number_before_name.is_active() {
                        rule_place = RulePlace::BeforeName;
                    } else if radio_button_add_number_after_name.is_active() {
                        rule_place = RulePlace::AfterName;
                    } else {
                        panic!("Invalid Rule Type for purge rule");
                    }

                    rule_data.fill_with_zeros = entry_add_number_zeros.text().to_string().parse::<i64>().unwrap_or(0);
                    rule_data.number_step = entry_add_number_step.text().to_string().parse::<i64>().unwrap_or(1);
                    rule_data.number_start = entry_add_number_start_number.text().to_string().parse::<i64>().unwrap_or(1);

                    let zeros = if rule_data.fill_with_zeros > 0 {
                        format!(" and filling with {} zeros,", rule_data.fill_with_zeros)
                    } else {
                        "".to_string()
                    };
                    rule_description = format!("Starting with {} with step {}{}", rule_data.number_step, rule_data.number_start, zeros);
                }
            }
            if let Some(edit_mode) = rule.edit_mode {
                rule.rules[edit_mode].rule_type = rule_type;
                rule.rules[edit_mode].rule_place = rule_place;
                rule.rules[edit_mode].rule_description = rule_description;
                rule.rules[edit_mode].rule_data = rule_data;
            } else {
                rule.add_rule(rule_type, rule_place, rule_data, rule_description);
            }
        }

        // Reset TreeView and populate it again

        update_records(&tree_view_results, shared_result_entries.clone(), rules.clone(), UpdateMode::RuleAdded, &label_files_folders); // TODO Not only RuleAdded but also RuleEdited, but for now there is no difference
        populate_rules_tree_view(&tree_view_window_rules, rules.clone());
    });
}
