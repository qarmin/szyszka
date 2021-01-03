use gtk::prelude::*;
use gtk::*;
use std::path::Path;

pub enum ColumnsResults {
    CurrentName = 0,
    FutureName,
    Path,
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
