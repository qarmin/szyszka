use crate::rules::*;
use gtk::prelude::*;
use gtk::*;
use std::cell::RefCell;
use std::ops::DerefMut;
use std::path::Path;
use std::rc::Rc;

#[allow(dead_code)]
pub enum ColumnsResults {
    CurrentName = 0,
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
    tree_view.get_model().unwrap().downcast::<gtk::ListStore>().unwrap()
}

pub fn populate_rules_tree_view(tree_view: &gtk::TreeView, rules: Rc<RefCell<Rules>>) {
    let mut rules = rules.borrow_mut();
    let rules = rules.deref_mut();

    let list_store = get_list_store_from_tree_view(&tree_view);

    list_store.clear();

    let col_indices = [0, 1, 2];
    for rule in &rules.rules {
        let values: [&dyn ToValue; 3] = [&rule_type_to_string(&rule.rule_type), &rule_place_to_string(&rule.rule_place), &rule.rule_description];
        list_store.set(&list_store.append(), &col_indices, &values);
    }
}

pub fn remove_selected_rows(tree_view: &gtk::TreeView) -> Vec<usize> {
    let selection = tree_view.get_selection();

    let (selected_rows, _tree_model) = selection.get_selected_rows();

    // Nothing selected
    if selected_rows.is_empty() {
        return Vec::new();
    }

    let list_store = get_list_store_from_tree_view(&tree_view);

    let mut vec_index_to_delete: Vec<_> = Vec::new();
    let mut current_iter: usize = 0;

    let tree_iter = list_store.get_iter_first().unwrap();

    // Get indexes of removed values
    for selected_tree_path in &selected_rows {
        loop {
            if list_store.get_path(&tree_iter).unwrap() == *selected_tree_path {
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
        list_store.remove(&list_store.get_iter(selected_tree_path).unwrap());
    }
    vec_index_to_delete
}
pub fn get_full_file_names_from_selection(tree_view: &gtk::TreeView) -> Vec<String> {
    let selection = tree_view.get_selection();

    let (selected_rows, _tree_model) = selection.get_selected_rows();

    let mut return_vec = Vec::with_capacity(selected_rows.len());

    // Nothing selected
    if selected_rows.is_empty() {
        return return_vec;
    }

    let list_store = get_list_store_from_tree_view(&tree_view);

    // Get indexes of removed values
    for selected_tree_path in &selected_rows {
        let tree_iter = list_store.get_iter(selected_tree_path).unwrap();
        return_vec.push(format!(
            "{}{}{}",
            list_store.get_value(&tree_iter, ColumnsResults::Path as i32).get::<String>().unwrap().unwrap(),
            CHARACTER,
            list_store.get_value(&tree_iter, ColumnsResults::CurrentName as i32).get::<String>().unwrap().unwrap()
        ));
    }

    return_vec
}

pub fn count_rows_in_tree_view(tree_view: &gtk::TreeView) -> u32 {
    let list_store = get_list_store_from_tree_view(&tree_view);
    let mut number = 0;

    if let Some(curr_iter) = list_store.get_iter_first() {
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

    let chooser_box = chooser.get_children()[0].clone().downcast::<gtk::Box>().unwrap();
    chooser_box.add(&question_label);
    chooser_box.show_all();

    chooser.run();
    chooser.close();
}
