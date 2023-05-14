use gtk4::prelude::*;

use crate::gui_data_things::gui_data::GuiData;
use crate::help_function::{get_list_store_from_tree_view, ColumnsResults};
use crate::update_records::{update_records, UpdateMode};

pub fn connect_results_modify_one_up(gui_data: &GuiData) {
    let button_results_one_up = gui_data.upper_buttons.button_results_one_up.clone();
    let tree_view_results = gui_data.results.tree_view_results.clone();
    let shared_result_entries = gui_data.shared_result_entries.clone();
    let rules = gui_data.rules.clone();

    let label_files_folders = gui_data.upper_buttons.label_files_folders.clone();

    button_results_one_up.connect_clicked(move |_e| {
        let selection = tree_view_results.selection();
        if selection.selected_rows().0.is_empty() {
            return;
        }

        {
            let list_store = get_list_store_from_tree_view(&tree_view_results);

            let (selected_rows, _tree_model) = selection.selected_rows();

            let first_iter = list_store.iter_first().unwrap();

            let mut selected_results = Vec::new();
            {
                let temp_iter = first_iter;
                let mut current_path_number = 0;

                loop {
                    let path = list_store.path(&temp_iter);
                    if selected_rows[current_path_number] == path {
                        selected_results.push(true);
                        current_path_number += 1;
                        // Checking if there are any other selections
                        if current_path_number >= selected_rows.len() {
                            break;
                        }
                    } else {
                        selected_results.push(false);
                    }

                    if !list_store.iter_next(&temp_iter) {
                        break;
                    }
                }
            }

            // Swap rules
            {
                let current_iter = first_iter;
                let mut previous_iter = first_iter;
                if !list_store.iter_next(&current_iter) {
                    return; // Only 1 record
                };

                selection.unselect_all();
                if selected_results[0] {
                    selection.select_iter(&first_iter);
                }

                for i in 1..selected_results.len() {
                    if selected_results[i] && !selected_results[i - 1] {
                        selected_results.swap(i, i - 1);

                        list_store_items(&list_store, &previous_iter, &current_iter);
                    }
                    if selected_results[i - 1] {
                        selection.select_iter(&previous_iter);
                    }
                    if selected_results[i] {
                        selection.select_iter(&current_iter);
                    }
                    previous_iter = current_iter;
                    if !list_store.iter_next(&current_iter) {
                        break;
                    }
                }
            }
        }
        update_records(&tree_view_results, &shared_result_entries, &rules, &UpdateMode::FileMoved, &label_files_folders);
    });
}

pub fn connect_results_modify_one_down(gui_data: &GuiData) {
    let button_results_one_down = gui_data.upper_buttons.button_results_one_down.clone();
    let tree_view_results = gui_data.results.tree_view_results.clone();
    let shared_result_entries = gui_data.shared_result_entries.clone();
    let rules = gui_data.rules.clone();

    let label_files_folders = gui_data.upper_buttons.label_files_folders.clone();

    button_results_one_down.connect_clicked(move |_e| {
        let selection = tree_view_results.selection();
        if selection.selected_rows().0.is_empty() {
            return;
        }

        {
            let list_store = get_list_store_from_tree_view(&tree_view_results);

            let (selected_rows, _tree_model) = selection.selected_rows();

            let first_iter = list_store.iter_first().unwrap();
            let mut end_iter;

            let mut selected_results = Vec::new();
            {
                let temp_iter = first_iter;
                let mut current_path_number = 0;

                loop {
                    let path = list_store.path(&temp_iter);
                    if selected_rows[current_path_number] == path {
                        selected_results.push(true);
                        current_path_number += 1;
                        // Checking if there are any other selections -different implementation than
                        if current_path_number >= selected_rows.len() {
                            loop {
                                end_iter = temp_iter;
                                if !list_store.iter_next(&temp_iter) {
                                    break;
                                }
                                selected_results.push(false);
                            }
                            break;
                        }
                    } else {
                        selected_results.push(false);
                    }

                    end_iter = temp_iter;
                    if !list_store.iter_next(&temp_iter) {
                        break;
                    }
                }
            }

            // Swap rules
            {
                let current_iter = end_iter;
                let mut previous_iter = end_iter;
                if !list_store.iter_previous(&current_iter) {
                    return; // Only 1 record
                };

                selection.unselect_all();
                if selected_results[selected_results.len() - 1] {
                    selection.select_iter(&end_iter);
                }

                for i in (0..selected_results.len() - 1).rev() {
                    if selected_results[i] && !selected_results[i + 1] {
                        selected_results.swap(i, i + 1);

                        list_store_items(&list_store, &previous_iter, &current_iter);
                    }
                    if selected_results[i + 1] {
                        selection.select_iter(&previous_iter);
                    }
                    if selected_results[i] {
                        selection.select_iter(&current_iter);
                    }
                    previous_iter = current_iter;
                    if !list_store.iter_previous(&current_iter) {
                        break;
                    }
                }
            }
        }
        update_records(&tree_view_results, &shared_result_entries, &rules, &UpdateMode::FileMoved, &label_files_folders);
    });
}

