use std::cmp::{max, Ordering};
use std::fs;
use std::path::PathBuf;
use std::time::UNIX_EPOCH;

use crate::gui_data_things::gui_data::GuiData;
use chrono::Local;
use glib::signal::Inhibit;
use gtk4::prelude::*;
use gtk4::{Orientation, ResponseType};
use walkdir::WalkDir;

use crate::help_function::{get_all_boxes_from_widget, get_list_store_from_tree_view, split_path, ColumnsResults};
use crate::update_records::{update_records, UpdateMode};

pub fn connect_add_folders_button(gui_data: &GuiData) {
    let button_add_folders = gui_data.upper_buttons.button_add_folders.clone();
    let tree_view_results = gui_data.results.tree_view_results.clone();
    let shared_result_entries = gui_data.shared_result_entries.clone();
    let rules = gui_data.rules.clone();

    let label_files_folders = gui_data.upper_buttons.label_files_folders.clone();

    let window_main = gui_data.window_main.clone();

    button_add_folders.connect_clicked(move |_| {
        let chooser = gtk4::FileChooserDialog::builder()
            .title("Files to include")
            .action(gtk4::FileChooserAction::SelectFolder)
            .transient_for(&window_main)
            .modal(true)
            .build();
        chooser.add_button("OK", ResponseType::Ok);
        chooser.add_button("Cancel", ResponseType::Cancel);

        chooser.set_select_multiple(true);
        {
            // Adds recursive button to FileDialog
            let box_pack = gtk4::Box::new(Orientation::Horizontal, 0);

            let switch_scan_inside = gtk4::Switch::new();
            box_pack.append(&switch_scan_inside);
            // box_pack.set_child_packing(&switch_scan_inside, false, true, 5, PackType::End); // TODO GTK 4

            let label_scan_inside = gtk4::Label::new(Some("Scan inside "));
            box_pack.append(&label_scan_inside);
            // box_pack.set_child_packing(&label_scan_inside, false, true, 0, PackType::End);

            let switch_ignore_folders = gtk4::Switch::new();
            switch_ignore_folders.set_active(true);
            box_pack.append(&switch_ignore_folders);
            // box_pack.set_child_packing(&switch_ignore_folders, false, true, 5, PackType::End);

            let label_ignore_folders = gtk4::Label::new(Some("Ignore folders "));
            box_pack.append(&label_ignore_folders);
            // box_pack.set_child_packing(&label_ignore_folders, false, true, 0, PackType::End);

            let internal_box = get_all_boxes_from_widget(&chooser)[0].clone();
            internal_box.append(&box_pack);

            switch_ignore_folders.set_sensitive(false);
            let sif = switch_ignore_folders.clone(); //  TODO GTK 4
            switch_scan_inside.connect_state_set(move |_, b| {
                sif.set_sensitive(b);
                Inhibit(false)
            });

            chooser.set_title(Some("Folders to include"));
            chooser.show();

            let shared_result_entries = shared_result_entries.clone();
            let label_files_folders = label_files_folders.clone();
            let tree_view_results = tree_view_results.clone();
            let rules = rules.clone();

            chooser.connect_response(move |chooser, response| {
                if response == ResponseType::Ok {
                    let mut result_entries = shared_result_entries.borrow_mut();

                    let list_store = get_list_store_from_tree_view(&tree_view_results);

                    let mut folders_to_check: Vec<PathBuf> = Vec::new();
                    let g_files = chooser.files();
                    for index in 0..g_files.n_items() {
                        let file = &g_files.item(index);
                        if let Some(file) = file {
                            let ss = file.clone().downcast::<gio::File>().unwrap();
                            if let Some(path_buf) = ss.path() {
                                folders_to_check.push(path_buf);
                            }
                        }
                    }

                    let mut folders;

                    let ignore_folders = switch_ignore_folders.is_active();
                    let check_folders_inside = switch_scan_inside.is_active();

                    let mut new_entries = Vec::new();

                    if check_folders_inside {
                        if ignore_folders {
                            for folder in folders_to_check {
                                for entry in WalkDir::new(folder).max_depth(9999).into_iter().filter_map(Result::ok) {
                                    if let Ok(metadata) = entry.metadata() {
                                        if metadata.is_file() {
                                            new_entries.push(entry.path().to_path_buf());
                                        }
                                    }
                                }
                            }
                        } else {
                            for folder in folders_to_check {
                                for entry in WalkDir::new(folder).max_depth(9999).into_iter().filter_map(Result::ok) {
                                    new_entries.push(entry.path().to_path_buf());
                                }
                            }
                        }
                        folders = new_entries;
                    } else {
                        folders = folders_to_check;
                    }

                    folders.sort_by(|a, b| {
                        let (path_a, name_a) = split_path(a);
                        let (path_b, name_b) = split_path(b);
                        let res = path_a.cmp(&path_b);
                        if res == Ordering::Equal {
                            return name_a.cmp(&name_b);
                        }
                        res
                    });

                    let timezone_offset = Local::now().offset().local_minus_utc();

                    for file_entry in &folders {
                        let (path, name) = split_path(file_entry);
                        let Some(full_name) = file_entry.to_str() else {
                            println!("Failed to read name of {file_entry:?} (some characters may be missing in this name)");
                            continue;
                        };

                        if result_entries.files.contains(full_name) {
                            // Remove this println
                            // println!("Already is used file name {}", full_name);
                            continue; //  There is already entry
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

                chooser.close();
            });
        };
    });
}
