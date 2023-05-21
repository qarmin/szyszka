#![allow(clippy::manual_let_else)]
// TODO check if this really not broke code

use gtk4::prelude::*;

use crate::fls;
use crate::gui_data_things::class_dialog_rules::GuiDialogRules;
use crate::localizer::generate_translation_hashmap;
use crate::notebook_enum::{to_notebook_enum, NotebookEnum};
use crate::rule::rules::{RuleData, RulePlace, RuleType, SingleRule};

// Notebook number point to current notebook tab
// In normal use this isn't problem, but notebook_choose_rule.current_page() points to invalid
// notebook when changing pages
pub fn read_rule_from_window(window_rules: &GuiDialogRules, notebook_number: Option<u32>) -> Option<SingleRule> {
    let notebook_choose_rule = window_rules.notebook_choose_rule.clone();

    let mut rule_data: RuleData = RuleData::new();

    let notebook_enum = if let Some(notebook_number) = notebook_number {
        to_notebook_enum(notebook_number)
    } else {
        to_notebook_enum(notebook_choose_rule.current_page().unwrap())
    };

    let (rule_type, rule_place, rule_description) = match match notebook_enum {
        NotebookEnum::CaseSize => read_rule_case_size(window_rules, &mut rule_data),
        NotebookEnum::Purge => read_rule_purge(window_rules, &mut rule_data),
        NotebookEnum::AddText => read_rule_add_text(window_rules, &mut rule_data),
        NotebookEnum::Trim => read_rule_trim(window_rules, &mut rule_data),
        NotebookEnum::Custom => read_rule_custom(window_rules, &mut rule_data),
        NotebookEnum::Replace => read_rule_replace(window_rules, &mut rule_data),
        NotebookEnum::AddNumber => read_rule_add_number(window_rules, &mut rule_data),
        NotebookEnum::Normalize => read_rule_normalize(window_rules, &mut rule_data),
    } {
        Some(t) => t,
        None => return None,
    };

    Some(SingleRule {
        rule_type,
        rule_place,
        rule_data,
        rule_description,
    })
}

fn read_rule_replace(window_rules: &GuiDialogRules, rule_data: &mut RuleData) -> Option<(RuleType, RulePlace, String)> {
    let rule_type = RuleType::Replace;
    let rule_place: RulePlace;
    let rule_description: String;

    let check_button_replace_extension = window_rules.replace.check_button_replace_extension.clone();
    let check_button_replace_name = window_rules.replace.check_button_replace_name.clone();
    let check_button_replace_both = window_rules.replace.check_button_replace_both.clone();
    let check_button_replace_case_insensitive = window_rules.replace.check_button_replace_case_insensitive.clone();
    let check_button_replace_case_sensitive = window_rules.replace.check_button_replace_case_sensitive.clone();
    let check_button_replace_regex = window_rules.replace.check_button_replace_regex.clone();
    let entry_replace_text_to_find = window_rules.replace.entry_replace_text_to_find.clone();
    let entry_replace_text_to_change = window_rules.replace.entry_replace_text_to_change.clone();
    let check_button_replace_replace_all = window_rules.replace.check_button_replace_replace_all.clone();

    if check_button_replace_regex.is_active() {
        rule_place = RulePlace::None;
    } else if check_button_replace_both.is_active() {
        rule_place = RulePlace::ExtensionAndName;
    } else if check_button_replace_name.is_active() {
        rule_place = RulePlace::Name;
    } else if check_button_replace_extension.is_active() {
        rule_place = RulePlace::Extension;
    } else {
        return None;
    }

    if check_button_replace_case_sensitive.is_active() {
        rule_data.case_sensitive = true;
    } else if check_button_replace_case_insensitive.is_active() {
        rule_data.case_sensitive = false;
    } else {
        return None;
    }

    rule_data.use_regex = check_button_replace_regex.is_active();
    rule_data.regex_replace_all = check_button_replace_replace_all.is_active();

    rule_data.text_to_find = entry_replace_text_to_find.text().to_string();
    rule_data.text_to_replace = entry_replace_text_to_change.text().to_string();
    let additional_regex_text = if check_button_replace_regex.is_active() { " regex" } else { "" };

    rule_description = fls!(
        "rule_description_replace",
        generate_translation_hashmap(vec![
            ("text_to_find", rule_data.text_to_find.clone()),
            ("text_to_replace", rule_data.text_to_replace.clone()),
            ("additional_regex_text", additional_regex_text.to_string()),
        ])
    );

    Some((rule_type, rule_place, rule_description))
}

