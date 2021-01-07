use crate::rules::*;
use gtk::prelude::*;
use gtk::*;
use std::path::Path;

pub const EXAMPLE_NAME: &str = "Ziemniak.jpG";

pub enum ColumnsResults {
    CurrentName = 0,
    FutureName,
    Path,
    // Size,
    // ModificationDate,
    // Dimensions,
}
pub enum ColumnsRules {
    RuleNumber = 0,
    RuleType,
    UsageType,
    // Size,
    // ModificationDate,
    // Dimensions,
}

pub fn split_path(path: &Path) -> (String, String) {
    match (path.parent(), path.file_name()) {
        (Some(dir), Some(file)) => (dir.display().to_string(), file.to_string_lossy().into_owned()),
        (Some(dir), None) => (dir.display().to_string(), String::new()),
        (None, _) => (String::new(), String::new()),
    }
}

pub fn get_list_store_from_tree_view(tree_view: &TreeView) -> ListStore {
    tree_view.get_model().unwrap().downcast::<gtk::ListStore>().unwrap()
}

pub fn populate_rules_tree_view(tree_view: &gtk::TreeView, rules: &Rules) {
    let list_store = get_list_store_from_tree_view(&tree_view);

    list_store.clear();

    let col_indices = [0, 1, 2];
    for number in 0..rules.rules_number {
        let values: [&dyn ToValue; 3] = [&(number as u32), &rule_type_to_string(&rules.rule_types[number]), &rule_place_to_string(&rules.rule_place[number])];
        list_store.set(&list_store.append(), &col_indices, &values);
    }
}
