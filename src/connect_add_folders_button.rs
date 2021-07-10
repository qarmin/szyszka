use crate::class_gui_data::GuiData;
use crate::help_function::{get_list_store_from_tree_view, split_path};
use crate::update_records::{update_records, UpdateMode};
use gtk::prelude::*;
use gtk::{FileChooserAction, Orientation, PackType};
use std::fs;
use std::time::UNIX_EPOCH;
use walkdir::WalkDir;

pub fn connect_add_folders_button(gui_data: &GuiData) {
    let button_add_folders = gui_data.upper_buttons.button_add_folders.clone();
    let tree_view_results = gui_data.results.tree_view_results.clone();
    let shared_result_entries = gui_data.shared_result_entries.clone();
    let rules = gui_data.rules.clone();

    let window_main = gui_data.window_main.clone();

    button_add_folders.connect_clicked(move |_| {
        let chooser = gtk::FileChooserDialog::with_buttons(Some("Files to include"), Some(&window_main), gtk::FileChooserAction::Open, &[("Ok", gtk::ResponseType::Ok), ("Close", gtk::ResponseType::Cancel)]);
        chooser.set_select_multiple(true);
        chooser.set_action(FileChooserAction::SelectFolder);

        // Adds recursive button to FileDialog
        let box_pack = gtk::Box::new(Orientation::Horizontal, 0);

        let switch = gtk::Switch::new();
        box_pack.add(&switch);
        box_pack.set_child_packing(&switch, false, true, 5, PackType::End);

        let label = gtk::Label::new(Some("Recursive check of folders "));
        box_pack.add(&label);
        box_pack.set_child_packing(&label, false, true, 0, PackType::End);

        let internal_box = chooser.children()[0].clone().downcast::<gtk::Box>().unwrap();
        internal_box.add(&box_pack);

        chooser.set_title("Folders to include");
        chooser.show_all();
        let response_type = chooser.run();
        if response_type == gtk::ResponseType::Ok {
            let mut folders = chooser.filenames();

            if switch.is_active() {
                let mut new_entries = Vec::new();
                for folder in folders {
                    for entry in WalkDir::new(folder).into_iter().filter_map(|e| e.ok()) {
                        new_entries.push(entry.path().to_path_buf());
                    }
                }
                folders = new_entries;
            }

            let mut result_entries = shared_result_entries.borrow_mut();

            let list_store = get_list_store_from_tree_view(&tree_view_results);

            for file_entry in &folders {
                let (path, name) = split_path(file_entry);
                let full_name = match file_entry.to_str() {
                    Some(t) => t,
                    None => {
                        println!("Failed to read name of {:?} (some characters may be missing in this name)", file_entry);
                        continue;
                    }
                };

                if result_entries.files.contains(full_name) {
                    // Remove this println
                    // println!("Already is used file name {}", full_name);
                    continue; // There is already entry
                }

                //// Read Metadata
                let file_metadata = match fs::metadata(&file_entry) {
                    Ok(t) => t,
                    Err(_) => {
                        eprintln!("Failed to load metadata of file {}", file_entry.display());
                        continue;
                    }
                };
                let size = file_metadata.len();
                let modification_date = match file_metadata.modified() {
                    Ok(t) => match t.duration_since(UNIX_EPOCH) {
                        Ok(d) => d.as_secs(),
                        Err(_) => {
                            eprintln!("File {} seems to be modified before Unix Epoch.", file_entry.display());
                            0
                        }
                    },
                    Err(_) => {
                        eprintln!("Unable to get modification date from file {}", file_entry.display());
                        continue;
                    }
                };
                let creation_date = match file_metadata.created() {
                    Ok(t) => match t.duration_since(UNIX_EPOCH) {
                        Ok(d) => d.as_secs(),
                        Err(_) => {
                            eprintln!("File {} seems to be created before Unix Epoch.", file_entry.display());
                            0
                        }
                    },
                    Err(_) => {
                        eprintln!("Unable to get creation date from file {}", file_entry.display());
                        continue;
                    }
                };

                //// Create entry and save it to metadata
                let values: [(u32, &dyn ToValue); 6] = [(0, &name), (1, &name), (2, &path), (3, &size), (4, &modification_date), (5, &creation_date)];
                list_store.set(&list_store.append(), &values);

                // Used to check if already in treeview is this values
                result_entries.files.insert(full_name.to_string());
            }
            update_records(&tree_view_results, shared_result_entries.clone(), rules.clone(), UpdateMode::FileAdded);
        }

        chooser.close();
    });
}
