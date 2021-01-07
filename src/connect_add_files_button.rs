use crate::class_gui_data::GuiData;
use crate::file_entry::FileEntry;
use crate::help_function::{get_list_store_from_tree_view, split_path};
use crate::update_records::{update_records, UpdateMode};
use gtk::prelude::*;
use std::fs;
use std::time::UNIX_EPOCH;

pub fn connect_add_files_button(gui_data: &GuiData) {
    let button_add_entries = gui_data.upper_buttons.button_add_entries.clone();
    let tree_view_results = gui_data.results.tree_view_results.clone();
    let shared_result_entries = gui_data.shared_result_entries.clone();
    let rules = gui_data.rules.clone();

    let window_main = gui_data.window_main.clone();
    button_add_entries.connect_clicked(move |_| {
        let chooser = gtk::FileChooserDialog::with_buttons(Some("Files to include"), Some(&window_main), gtk::FileChooserAction::Open, &[("Ok", gtk::ResponseType::Ok), ("Close", gtk::ResponseType::Cancel)]);
        chooser.set_select_multiple(true);
        chooser.show_all();
        let response_type = chooser.run();
        if response_type == gtk::ResponseType::Ok {
            let folder = chooser.get_filenames();

            let mut result_entries = shared_result_entries.borrow_mut();

            let list_store = get_list_store_from_tree_view(&tree_view_results);
            // TODO Update names to be
            let col_indices = [0, 1, 2];
            for file_entry in &folder {
                //// Read Metadata
                let file_metadata = match fs::metadata(&file_entry) {
                    Ok(t) => t,
                    Err(_) => {
                        println!("Failed to load metadata of file {}", file_entry.display());
                        continue;
                    }
                };
                let size = file_metadata.len();
                let modification_date = match file_metadata.modified() {
                    Ok(t) => match t.duration_since(UNIX_EPOCH) {
                        Ok(d) => d.as_secs(),
                        Err(_) => {
                            println!("File {} seems to be modified before Unix Epoch.", file_entry.display());
                            0
                        }
                    },
                    Err(_) => {
                        println!("Unable to get modification date from file {}", file_entry.display());
                        continue;
                    }
                };
                let creation_date = match file_metadata.created() {
                    Ok(t) => match t.duration_since(UNIX_EPOCH) {
                        Ok(d) => d.as_secs(),
                        Err(_) => {
                            println!("File {} seems to be created before Unix Epoch.", file_entry.display());
                            0
                        }
                    },
                    Err(_) => {
                        println!("Unable to get creation date from file {}", file_entry.display());
                        continue;
                    }
                };

                //// Create entry and save it to metadata
                let (path, name) = split_path(file_entry);
                let values: [&dyn ToValue; 3] = [&name, &name, &path];
                list_store.set(&list_store.append(), &col_indices, &values);

                result_entries.entries.push(FileEntry {
                    current_name: name.clone(),
                    future_names: vec![name],
                    path,
                    size,
                    modification_date,
                    creation_date,
                    opening_date: 0,
                    image_dimensions: 0,
                });
            }
        }

        update_records(&tree_view_results, shared_result_entries.clone(), rules.clone(), UpdateMode::FileAdded);

        chooser.close();
    });
}
