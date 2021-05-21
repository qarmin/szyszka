use crate::file_entry::ResultEntries;
use crate::help_function::{get_list_store_from_tree_view, ColumnsResults};
use crate::rules::Rules;
use glib::Value;
use gtk::prelude::GtkListStoreExtManual;
use gtk::{TreeModelExt, TreeView};
use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;

#[allow(dead_code)]
pub enum UpdateMode {
    FileAdded,
    FileRemoved,
    // FileMoved - Not sure if it is possible
    RuleAdded,
    RuleRemoved,
    RuleMoved,
}

// TODO currently everything is counted from begginng
pub fn update_records(files_tree_view: &TreeView, _shared_result_entries: Rc<RefCell<ResultEntries>>, rules: Rc<RefCell<Rules>>, update_mode: UpdateMode) {
    let list_store = get_list_store_from_tree_view(&files_tree_view);
    // let mut shared_result_entries = shared_result_entries.borrow_mut();
    let mut rules = rules.borrow_mut();
    // let shared_result_entries = shared_result_entries.deref_mut();
    let rules = rules.deref_mut();

    match update_mode {
        UpdateMode::FileAdded | UpdateMode::RuleAdded | UpdateMode::FileRemoved | UpdateMode::RuleRemoved | UpdateMode::RuleMoved => {
            if let Some(iter) = list_store.get_iter_first() {
                let mut current_index = 0;

                // We count how much
                // let mut current_index = 1;
                // let mut end_of_records = false;

                // TODO Properly count number of added elements
                // loop {
                //     if current_index == shared_result_entries.entries.len() {
                //         break;
                //     }
                //     if !list_store.iter_next(&iter) {
                //         panic!("This should never happens, looks that elements was not added but even removed");
                //         //break;
                //     }
                //     current_index += 1;
                // }
                // TODO get info about current row and change it
                loop {
                    let value_to_change = list_store.get_value(&iter, ColumnsResults::CurrentName as i32).get::<String>().unwrap().unwrap();
                    let modification_date: u64 = list_store.get_value(&iter, ColumnsResults::ModificationDate as i32).get::<u64>().unwrap().unwrap();
                    let creation_date: u64 = list_store.get_value(&iter, ColumnsResults::CreationDate as i32).get::<u64>().unwrap().unwrap();
                    let file_size: u64 = list_store.get_value(&iter, ColumnsResults::Size as i32).get::<u64>().unwrap().unwrap();
                    let changed_value = rules.apply_all_rules_to_item(value_to_change, current_index, (modification_date, creation_date, file_size));
                    list_store.set_value(&iter, ColumnsResults::FutureName as u32, &Value::from(&changed_value));
                    if !list_store.iter_next(&iter) {
                        break; // This is the end
                    }
                    current_index += 1;
                }
            }
        } // UpdateMode::Re => {}
          // _ => {
          //     panic!("Not implemented yet")
          // }
    }
}
