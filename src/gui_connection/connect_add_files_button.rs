use std::cmp::Ordering;

use std::path::PathBuf;

use gtk4::prelude::*;
use gtk4::ResponseType;

use crate::gui_data_things::gui_data::GuiData;
use crate::help_function::{collect_files, get_list_store_from_tree_view, get_selected_folders_files_in_dialog, split_path, to_dir_file_name, to_dir_file_type, ColumnsResults};
use crate::update_records::{update_records, UpdateMode};

pub fn connect_add_files_button(gui_data: &GuiData) {
    let button_add_files = gui_data.upper_buttons.button_add_files.clone();
    let tree_view_results = gui_data.results.tree_view_results.clone();
    let shared_result_entries = gui_data.shared_result_entries.clone();
    let rules = gui_data.rules.clone();

    let label_files_folders = gui_data.upper_buttons.label_files_folders.clone();

    let window_main = gui_data.window_main.clone();
    button_add_files.connect_clicked(move |_| {
        let chooser = gtk4::FileChooserDialog::builder().title("Files to include").action(gtk4::FileChooserAction::Open).transient_for(&window_main).modal(true).build();
        chooser.add_button("OK", ResponseType::Ok);
        chooser.add_button("Cancel", ResponseType::Cancel);

        chooser.set_select_multiple(true);
        chooser.show();

        let tree_view_results = tree_view_results.clone();
        let label_files_folders = label_files_folders.clone();
        let shared_result_entries = shared_result_entries.clone();
        let rules = rules.clone();

        chooser.connect_response(move |dialog, response_type| {
            if response_type == ResponseType::Ok {
                let mut files: Vec<PathBuf> = get_selected_folders_files_in_dialog(dialog);

                let mut result_entries = shared_result_entries.borrow_mut();

                let list_store = get_list_store_from_tree_view(&tree_view_results);

                files.sort_by(|a, b| {
                    let (path_a, name_a) = split_path(a);
                    let (path_b, name_b) = split_path(b);
                    let res = path_a.cmp(&path_b);
                    if res == Ordering::Equal {
                        return name_a.cmp(&name_b);
                    }
                    res
                });

                let results = collect_files(&files, &mut result_entries);

                for result in results {
                    let values: [(u32, &dyn ToValue); 8] = [
                        (ColumnsResults::Type as u32, &(to_dir_file_type(result.is_dir) as u8)),
                        (ColumnsResults::TypeString as u32, &to_dir_file_name(result.is_dir)),
                        (ColumnsResults::CurrentName as u32, &result.name),
                        (ColumnsResults::FutureName as u32, &result.name),
                        (ColumnsResults::Path as u32, &result.path),
                        (ColumnsResults::Size as u32, &result.size),
                        (ColumnsResults::ModificationDate as u32, &result.modification_date),
                        (ColumnsResults::CreationDate as u32, &result.creation_date),
                    ];
                    list_store.set(&list_store.append(), &values);
                }
            }
            update_records(&tree_view_results, &shared_result_entries, &rules, &UpdateMode::FileAdded, &label_files_folders);

            dialog.close();
        });
    });
}
