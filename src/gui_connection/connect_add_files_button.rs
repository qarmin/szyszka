use std::cmp::{max, Ordering};
use std::fs;
use std::path::PathBuf;
use std::time::UNIX_EPOCH;

use chrono::Local;
use gtk4::prelude::*;
use gtk4::ResponseType;

use crate::gui_data::GuiData;
use crate::help_function::{get_list_store_from_tree_view, split_path, ColumnsResults};
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
                let mut folder: Vec<PathBuf> = Vec::new();
                let g_files = dialog.files();
                for index in 0..g_files.n_items() {
                    let file = &g_files.item(index);
                    if let Some(file) = file {
                        let ss = file.clone().downcast::<gio::File>().unwrap();
                        if let Some(path_buf) = ss.path() {
                            folder.push(path_buf);
                        }
                    }
                }

                let mut result_entries = shared_result_entries.borrow_mut();

                let list_store = get_list_store_from_tree_view(&tree_view_results);

                folder.sort_by(|a, b| {
                    let (path_a, name_a) = split_path(a);
                    let (path_b, name_b) = split_path(b);
                    let res = path_a.cmp(&path_b);
                    if res == Ordering::Equal {
                        return name_a.cmp(&name_b);
                    }
                    res
                });

                let timezone_offset = Local::now().offset().local_minus_utc();

                for file_entry in &folder {
                    let (path, name) = split_path(file_entry);
                    let full_name = if let Some(t) = file_entry.to_str() {
                        t
                    } else {
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
                    let is_dir = if file_metadata.is_dir() { "Dir" } else { "File" };

                    //// Create entry and save it to metadata
                    let values: [(u32, &dyn ToValue); 7] = [
                        (ColumnsResults::Type as u32, &is_dir),
                        (ColumnsResults::CurrentName as u32, &name),
                        (ColumnsResults::FutureName as u32, &name),
                        (ColumnsResults::Path as u32, &path),
                        (ColumnsResults::Size as u32, &size),
                        (ColumnsResults::ModificationDate as u32, &modification_date),
                        (ColumnsResults::CreationDate as u32, &creation_date),
                    ];
                    list_store.set(&list_store.append(), &values);

                    // Used to check if already in treeview is this values
                    result_entries.files.insert(full_name.to_string());
                }
            }
            update_records(&tree_view_results, &shared_result_entries, &rules, &UpdateMode::FileAdded, &label_files_folders);

            dialog.close();
        });
    });
}
