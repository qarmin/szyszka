use std::cell::RefCell;
use std::cmp::max;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::time::UNIX_EPOCH;

use chrono::{Local, NaiveDateTime};
use gtk4::prelude::*;
use gtk4::*;

use crate::rule::rules::*;

pub struct ResultEntries {
    pub files: BTreeSet<String>,
}

pub enum ColumnsResults {
    TypeString = 0,
    Type,
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

#[derive(PartialEq, Eq)]
pub enum DirFileType {
    File = 0,
    Directory,
}

pub fn to_dir_file_from_u8(dir_file_type: u8) -> DirFileType {
    match dir_file_type {
        0 => DirFileType::File,
        1 => DirFileType::Directory,
        _ => panic!("Unknown DirFileType"),
    }
}

pub fn to_dir_file_type(is_dir: bool) -> DirFileType {
    if is_dir {
        DirFileType::Directory
    } else {
        DirFileType::File
    }
}

pub fn to_dir_file_name(is_dir: bool) -> &'static str {
    if is_dir {
        "Dir"
    } else {
        "File"
    }
}

#[cfg(not(target_family = "windows"))]
pub const CHARACTER: char = '/';
#[cfg(target_family = "windows")]
pub const CHARACTER: char = '\\';

pub fn validate_name(before_name: &str) -> String {
    // TODO when trying to print text in middle of text, then caret change position, fix it
    before_name.chars().filter(|e| *e != '\\' && *e != '/').collect::<String>()
}

pub fn validate_number(before_name: &str) -> String {
    before_name.chars().filter(char::is_ascii_digit).collect::<String>()
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
    tree_view.model().unwrap().downcast::<ListStore>().unwrap()
}

