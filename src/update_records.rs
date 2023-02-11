use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use glib::Value;
use gtk4::prelude::*;
use gtk4::{Label, TreeView};

use crate::help_function::{get_list_store_from_tree_view, ColumnsResults, ResultEntries};
use crate::rule::rules::Rules;

// Do not update records automatically when there is a big number of entries each time due possible freezes, when rule_number * files_number > RULES_UPDATE_LIMIT, show info about needing to update results
const RULES_UPDATE_LIMIT: usize = 20000;

#[allow(dead_code)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
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
pub fn update_records(files_tree_view: &TreeView, shared_result_entries: &Rc<RefCell<ResultEntries>>, rules: &Rc<RefCell<Rules>>, update_mode: &UpdateMode, label_files_folders: &Label) {
    let list_store = get_list_store_from_tree_view(files_tree_view);
    let mut rules = rules.borrow_mut();
    let rules = &mut *rules;
    let mut shared_result_entries = shared_result_entries.borrow_mut();
    let shared_result_entries = &mut *shared_result_entries;

    rules.edit_mode = None;
    if shared_result_entries.files.len() * rules.rules.len() > RULES_UPDATE_LIMIT && update_mode != &UpdateMode::UpdateRecords {
        label_files_folders.set_text(format!("Files/Folders({}) - ##### UPDATE REQUIRED ##### ", shared_result_entries.files.len()).as_str());
        rules.updated = false;
        return;
    }
    rules.updated = true;
    label_files_folders.set_text(format!("Files/Folders({}) - up to date", shared_result_entries.files.len()).as_str());

    match update_mode {
        UpdateMode::FileAdded | UpdateMode::RuleAdded | UpdateMode::FileRemoved | UpdateMode::RuleRemoved | UpdateMode::RuleMoved | UpdateMode::UpdateRecords | UpdateMode::FileMoved => {
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
                    let changed_value = rules.apply_all_rules_to_item(value_to_change, current_index + 1, *curr_folder_file_index + 1, (modification_date, creation_date, file_size, &path));
                    *curr_folder_file_index += 1;
                    list_store.set_value(&iter, ColumnsResults::FutureName as u32, &Value::from(&changed_value));
                    if !list_store.iter_next(&iter) {
                        break; // This is the end
                    }
                    current_index += 1;
                }
            }
        } // TODO Add Optimized version, that not calculate rules not changed files, rules etc.(e.g. when adding files, old files not needs to be calculated)
    }
}
