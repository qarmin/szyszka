use gtk4::prelude::*;
use gtk4::ResponseType;

use crate::add_files_folders::add_files_to_check;

use crate::gui_data_things::gui_data::GuiData;
use crate::help_function::{get_list_store_from_tree_view, get_selected_folders_files_in_dialog};
use crate::update_records::{update_records, UpdateMode};

pub fn connect_add_files_button(gui_data: &GuiData) {
    let button_add_files = gui_data.upper_buttons.button_add_files.clone();
    let tree_view_results = gui_data.results.tree_view_results.clone();
    let shared_result_entries = gui_data.shared_result_entries.clone();
    let rules = gui_data.rules.clone();

    let label_files_folders = gui_data.upper_buttons.label_files_folders.clone();

    let file_chooser_dialog_add_files = gui_data.upper_buttons.file_chooser_dialog_add_files.clone();

    file_chooser_dialog_add_files.connect_response(move |file_chooser_dialog_add_files, response| {
        let shared_result_entries = shared_result_entries.clone();
        if response == ResponseType::Accept {
            let files_to_check = get_selected_folders_files_in_dialog(file_chooser_dialog_add_files);

            let list_store = get_list_store_from_tree_view(&tree_view_results);
            {
                let mut result_entries = shared_result_entries.borrow_mut();
                add_files_to_check(files_to_check, &list_store, &mut result_entries);
            }

            update_records(&tree_view_results, &shared_result_entries, &rules, &UpdateMode::FileAdded, &label_files_folders);
        }
    });

    button_add_files.connect_clicked(move |_| {
        file_chooser_dialog_add_files.show();
    });
}
