use gtk4::prelude::*;

use crate::gui_data::GuiData;
use crate::update_records::{update_records, UpdateMode};

pub fn connect_button_update_names(gui_data: &GuiData) {
    let button_update_names = gui_data.upper_buttons.button_update_names.clone();
    let tree_view_results = gui_data.results.tree_view_results.clone();
    let shared_result_entries = gui_data.shared_result_entries.clone();
    let rules = gui_data.rules.clone();

    let label_files_folders = gui_data.upper_buttons.label_files_folders.clone();

    button_update_names.connect_clicked(move |_| {
        update_records(&tree_view_results, &shared_result_entries, &rules, &UpdateMode::UpdateRecords, &label_files_folders);
    });
}
