use std::path::PathBuf;

use crate::add_files_folders::add_files_to_check;
use crate::fls;
use gtk4::prelude::*;
use gtk4::ResponseType;

use crate::gui_data_things::gui_data::GuiData;
use crate::help_function::{get_list_store_from_tree_view, get_selected_folders_files_in_dialog};
use crate::update_records::{update_records, UpdateMode};

pub fn connect_add_files_button(gui_data: &GuiData) {
    let button_add_files = gui_data.upper_buttons.button_add_files.clone();
    let tree_view_results = gui_data.results.tree_view_results.clone();
    let shared_result_entries = gui_data.shared_result_entries.clone();
    let rules = gui_data.rules.clone();

    let label_files_folders = gui_data.upper_buttons.label_files_folders.clone();

    let window_main = gui_data.window_main.clone();
    button_add_files.connect_clicked(move |_| {
        let chooser = gtk4::FileChooserDialog::builder()
            .title(fls!("dialog_name_files_to_include"))
            .action(gtk4::FileChooserAction::Open)
            .transient_for(&window_main)
            .modal(true)
            .build();
        chooser.add_button(&fls!("dialog_button_ok"), ResponseType::Ok);
        chooser.add_button(&fls!("dialog_button_cancel"), ResponseType::Cancel);

        chooser.set_select_multiple(true);
        chooser.show();

        let tree_view_results = tree_view_results.clone();
        let label_files_folders = label_files_folders.clone();
        let shared_result_entries = shared_result_entries.clone();
        let rules = rules.clone();

        chooser.connect_response(move |dialog, response_type| {
            if response_type == ResponseType::Ok {
                let files: Vec<PathBuf> = get_selected_folders_files_in_dialog(dialog);

                let list_store = get_list_store_from_tree_view(&tree_view_results);
                {
                    let mut result_entries = shared_result_entries.borrow_mut();
                    add_files_to_check(files, &list_store, &mut result_entries);
                }

                update_records(&tree_view_results, &shared_result_entries, &rules, &UpdateMode::FileAdded, &label_files_folders);
            }

            dialog.close();
        });
    });
}