pub fn populate_rules_tree_view(tree_view: &TreeView, rules: &Rc<RefCell<Rules>>) {
    let mut rules = rules.borrow_mut();
    let rules = &mut *rules;

    let list_store = get_list_store_from_tree_view(tree_view);

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

pub fn remove_selected_rows(tree_view: &TreeView) -> Vec<usize> {
    let selection = tree_view.selection();

    let (selected_rows, _tree_model) = selection.selected_rows();

    // Nothing selected
    if selected_rows.is_empty() {
        return Vec::new();
    }

    let list_store = get_list_store_from_tree_view(tree_view);

    let mut vec_index_to_delete: Vec<_> = Vec::new();
    let mut current_iter: usize = 0;

    let tree_iter = list_store.iter_first().unwrap();

    // Get indexes of removed values
    for selected_tree_path in &selected_rows {
        loop {
            if list_store.path(&tree_iter) == *selected_tree_path {
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

pub fn get_full_file_names_from_selection(tree_view: &TreeView) -> Vec<String> {
    let selection = tree_view.selection();

    let (selected_rows, _tree_model) = selection.selected_rows();

    let mut return_vec = Vec::with_capacity(selected_rows.len());

    // Nothing selected
    if selected_rows.is_empty() {
        return return_vec;
    }

    let list_store = get_list_store_from_tree_view(tree_view);

    // Get indexes of removed values
    for selected_tree_path in &selected_rows {
        let tree_iter = list_store.iter(selected_tree_path).unwrap();
        return_vec.push(format!(
            "{}{}{}",
            list_store.get::<String>(&tree_iter, ColumnsResults::Path as i32),
            CHARACTER,
            list_store.get::<String>(&tree_iter, ColumnsResults::CurrentName as i32)
        ));
    }

    return_vec
}

pub fn count_rows_in_tree_view(tree_view: &TreeView) -> u32 {
    let list_store = get_list_store_from_tree_view(tree_view);
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

pub fn get_dialog_box_child(dialog: &Dialog) -> Box {
    dialog.child().unwrap().downcast::<Box>().unwrap()
}

pub fn create_message_window(window_main: &Window, title: &str, message: &str) {
    let dialog = Dialog::builder().title(title).transient_for(window_main).modal(true).build();
    dialog.connect_response(|e, _r| e.close());
    dialog.add_button("Ok", ResponseType::Ok);

    let question_label = Label::new(Some(message));

    let chooser_box = get_dialog_box_child(&dialog);
    chooser_box.insert_child_after(&question_label, None::<&Widget>);
    chooser_box.set_margin_top(5);
    chooser_box.set_margin_bottom(5);
    chooser_box.set_margin_start(5);
    chooser_box.set_margin_end(5);

    dialog.show();
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
    if !expression.starts_with('*') && directory.find(splits[0]).unwrap() > 0 {
        return false;
    }
    // `*home` shouldn't be true for `/homeowner`
    if !expression.ends_with('*') && !directory.ends_with(splits.last().unwrap()) {
        return false;
    }

    // At the end we check if parts between * are correctly positioned
    position_of_splits.push(directory.find(splits[0]).unwrap());
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

pub fn get_all_boxes_from_widget<P: IsA<Widget>>(item: &P) -> Vec<Box> {
    let mut widgets_to_check = vec![item.clone().upcast::<Widget>()];
    let mut boxes = Vec::new();

    while let Some(widget) = widgets_to_check.pop() {
        widgets_to_check.extend(get_all_children(&widget));
        if let Ok(bbox) = widget.clone().downcast::<Box>() {
            boxes.push(bbox);
        }
    }
    boxes
}

pub fn get_all_children<P: IsA<Widget>>(wid: &P) -> Vec<Widget> {
    let mut vector = vec![];
    if let Some(mut child) = wid.first_child() {
        vector.push(child.clone());
        loop {
            child = match child.next_sibling() {
                Some(t) => t,
                None => break,
            };
            vector.push(child.clone());
        }
    }

    vector
}

pub struct ItemStruct {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub modification_date: u64,
    pub creation_date: u64,
    pub date: String,
    pub is_dir: bool,
}

pub fn collect_files(items_to_check: &[PathBuf], result_entries: &mut ResultEntries) -> Vec<ItemStruct> {
    let mut collected_items = Vec::new();
    let timezone_offset = Local::now().offset().local_minus_utc();

    for file_entry in items_to_check {
        let (path, name) = split_path(file_entry);
        let Some(full_name) = file_entry.to_str() else {
            println!("Failed to read name of {file_entry:?} (some characters may be missing in this name)");
            continue;
        };

        if result_entries.files.contains(full_name) {
            // Remove this println
            // println!("Already is used file name {}", full_name);
            continue; // There is already entry
        }

        //// Read Metadata
        let file_metadata = match fs::metadata(file_entry) {
            Ok(t) => t,
            Err(err) => {
                eprintln!("Failed to load metadata of file {}, reason - \"{}\"", file_entry.display(), err);
                continue;
            }
        };
        let size = file_metadata.len();
        let modification_date = match file_metadata.modified() {
            Ok(t) => {
                if let Ok(d) = t.duration_since(UNIX_EPOCH) {
                    max(d.as_secs() as i64 + timezone_offset as i64, 0) as u64
                } else {
                    eprintln!("File {} seems to be modified before Unix Epoch.", file_entry.display());
                    0
                }
            }
            Err(err) => {
                eprintln!("Unable to get modification date from file {}, reason - \"{}\"", file_entry.display(), err);
                0
            }
        };
        let creation_date = match file_metadata.created() {
            Ok(t) => {
                if let Ok(d) = t.duration_since(UNIX_EPOCH) {
                    max(d.as_secs() as i64 + timezone_offset as i64, 0) as u64
                } else {
                    eprintln!("File {} seems to be created before Unix Epoch.", file_entry.display());
                    0
                }
            }
            Err(err) => {
                eprintln!("Unable to get creation date from file {}, reason - \"{}\"", file_entry.display(), err);
                0
            }
        };

        collected_items.push(ItemStruct {
            name: name.to_string(),
            path: path.to_string(),
            size,
            modification_date,
            creation_date,
            date: NaiveDateTime::from_timestamp_opt(creation_date as i64, 0).unwrap().format("%Y-%m-%d %H:%M:%S").to_string(),
            is_dir: file_metadata.is_dir(),
        });

        // Used to check if already in treeview is this values
        result_entries.files.insert(full_name.to_string());
    }

    collected_items
}

pub fn get_selected_folders_files_in_dialog(dialog: &FileChooserDialog) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = Vec::new();
    let g_files = dialog.files();
    for index in 0..g_files.n_items() {
        let file = &g_files.item(index);
        if let Some(file) = file {
            let ss = file.clone().downcast::<gio::File>().unwrap();
            if let Some(path_buf) = ss.path() {
                files.push(path_buf);
            }
        }
    }
    files
}