#[allow(clippy::unnecessary_wraps)]
fn read_rule_custom(window_rules: &GuiDialogRules, rule_data: &mut RuleData) -> Option<(RuleType, RulePlace, String)> {
    let rule_type = RuleType::Custom;
    let rule_place: RulePlace;
    let rule_description: String;
    rule_place = RulePlace::None;

    let entry_custom_text_to_change = window_rules.custom.entry_custom_text_to_change.clone();

    rule_data.custom_text = entry_custom_text_to_change.text().to_string();
    rule_description = fls!("rule_description_custom_rule", generate_translation_hashmap(vec![("custom_rule", rule_data.custom_text.clone())]));

    Some((rule_type, rule_place, rule_description))
}

fn read_rule_trim(window_rules: &GuiDialogRules, rule_data: &mut RuleData) -> Option<(RuleType, RulePlace, String)> {
    let rule_type = RuleType::Trim;
    let rule_place: RulePlace;
    let rule_description: String;

    let entry_add_text_text_to_trim = window_rules.trim.entry_add_text_text_to_trim.clone();
    let check_button_trim_name_start = window_rules.trim.check_button_trim_name_start.clone();
    let check_button_trim_name_end = window_rules.trim.check_button_trim_name_end.clone();
    let check_button_trim_extension_start = window_rules.trim.check_button_trim_extension_start.clone();
    let check_button_trim_extension_end = window_rules.trim.check_button_trim_extension_end.clone();
    let check_button_trim_case_insensitive = window_rules.trim.check_button_trim_case_insensitive.clone();
    let check_button_trim_case_sensitive = window_rules.trim.check_button_trim_case_sensitive.clone();

    if check_button_trim_case_sensitive.is_active() {
        rule_data.case_sensitive = true;
    } else if check_button_trim_case_insensitive.is_active() {
        rule_data.case_sensitive = false;
    } else {
        return None;
    }

    let where_remove;

    if check_button_trim_name_start.is_active() {
        rule_place = RulePlace::FromNameStart;
        where_remove = fls!("rule_description_start");
    } else if check_button_trim_name_end.is_active() {
        rule_place = RulePlace::FromNameEndReverse;
        where_remove = fls!("rule_description_end_of_name");
    } else if check_button_trim_extension_start.is_active() {
        rule_place = RulePlace::FromExtensionStart;
        where_remove = fls!("rule_description_extension");
    } else if check_button_trim_extension_end.is_active() {
        rule_place = RulePlace::FromExtensionEndReverse;
        where_remove = fls!("rule_description_end_of_extension");
    } else {
        return None;
    }
    rule_data.trim_text = entry_add_text_text_to_trim.text().to_string();
    rule_description = fls!("rule_description_trimming", generate_translation_hashmap(vec![("trim_text", rule_data.trim_text.clone()), ("where_remove", where_remove)]));

    Some((rule_type, rule_place, rule_description))
}

fn read_rule_add_text(window_rules: &GuiDialogRules, rule_data: &mut RuleData) -> Option<(RuleType, RulePlace, String)> {
    let rule_type = RuleType::AddText;
    let rule_place: RulePlace;
    let rule_description: String;

    let check_button_add_text_after_name = window_rules.add_text.check_button_add_text_after_name.clone();
    let check_button_add_text_before_name = window_rules.add_text.check_button_add_text_before_name.clone();
    let entry_add_text_text_to_add = window_rules.add_text.entry_add_text_text_to_add.clone();

    if check_button_add_text_before_name.is_active() {
        rule_place = RulePlace::BeforeName;
    } else if check_button_add_text_after_name.is_active() {
        rule_place = RulePlace::AfterName;
    } else {
        return None;
    }
    rule_data.add_text_text = entry_add_text_text_to_add.text().to_string();
    rule_description = format!("{} {}", fls!("rule_description_added_text"), rule_data.add_text_text);

    Some((rule_type, rule_place, rule_description))
}

fn read_rule_purge(window_rules: &GuiDialogRules, _rule_data: &mut RuleData) -> Option<(RuleType, RulePlace, String)> {
    let rule_type = RuleType::Purge;
    let rule_place: RulePlace;
    let rule_description: String;

    let check_button_purge_name = window_rules.purge.check_button_purge_name.clone();
    let check_button_purge_extension = window_rules.purge.check_button_purge_extension.clone();
    let check_button_purge_both = window_rules.purge.check_button_purge_both.clone();

    if check_button_purge_extension.is_active() {
        rule_place = RulePlace::Extension;
    } else if check_button_purge_both.is_active() {
        rule_place = RulePlace::ExtensionAndName;
    } else if check_button_purge_name.is_active() {
        rule_place = RulePlace::Name;
    } else {
        return None;
    }
    rule_description = String::new();

    Some((rule_type, rule_place, rule_description))
}

