use crate::class_dialog_rules::GuiDialogRules;
use crate::notebook_enum::{to_notebook_enum, NotebookEnum};
use crate::rules::*;
use gtk::prelude::*;
use gtk::*;
use std::cell::RefCell;
use std::ops::DerefMut;
use std::path::Path;
use std::rc::Rc;

#[allow(dead_code)]
pub enum ColumnsResults {
    Type = 0,
    CurrentName,
    FutureName,
    Path,
    Size,
    ModificationDate,
    CreationDate,
}
pub enum ColumnsRules {
    //RuleNumber = 0,
    RuleType = 0,
    UsageType,
    Description,
}

#[cfg(target_family = "windows")]
pub static CHARACTER: &str = "\\";

#[cfg(not(target_family = "windows"))]
pub static CHARACTER: &str = "/";

pub fn validate_name(before_name: String) -> String {
    // TODO when trying to print text in middle of text, then caret change position, fix it

    let mut after_name = before_name;
    after_name = after_name.replace("\\", "");
    if cfg!(target_family = "windows") {
        after_name = after_name.replace("/", "");
        after_name = after_name.replace("<", "");
        after_name = after_name.replace(">", "");
        after_name = after_name.replace(":", "");
        after_name = after_name.replace("", "");
        after_name = after_name.replace("|", "");
        after_name = after_name.replace("?", "");
        after_name = after_name.replace("*", "");
    }
    after_name
}
pub fn validate_number(before_name: String) -> String {
    before_name.chars().filter(|e| e.is_digit(10)).collect::<String>()
}

pub fn split_path(path: &Path) -> (String, String) {
    match (path.parent(), path.file_name()) {
        (Some(dir), Some(file)) => (dir.display().to_string(), file.to_string_lossy().into_owned()),
        (Some(dir), None) => (dir.display().to_string(), String::new()),
        (None, _) => (String::new(), String::new()),
    }
}
pub fn split_file_name(path: &Path) -> (String, String) {
    match (path.file_stem(), path.extension()) {
        (Some(name), Some(extension)) => (name.to_string_lossy().to_string(), extension.to_string_lossy().to_string()),
        (Some(name), None) => (name.to_string_lossy().to_string(), String::new()),
        (None, _) => (String::new(), String::new()),
    }
}

pub fn get_list_store_from_tree_view(tree_view: &TreeView) -> ListStore {
    tree_view.model().unwrap().downcast::<gtk::ListStore>().unwrap()
}

pub fn populate_rules_tree_view(tree_view: &gtk::TreeView, rules: Rc<RefCell<Rules>>) {
    let mut rules = rules.borrow_mut();
    let rules = rules.deref_mut();

    let list_store = get_list_store_from_tree_view(&tree_view);

    list_store.clear();

    for rule in &rules.rules {
        let values: [(u32, &dyn ToValue); 3] = [
            (ColumnsRules::RuleType as u32, &rule_type_to_string(&rule.rule_type)),
            (ColumnsRules::UsageType as u32, &rule_place_to_string(&rule.rule_place)),
            (ColumnsRules::Description as u32, &rule.rule_description),
        ];
        list_store.set(&list_store.append(), &values);
    }
}

pub fn remove_selected_rows(tree_view: &gtk::TreeView) -> Vec<usize> {
    let selection = tree_view.selection();

    let (selected_rows, _tree_model) = selection.selected_rows();

    // Nothing selected
    if selected_rows.is_empty() {
        return Vec::new();
    }

    let list_store = get_list_store_from_tree_view(&tree_view);

    let mut vec_index_to_delete: Vec<_> = Vec::new();
    let mut current_iter: usize = 0;

    let tree_iter = list_store.iter_first().unwrap();

    // Get indexes of removed values
    for selected_tree_path in &selected_rows {
        loop {
            if list_store.path(&tree_iter).unwrap() == *selected_tree_path {
                vec_index_to_delete.push(current_iter);
                list_store.iter_next(&tree_iter);
                current_iter += 1;
                break;
            }
            list_store.iter_next(&tree_iter);
            current_iter += 1;
        }
    }

    // Remove selected rows
    for selected_tree_path in selected_rows.iter().rev() {
        list_store.remove(&list_store.iter(selected_tree_path).unwrap());
    }
    vec_index_to_delete
}
pub fn get_full_file_names_from_selection(tree_view: &gtk::TreeView) -> Vec<String> {
    let selection = tree_view.selection();

    let (selected_rows, _tree_model) = selection.selected_rows();

    let mut return_vec = Vec::with_capacity(selected_rows.len());

    // Nothing selected
    if selected_rows.is_empty() {
        return return_vec;
    }

    let list_store = get_list_store_from_tree_view(&tree_view);

    // Get indexes of removed values
    for selected_tree_path in &selected_rows {
        let tree_iter = list_store.iter(selected_tree_path).unwrap();
        return_vec.push(format!(
            "{}{}{}",
            list_store.value(&tree_iter, ColumnsResults::Path as i32).get::<String>().unwrap(),
            CHARACTER,
            list_store.value(&tree_iter, ColumnsResults::CurrentName as i32).get::<String>().unwrap()
        ));
    }

    return_vec
}

