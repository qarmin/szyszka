use std::cmp::Ordering;

use std::path::PathBuf;

use crate::gui_data_things::gui_data::GuiData;

use glib::signal::Inhibit;
use gtk4::prelude::*;
use gtk4::{Orientation, ResponseType};
use jwalk::WalkDir;

use crate::help_function::{collect_files, get_all_boxes_from_widget, get_list_store_from_tree_view, get_selected_folders_files_in_dialog, split_path, to_dir_file_name, to_dir_file_type, ColumnsResults};
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

                    let folders_to_check: Vec<PathBuf> = get_selected_folders_files_in_dialog(chooser);

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
                                            new_entries.push(entry.path());
                                        }
                                    }
                                }
                            }
                        } else {
                            for folder in folders_to_check {
                                for entry in WalkDir::new(folder).max_depth(9999).into_iter().filter_map(Result::ok) {
                                    new_entries.push(entry.path());
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

                    let results = collect_files(&folders, &mut result_entries);

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

                chooser.close();
            });
        };
    });
}
