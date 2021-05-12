use crate::class_gui_data::GuiData;
use crate::help_function::{get_full_file_names_from_selection, remove_selected_rows};
use crate::update_records::{update_records, UpdateMode};
use gtk::prelude::*;

pub fn connect_remove_files_button(gui_data: &GuiData) {
    let button_remove_selection = gui_data.upper_buttons.button_remove_selection.clone();
    let tree_view_results = gui_data.results.tree_view_results.clone();
    let shared_result_entries = gui_data.shared_result_entries.clone();
    let rules = gui_data.rules.clone();

    button_remove_selection.connect_clicked(move |_| {
        let mut result_entries = shared_result_entries.borrow_mut();

        for i in get_full_file_names_from_selection(&tree_view_results) {
            result_entries.files.remove(&i);
        }

        remove_selected_rows(&tree_view_results);
        update_records(&tree_view_results, shared_result_entries.clone(), rules.clone(), UpdateMode::FileRemoved);
    });
}