pub fn count_rows_in_tree_view(tree_view: &gtk::TreeView) -> u32 {
    let list_store = get_list_store_from_tree_view(&tree_view);
    let mut number = 0;

    if let Some(curr_iter) = list_store.iter_first() {
        loop {
            number += 1;
            if !list_store.iter_next(&curr_iter) {
                break;
            }
        }
    }

    number
}

pub fn create_message_window(window_main: &gtk::Window, title: &str, message: &str) {
    let chooser = gtk::Dialog::with_buttons(Some(title), Some(window_main), DialogFlags::DESTROY_WITH_PARENT, &[("Ok", gtk::ResponseType::Ok)]);

    let question_label = gtk::Label::new(Some(message));

    let chooser_box = chooser.children()[0].clone().downcast::<gtk::Box>().unwrap();
    chooser_box.add(&question_label);
    chooser_box.show_all();

    chooser.run();
    chooser.close();
}
pub fn regex_check(expression: &str, directory: impl AsRef<Path>) -> bool {
    let temp_splits: Vec<&str> = expression.split('*').collect();
    let mut splits: Vec<&str> = Vec::new();
    for i in temp_splits {
        if !i.is_empty() {
            splits.push(i);
        }
    }
    if splits.is_empty() {
        return false;
    }

    // Get rid of non unicode characters
    let directory = directory.as_ref().to_string_lossy();

    // Early checking if directory contains all parts needed by expression
    for split in &splits {
        if !directory.contains(split) {
            return false;
        }
    }

    let mut position_of_splits: Vec<usize> = Vec::new();

    // `git*` shouldn't be true for `/gitsfafasfs`
    if !expression.starts_with('*') && directory.find(&splits[0]).unwrap() > 0 {
        return false;
    }
    // `*home` shouldn't be true for `/homeowner`
    if !expression.ends_with('*') && !directory.ends_with(splits.last().unwrap()) {
        return false;
    }

    // At the end we check if parts between * are correctly positioned
    position_of_splits.push(directory.find(&splits[0]).unwrap());
    let mut current_index: usize;
    let mut found_index: usize;
    for i in splits[1..].iter().enumerate() {
        current_index = *position_of_splits.get(i.0).unwrap() + i.1.len();
        found_index = match directory[current_index..].find(i.1) {
            Some(t) => t,
            None => return false,
        };
        position_of_splits.push(found_index + current_index);
    }
    true
}
// Notebook number point to current notebook tab
// In normal use this isn't problem, but notebook_choose_rule.current_page() points to invalid
// notebook when changing pages
pub fn read_rule_from_window(window_rules: &GuiDialogRules, notebook_number: Option<u32>) -> SingleRule {
    let notebook_choose_rule = window_rules.notebook_choose_rule.clone();

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
    let radio_button_trim_case_sensitive = window_rules.trim.radio_button_trim_case_sensitive.clone();

    let entry_custom_text_to_change = window_rules.custom.entry_custom_text_to_change.clone();

    let radio_button_replace_extension = window_rules.replace.radio_button_replace_extension.clone();
    let radio_button_replace_name = window_rules.replace.radio_button_replace_name.clone();
    let radio_button_replace_both = window_rules.replace.radio_button_replace_both.clone();
    let radio_button_replace_case_insensitive = window_rules.replace.radio_button_replace_case_insensitive.clone();
    let radio_button_replace_case_sensitive = window_rules.replace.radio_button_replace_case_sensitive.clone();
    let entry_replace_text_to_remove = window_rules.replace.entry_replace_text_to_remove.clone();
    let entry_replace_text_to_change = window_rules.replace.entry_replace_text_to_change.clone();

    let radio_button_add_number_before_name = window_rules.add_number.radio_button_add_number_before_name.clone();
    let radio_button_add_number_after_name = window_rules.add_number.radio_button_add_number_after_name.clone();
    let entry_add_number_start_number = window_rules.add_number.entry_add_number_start_number.clone();
    let entry_add_number_step = window_rules.add_number.entry_add_number_step.clone();
    let entry_add_number_zeros = window_rules.add_number.entry_add_number_zeros.clone();

    let rule_type: RuleType;
    let rule_place: RulePlace;
    let mut rule_data: RuleData = RuleData::new();
    let rule_description: String;

    let notebook_enum = if let Some(notebook_number) = notebook_number {
        to_notebook_enum(notebook_number)
    } else {
        to_notebook_enum(notebook_choose_rule.current_page().unwrap())
    };

    match notebook_enum {
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
    SingleRule {
        rule_type,
        rule_place,
        rule_data,
        rule_description,
    }
}
