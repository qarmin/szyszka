use gtk4::prelude::*;
use gtk4::TreeView;
use std::collections::HashSet;

use crate::gui_data_things::gui_data::GuiData;
use crate::help_function::{cache_list_store_items, get_list_store_from_tree_view, swap_cached_list_store_items};
use crate::update_records::{update_records, UpdateMode};

pub fn connect_results_modify_one_up(gui_data: &GuiData) {
    let button_results_one_up = gui_data.upper_buttons.button_results_one_up.clone();
    let tree_view_results = gui_data.results.tree_view_results.clone();
    let shared_result_entries = gui_data.shared_result_entries.clone();
    let rules = gui_data.rules.clone();

    let label_files_folders = gui_data.upper_buttons.label_files_folders.clone();

    button_results_one_up.connect_clicked(move |_e| {
        move_items_multiple(&tree_view_results, true);
        update_records(&tree_view_results, &shared_result_entries, &rules, &UpdateMode::FileMoved, &label_files_folders);
    });
}
fn move_items_multiple(tree_view: &TreeView, going_up: bool) {
    let selection = tree_view.selection();
    if selection.selected_rows().0.is_empty() {
        return;
    }

    let list_store = get_list_store_from_tree_view(tree_view);
    let list_store_length = list_store.iter_n_children(None);
    let (selected_rows, _tree_model) = selection.selected_rows();
    let mut cached_items = cache_list_store_items(&list_store);

    let mut selected_results = selected_rows.iter().map(|path| path.indices()[0]).collect::<Vec<_>>();
    selected_results.sort_unstable();

    // Items can be swapped, only if element is not selected
    let mut disallowed_indexes = selected_results.iter().copied().collect::<HashSet<_>>();
    let going_up_int = if going_up { -1 } else { 1 };
    if !going_up {
        selected_results.reverse();
    }

    for idx in selected_results {
        if (going_up && idx == 0) || (!going_up && idx == list_store_length - 1) || disallowed_indexes.contains(&(idx + going_up_int)) {
            continue;
        }
        swap_cached_list_store_items(&list_store, &mut cached_items, idx as usize, (idx + going_up_int) as usize);
        disallowed_indexes.remove(&idx);
    }
}

pub fn connect_results_modify_one_down(gui_data: &GuiData) {
    let button_results_one_down = gui_data.upper_buttons.button_results_one_down.clone();
    let tree_view_results = gui_data.results.tree_view_results.clone();
    let shared_result_entries = gui_data.shared_result_entries.clone();
    let rules = gui_data.rules.clone();

    let label_files_folders = gui_data.upper_buttons.label_files_folders.clone();

    button_results_one_down.connect_clicked(move |_e| {
        move_items_multiple(&tree_view_results, false);
        update_records(&tree_view_results, &shared_result_entries, &rules, &UpdateMode::FileMoved, &label_files_folders);
    });
}