fn list_store_items(list_store: &gtk4::ListStore, previous_iter: &gtk4::TreeIter, current_iter: &gtk4::TreeIter) {
    let previous_creation = list_store.get::<u64>(previous_iter, ColumnsResults::CreationDate as i32);
    let previous_modification = list_store.get::<u64>(previous_iter, ColumnsResults::ModificationDate as i32);
    let previous_size = list_store.get::<u64>(previous_iter, ColumnsResults::Size as i32);
    let previous_path = list_store.get::<String>(previous_iter, ColumnsResults::Path as i32);
    let previous_future_name = list_store.get::<String>(previous_iter, ColumnsResults::FutureName as i32);
    let previous_type = list_store.get::<String>(previous_iter, ColumnsResults::Type as i32);
    let previous_current_name = list_store.get::<String>(previous_iter, ColumnsResults::CurrentName as i32);

    let current_creation = list_store.get::<u64>(current_iter, ColumnsResults::CreationDate as i32);
    let current_modification = list_store.get::<u64>(current_iter, ColumnsResults::ModificationDate as i32);
    let current_size = list_store.get::<u64>(current_iter, ColumnsResults::Size as i32);
    let current_path = list_store.get::<String>(current_iter, ColumnsResults::Path as i32);
    let current_future_name = list_store.get::<String>(current_iter, ColumnsResults::FutureName as i32);
    let current_type = list_store.get::<String>(current_iter, ColumnsResults::Type as i32);
    let current_current_name = list_store.get::<String>(current_iter, ColumnsResults::CurrentName as i32);

    list_store.set_value(previous_iter, ColumnsResults::CreationDate as u32, &current_creation.to_value());
    list_store.set_value(previous_iter, ColumnsResults::ModificationDate as u32, &current_modification.to_value());
    list_store.set_value(previous_iter, ColumnsResults::Size as u32, &current_size.to_value());
    list_store.set_value(previous_iter, ColumnsResults::Path as u32, &current_path.to_value());
    list_store.set_value(previous_iter, ColumnsResults::FutureName as u32, &current_future_name.to_value());
    list_store.set_value(previous_iter, ColumnsResults::Type as u32, &current_type.to_value());
    list_store.set_value(previous_iter, ColumnsResults::CurrentName as u32, &current_current_name.to_value());

    list_store.set_value(current_iter, ColumnsResults::CreationDate as u32, &previous_creation.to_value());
    list_store.set_value(current_iter, ColumnsResults::ModificationDate as u32, &previous_modification.to_value());
    list_store.set_value(current_iter, ColumnsResults::Size as u32, &previous_size.to_value());
    list_store.set_value(current_iter, ColumnsResults::Path as u32, &previous_path.to_value());
    list_store.set_value(current_iter, ColumnsResults::FutureName as u32, &previous_future_name.to_value());
    list_store.set_value(current_iter, ColumnsResults::Type as u32, &previous_type.to_value());
    list_store.set_value(current_iter, ColumnsResults::CurrentName as u32, &previous_current_name.to_value());
}
