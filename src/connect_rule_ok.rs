use crate::class_gui_data::GuiData;
use crate::help_function::populate_rules_tree_view;
use crate::notebook_enum::{to_notebook_enum, NotebookEnum};
use crate::rules::{RuleCaseSensitivity, RuleData, RulePlace, RuleType};
use gtk::prelude::*;
use gtk::{ButtonExt, WidgetExt};
use std::ops::DerefMut;

pub fn connect_rule_ok(gui_data: &GuiData) {
    let button_dialog_ok = gui_data.dialog_rules.button_dialog_ok.clone();
    let dialog_with_rules = gui_data.dialog_rules.dialog_with_rules.clone();
    let window_main = gui_data.window_main.clone();
    let notebook_choose_rule = gui_data.dialog_rules.notebook_choose_rule.clone();
    let rules = gui_data.rules.clone();
    let tree_view_window_rules = gui_data.rules_bottom_panel.tree_view_window_rules.clone();

    let dialog_rules = gui_data.dialog_rules.clone();

    let radio_button_letters_type_uppercase = dialog_rules.size_letters.radio_button_letters_type_uppercase.clone();
    let radio_button_letters_type_lowercase = dialog_rules.size_letters.radio_button_letters_type_lowercase.clone();
    let radio_button_letters_usage_name = dialog_rules.size_letters.radio_button_letters_usage_name.clone();
    let radio_button_letters_usage_extension = dialog_rules.size_letters.radio_button_letters_usage_extension.clone();
    let radio_button_letters_usage_both = dialog_rules.size_letters.radio_button_letters_usage_both.clone();

    let radio_button_purge_name = dialog_rules.purge.radio_button_purge_name.clone();
    let radio_button_purge_extension = dialog_rules.purge.radio_button_purge_extension.clone();
    let radio_button_purge_both = dialog_rules.purge.radio_button_purge_both.clone();

    let radio_button_add_text_after_name = dialog_rules.add_text.radio_button_add_text_after_name.clone();
    let radio_button_add_text_before_name = dialog_rules.add_text.radio_button_add_text_before_name.clone();
    let entry_add_text_text_to_add = dialog_rules.add_text.entry_add_text_text_to_add.clone();

    let radio_button_trim_name_start = dialog_rules.trim.radio_button_trim_name_start.clone();
    let radio_button_trim_name_end = dialog_rules.trim.radio_button_trim_name_end.clone();
    let radio_button_trim_extension_start = dialog_rules.trim.radio_button_trim_extension_start.clone();
    let radio_button_trim_extension_end = dialog_rules.trim.radio_button_trim_extension_end.clone();
    let radio_button_trim_case_insensitive = dialog_rules.trim.radio_button_trim_case_insensitive.clone();
    let radio_button_trim_case_sensitive = dialog_rules.trim.radio_button_trim_case_sensitive;

    button_dialog_ok.connect_clicked(move |_e| {
        dialog_with_rules.hide();
        window_main.set_sensitive(true);
        let mut rule = rules.borrow_mut();
        let rule = rule.deref_mut();

        let rule_type: RuleType;
        let rule_place: RulePlace;
        let mut rule_case_sensitivity = RuleCaseSensitivity::Sensitive;
        let mut rule_data: RuleData = RuleData::new();

        match to_notebook_enum(notebook_choose_rule.get_current_page().unwrap()) {
            NotebookEnum::CaseSize => {
                if radio_button_letters_type_uppercase.get_active() {
                    rule_type = RuleType::UpperCase;
                } else if radio_button_letters_type_lowercase.get_active() {
                    rule_type = RuleType::LowerCase;
                } else {
                    panic!("Button not available");
                }
                if radio_button_letters_usage_extension.get_active() {
                    rule_place = RulePlace::Extension;
                } else if radio_button_letters_usage_both.get_active() {
                    rule_place = RulePlace::ExtensionAndName;
                } else if radio_button_letters_usage_name.get_active() {
                    rule_place = RulePlace::Name;
                } else {
                    panic!("Invalid Button Clicked");
                }
            }
            NotebookEnum::Purge => {
                rule_type = RuleType::Purge;
                if radio_button_purge_extension.get_active() {
                    rule_place = RulePlace::Extension;
                } else if radio_button_purge_both.get_active() {
                    rule_place = RulePlace::ExtensionAndName;
                } else if radio_button_purge_name.get_active() {
                    rule_place = RulePlace::Name;
                } else {
                    panic!("Invalid Button Clicked");
                }
            }
            NotebookEnum::AddText => {
                rule_type = RuleType::AddText;
                if radio_button_add_text_after_name.get_active() {
                    rule_place = RulePlace::BeforeName;
                } else if radio_button_add_text_before_name.get_active() {
                    rule_place = RulePlace::AfterName;
                } else {
                    panic!("Invalid Button Clicked");
                }
                rule_data.add_text_text = entry_add_text_text_to_add.get_text().to_string();
            }
            NotebookEnum::Trim => {
                rule_type = RuleType::Trim;

                if radio_button_trim_case_sensitive.get_active() {
                    rule_case_sensitivity = RuleCaseSensitivity::Sensitive;
                } else if radio_button_trim_case_insensitive.get_active() {
                    rule_case_sensitivity = RuleCaseSensitivity::Insensitive;
                } else {
                    panic!("Invalid Button Clicked");
                }

                if radio_button_trim_name_start.get_active() {
                    rule_place = RulePlace::FromNameStart;
                } else if radio_button_trim_name_end.get_active() {
                    rule_place = RulePlace::FromNameEndReverse;
                } else if radio_button_trim_extension_start.get_active() {
                    rule_place = RulePlace::FromExtensionStart;
                } else if radio_button_trim_extension_end.get_active() {
                    rule_place = RulePlace::FromExtensionEndReverse;
                } else {
                    panic!("Invalid Button Clicked");
                }
            }

            _ => {
                panic!("Invalid notebook name");
            }
        }
        rule.add_rule(rule_type, rule_place, rule_case_sensitivity);

        // Reset TreeView and populate it again
        populate_rules_tree_view(&tree_view_window_rules, &rule);
    });
}
