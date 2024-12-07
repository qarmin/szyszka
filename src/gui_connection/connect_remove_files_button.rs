use gtk4::prelude::*;

use crate::gui_data_things::gui_data::GuiData;
use crate::help_function::{get_full_file_names_from_selection, remove_selected_rows};
use crate::update_records::{update_records, UpdateMode};

pub fn connect_remove_files_button(gui_data: &GuiData) {
    let button_remove_selection = gui_data.upper_buttons.button_remove_selection.clone();
    let tree_view_results = gui_data.results.tree_view_results.clone();
    let shared_result_entries = gui_data.shared_result_entries.clone();
    let rules = gui_data.rules.clone();

    let label_files_folders = gui_data.upper_buttons.label_files_folders.clone();

    button_remove_selection.connect_clicked(move |_| {
        {
            let mut result_entries = shared_result_entries.borrow_mut();

            for i in get_full_file_names_from_selection(&tree_view_results) {
                let removed = result_entries.files.remove(&i);
                debug_assert!(removed);
            }
        }

        remove_selected_rows(&tree_view_results);
        update_records(&tree_view_results, &shared_result_entries, &rules, UpdateMode::FileRemoved, &label_files_folders);
    });
}