fn read_rule_case_size(window_rules: &GuiDialogRules, rule_data: &mut RuleData) -> Option<(RuleType, RulePlace, String)> {
    let rule_type = RuleType::CaseSize;
    let rule_place: RulePlace;
    let rule_description: String;

    let check_button_letters_type_uppercase = window_rules.size_letters.check_button_letters_type_uppercase.clone();
    let check_button_letters_type_lowercase = window_rules.size_letters.check_button_letters_type_lowercase.clone();
    let check_button_letters_usage_name = window_rules.size_letters.check_button_letters_usage_name.clone();
    let check_button_letters_usage_extension = window_rules.size_letters.check_button_letters_usage_extension.clone();
    let check_button_letters_usage_both = window_rules.size_letters.check_button_letters_usage_both.clone();

    rule_data.to_lowercase = true;
    if check_button_letters_type_uppercase.is_active() {
        rule_data.to_lowercase = false;
    } else if check_button_letters_type_lowercase.is_active() {
        rule_data.to_lowercase = true;
    } else {
        return None;
    }
    if check_button_letters_usage_extension.is_active() {
        rule_place = RulePlace::Extension;
    } else if check_button_letters_usage_both.is_active() {
        rule_place = RulePlace::ExtensionAndName;
    } else if check_button_letters_usage_name.is_active() {
        rule_place = RulePlace::Name;
    } else {
        return None;
    }

    let text = if rule_data.to_lowercase {
        format!("{} {}", fls!("rule_description_lowercase"), fls!("rule_description_text"))
    } else {
        format!("{} {}", fls!("rule_description_uppercase"), fls!("rule_description_text"))
    };

    rule_description = text;
    Some((rule_type, rule_place, rule_description))
}

fn read_rule_add_number(window_rules: &GuiDialogRules, rule_data: &mut RuleData) -> Option<(RuleType, RulePlace, String)> {
    let rule_type = RuleType::AddNumber;
    let rule_place: RulePlace;
    let rule_description: String;

    let check_button_add_number_before_name = window_rules.add_number.check_button_add_number_before_name.clone();
    let check_button_add_number_after_name = window_rules.add_number.check_button_add_number_after_name.clone();
    let entry_add_number_start_number = window_rules.add_number.entry_add_number_start_number.clone();
    let entry_add_number_step = window_rules.add_number.entry_add_number_step.clone();
    let entry_add_number_zeros = window_rules.add_number.entry_add_number_zeros.clone();

    if check_button_add_number_before_name.is_active() {
        rule_place = RulePlace::BeforeName;
    } else if check_button_add_number_after_name.is_active() {
        rule_place = RulePlace::AfterName;
    } else {
        return None;
    }

    rule_data.fill_with_zeros = entry_add_number_zeros.text().to_string().parse::<i64>().unwrap_or(0);
    rule_data.number_step = entry_add_number_step.text().to_string().parse::<i64>().unwrap_or(1);
    rule_data.number_start = entry_add_number_start_number.text().to_string().parse::<i64>().unwrap_or(1);

    let zeros = if rule_data.fill_with_zeros > 0 {
        format!(" {}", fls!("rule_description_zeros", generate_translation_hashmap(vec![("zeros", rule_data.fill_with_zeros.to_string())])))
    } else {
        String::new()
    };
    rule_description = fls!(
        "rule_description_step",
        generate_translation_hashmap(vec![("step", rule_data.number_step.to_string()), ("start", rule_data.number_start.to_string()), ("zeros", zeros)])
    );

    Some((rule_type, rule_place, rule_description))
}

fn read_rule_normalize(window_rules: &GuiDialogRules, rule_data: &mut RuleData) -> Option<(RuleType, RulePlace, String)> {
    let rule_type = RuleType::Normalize;
    let rule_place = RulePlace::ExtensionAndName;
    let rule_description;

    let check_button_normalize_everything = window_rules.normalize.check_button_normalize_everything.clone();
    let check_button_normalize_partial = window_rules.normalize.check_button_normalize_partial.clone();

    if check_button_normalize_everything.is_active() {
        rule_data.full_normalize = true;
    } else if check_button_normalize_partial.is_active() {
        rule_data.full_normalize = false;
    } else {
        return None;
    }

    if rule_data.full_normalize {
        rule_description = fls!("rule_description_full_normalize");
    } else {
        rule_description = fls!("rule_description_partial_normalize");
    }
    Some((rule_type, rule_place, rule_description))
}
