use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::fls;
use glib::Value;
use gtk4::prelude::*;
use gtk4::{Label, TreeView};
use regex::Regex;

use crate::help_function::{get_list_store_from_tree_view, ColumnsResults, ResultEntries};
use crate::localizer::generate_translation_hashmap;
use crate::rule::rules::{RuleType, Rules};

// Do not update records automatically when there is a big number of entries each time due possible freezes, when rule_number * files_number > RULES_UPDATE_LIMIT, show info about needing to update results
const RULES_UPDATE_LIMIT: usize = 20000;

#[allow(dead_code)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum UpdateMode {
    FileAdded,
    FileRemoved,
    FileMoved,
    RuleAdded,
    RuleRemoved,
    RuleMoved,
    UpdateRecords, // User clicked update names button
}

// TODO currently everything is counted from beginning
pub fn update_records(
    files_tree_view: &TreeView,
    shared_result_entries: &Rc<RefCell<ResultEntries>>,
    rules: &Rc<RefCell<Rules>>,
    update_mode: &UpdateMode,
    label_files_folders: &Label,
) {
    let list_store = get_list_store_from_tree_view(files_tree_view);
    let mut rules = rules.borrow_mut();
    let rules = &mut *rules;
    let mut shared_result_entries = shared_result_entries.borrow_mut();
    let shared_result_entries = &mut *shared_result_entries;

    rules.edit_mode = None;
    if shared_result_entries.files.len() * rules.rules.len() > RULES_UPDATE_LIMIT && update_mode != &UpdateMode::UpdateRecords {
        label_files_folders.set_text(&fls!(
            "upper_files_folders_label_update",
            generate_translation_hashmap(vec![("files_number", shared_result_entries.files.len().to_string()),])
        ));
        rules.updated = false;
        return;
    }
    rules.updated = true;
    label_files_folders.set_text(&fls!(
        "upper_files_folders_label_up_to_date",
        generate_translation_hashmap(vec![("files_number", shared_result_entries.files.len().to_string()),])
    ));

    let compiled_regexes: Vec<Option<Regex>> = rules
        .rules
        .iter()
        .map(|e| {
            if e.rule_data.use_regex {
                match Regex::new(&e.rule_data.text_to_find) {
                    Ok(regex) => Some(regex),
                    Err(_) => None,
                }
            } else {
                None
            }
        })
        .collect(); // TODO maybe there is a way to compile regexes only once, when adding them?

    match update_mode {
        UpdateMode::FileAdded | UpdateMode::RuleAdded | UpdateMode::RuleRemoved | UpdateMode::RuleMoved | UpdateMode::UpdateRecords => {
            update_records_general(&list_store, rules, &compiled_regexes);
        } // TODO Add Optimized version, that not calculate rules not changed files, rules etc.(e.g. when adding files, old files not needs to be calculated)
        UpdateMode::FileRemoved | UpdateMode::FileMoved => {
            // When using custom rules that are not related to its index in list store, update all records
            if rules
                .rules
                .iter()
                .any(|e| (e.rule_type == RuleType::Custom) && (e.rule_data.custom_text.contains("(K") || e.rule_data.custom_text.contains("(N")))
            {
                update_records_general(&list_store, rules, &compiled_regexes);
            }
        }
    }
}

fn update_records_general(list_store: &gtk4::ListStore, rules: &mut Rules, compiled_regexes: &[Option<Regex>]) {
    if let Some(iter) = list_store.iter_first() {
        let mut current_index = 0;
        let mut folder_name_counter: HashMap<String, u32> = Default::default();
        loop {
            let value_to_change = list_store.get::<String>(&iter, ColumnsResults::CurrentName as i32);
            let modification_date: u64 = list_store.get::<u64>(&iter, ColumnsResults::ModificationDate as i32);
            let creation_date: u64 = list_store.get::<u64>(&iter, ColumnsResults::CreationDate as i32);
            let file_size: u64 = list_store.get::<u64>(&iter, ColumnsResults::Size as i32);
            let path: String = list_store.get::<String>(&iter, ColumnsResults::Path as i32);
            let curr_folder_file_index = folder_name_counter.entry(path.clone()).or_insert(0);
            let changed_value = rules.apply_all_rules_to_item(
                value_to_change,
                current_index + 1,
                *curr_folder_file_index + 1,
                (modification_date, creation_date, file_size, &path),
                compiled_regexes,
            );
            *curr_folder_file_index += 1;
            list_store.set_value(&iter, ColumnsResults::FutureName as u32, &Value::from(&changed_value));
            if !list_store.iter_next(&iter) {
                break; // This is the end
            }
            current_index += 1;
        }
    }
}
