use crate::class_gui_data::GuiData;
use crate::help_function::{get_list_store_from_tree_view, remove_selected_rows, split_path};
use crate::update_records::{update_records, UpdateMode};
use gtk::prelude::*;
use std::fs;
use std::ops::DerefMut;
use std::time::UNIX_EPOCH;

pub fn connect_remove_files_button(gui_data: &GuiData) {
    let button_remove_selection = gui_data.upper_buttons.button_remove_selection.clone();
    let tree_view_results = gui_data.results.tree_view_results.clone();
    let shared_result_entries = gui_data.shared_result_entries.clone();
    let rules = gui_data.rules.clone();

    let window_main = gui_data.window_main.clone();

    let list_store = get_list_store_from_tree_view(&tree_view_results);

    button_remove_selection.connect_clicked(move |_| {
        // TODO Remove files from BTreeMap
        remove_selected_rows(&tree_view_results);
        update_records(&tree_view_results, shared_result_entries.clone(), rules.clone(), UpdateMode::FileRemoved);
    });
}
