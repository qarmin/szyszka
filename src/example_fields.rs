use crate::class_dialog_rules::GUIDialogRules;
use crate::class_gui_data::GuiData;
use crate::notebook_enum::{to_notebook_enum, NotebookEnum, EXAMPLE_NAME};
use crate::rule_add_text::rule_add_text;
use crate::rule_change_size_letters::rule_change_size_letters;
use crate::rule_purge::rule_purge;
use crate::rule_trim::rule_trim;
use crate::rules::{RuleData, RulePlace, RuleType};
use gtk::prelude::*;

pub fn connect_update_examples(gui_data: &GuiData) {
    let notebook_choose_rule = gui_data.window_rules.notebook_choose_rule.clone();

    let button_example_reset = gui_data.window_rules.button_example_reset.clone();

    let window_rules = gui_data.window_rules.clone();
    notebook_choose_rule.connect_switch_page(move |_e, _y, z| {
        update_examples(&window_rules, Some(z));
    });

    let entry_example_before = gui_data.window_rules.entry_example_before.clone();
    button_example_reset.connect_clicked(move |_e| {
        entry_example_before.set_text(EXAMPLE_NAME);
    });

    let window_rules = gui_data.window_rules.clone();
    let entry_example_before = gui_data.window_rules.entry_example_before.clone();
    entry_example_before.connect_changed(move |_e| {
        update_examples(&window_rules, None);
    });
}

pub fn update_examples(window_rules: &GUIDialogRules, notebook_number: Option<u32>) {
    let notebook_choose_rule = window_rules.notebook_choose_rule.clone();

    let text_to_change: String = window_rules.entry_example_before.get_text().to_string();
    let label_example_after = window_rules.label_example_after.clone();

    let rule_type: RuleType;
    let rule_place: RulePlace;
    let mut rule_data: RuleData = RuleData::new();

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

    let radio_button_trim_name_start = window_rules.trim.radio_button_trim_name_start.clone();
    let radio_button_trim_name_end = window_rules.trim.radio_button_trim_name_end.clone();
    let radio_button_trim_extension_start = window_rules.trim.radio_button_trim_extension_start.clone();
    let radio_button_trim_extension_end = window_rules.trim.radio_button_trim_extension_end.clone();
    let radio_button_trim_case_insensitive = window_rules.trim.radio_button_trim_case_insensitive.clone();
    let radio_button_trim_case_sensitive = window_rules.trim.radio_button_trim_case_sensitive.clone();
    let entry_add_text_text_to_trim = window_rules.trim.entry_add_text_text_to_trim.clone();

    let notebook_enum = if let Some(notebook_number) = notebook_number {
        to_notebook_enum(notebook_number)
    } else {
        to_notebook_enum(notebook_choose_rule.get_current_page().unwrap())
    };

    match notebook_enum {
        NotebookEnum::CaseSize => {
            rule_type = RuleType::CaseSize;

            if radio_button_letters_type_uppercase.get_active() {
                rule_data.to_lowercase = false;
            } else if radio_button_letters_type_lowercase.get_active() {
                rule_data.to_lowercase = true;
            } else {
                panic!("Missing radio button");
            }

            if radio_button_letters_usage_both.get_active() {
                rule_place = RulePlace::ExtensionAndName;
            } else if radio_button_letters_usage_name.get_active() {
                rule_place = RulePlace::Name;
            } else if radio_button_letters_usage_extension.get_active() {
                rule_place = RulePlace::Extension;
            } else {
                panic!("Missing radio button");
            }

            label_example_after.set_text(rule_change_size_letters(text_to_change.as_str(), &rule_type, &rule_place, &rule_data).as_str());
        }
        NotebookEnum::AddText => {
            rule_data.add_text_text = entry_add_text_text_to_add.get_text().to_string();
            rule_type = RuleType::AddText;
            if radio_button_add_text_before_name.get_active() {
                rule_place = RulePlace::BeforeName;
            } else if radio_button_add_text_after_name.get_active() {
                rule_place = RulePlace::AfterName;
            } else {
                panic!("Invalid Rule Type for add_text rule");
            }

            label_example_after.set_text(rule_add_text(text_to_change.as_str(), &rule_type, &rule_place, &rule_data).as_str());
        }
        NotebookEnum::Purge => {
            rule_type = RuleType::Purge;

            if radio_button_purge_both.get_active() {
                rule_place = RulePlace::ExtensionAndName;
            } else if radio_button_purge_name.get_active() {
                rule_place = RulePlace::Name;
            } else if radio_button_purge_extension.get_active() {
                rule_place = RulePlace::Extension;
            } else {
                panic!("Invalid Rule Type for purge rule");
            }

            label_example_after.set_text(rule_purge(text_to_change.as_str(), &rule_type, &rule_place).as_str());
        }
        NotebookEnum::Trim => {
            rule_type = RuleType::Trim;
            rule_data.trim_text = entry_add_text_text_to_trim.get_text().to_string();
            if radio_button_trim_case_sensitive.get_active() {
                rule_data.case_sensitive = true;
            } else if radio_button_trim_case_insensitive.get_active() {
                rule_data.case_sensitive = false;
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
            label_example_after.set_text(rule_trim(text_to_change.as_str(), &rule_type, &rule_place, &rule_data).as_str());
        }
        _ => {
            println!("Missing notebook Handler!, Needs to fix");
        }
    }
}
