use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use glib::signal::Inhibit;
use gtk4::prelude::*;
use gtk4::{Orientation, ResponseType, Widget, Window};

use crate::add_files_folders::add_folders_to_check;
use crate::fls;
use crate::gui_data_things::gui_data::GuiData;
use crate::help_function::{get_list_store_from_tree_view, get_selected_folders_files_in_dialog, ResultEntries};
use crate::rule::rules::Rules;
use crate::update_records::{update_records, UpdateMode};

pub fn connect_add_folders_button(gui_data: &GuiData) {
    let button_add_folders = gui_data.upper_buttons.button_add_folders.clone();
    let tree_view_results = gui_data.results.tree_view_results.clone();
    let shared_result_entries = gui_data.shared_result_entries.clone();
    let rules = gui_data.rules.clone();

    let label_files_folders = gui_data.upper_buttons.label_files_folders.clone();

    let window_main = gui_data.window_main.clone();

    let file_chooser_dialog_add_folders = gui_data.upper_buttons.file_chooser_dialog_add_folders.clone();

    let shared_result_entries = shared_result_entries;
    let label_files_folders = label_files_folders;
    let tree_view_results = tree_view_results;
    let rules = rules;

    file_chooser_dialog_add_folders.connect_response(move |file_chooser_dialog_add_folders, response| {
        let shared_result_entries = shared_result_entries.clone();
        if response == ResponseType::Accept {
            let folders_to_check = get_selected_folders_files_in_dialog(file_chooser_dialog_add_folders);
            if !folders_to_check.is_empty() {
                create_scan_inside_ignore_files_dialog(
                    folders_to_check,
                    tree_view_results.clone(),
                    shared_result_entries,
                    rules.clone(),
                    label_files_folders.clone(),
                    &window_main,
                );
            }
        }
    });

    button_add_folders.connect_clicked(move |_| {
        file_chooser_dialog_add_folders.show();
    });
}

fn create_scan_inside_ignore_files_dialog(
    folders_to_check: Vec<PathBuf>,
    tree_view_results: gtk4::TreeView,
    shared_result_entries: Rc<RefCell<ResultEntries>>,
    rules: Rc<RefCell<Rules>>,
    label_files_folders: gtk4::Label,
    window_main: &Window,
) {
    let dialog = gtk4::Dialog::builder().transient_for(window_main).modal(true).build();
    dialog.add_button(&fls!("dialog_button_ok"), ResponseType::Ok);
    dialog.add_button(&fls!("dialog_button_cancel"), ResponseType::Cancel);

    dialog.show();

    let dialog_box = dialog.child().unwrap().downcast::<gtk4::Box>().unwrap();
    dialog_box.set_margin_top(10);
    dialog_box.set_margin_bottom(10);
    dialog_box.set_margin_start(10);
    dialog_box.set_margin_end(10);

    let box_general = gtk4::Box::builder()
        .orientation(Orientation::Vertical)
        .margin_end(10)
        .margin_top(10)
        .margin_start(10)
        .margin_bottom(10)
        .build();

    let box_first = gtk4::Box::new(Orientation::Horizontal, 10);
    let switch_scan_inside = gtk4::Switch::new();
    let label_scan_inside = gtk4::Label::builder().label(fls!("dialog_scan_inside")).margin_end(5).build();
    box_first.append(&switch_scan_inside);
    box_first.append(&label_scan_inside);

    let box_second = gtk4::Box::new(Orientation::Horizontal, 10);
    let switch_ignore_folders = gtk4::Switch::new();
    switch_ignore_folders.set_active(true);
    let label_ignore_folders = gtk4::Label::builder().label(fls!("dialog_ignore_folders")).margin_end(5).build();
    box_second.append(&switch_ignore_folders);
    box_second.append(&label_ignore_folders);

    box_general.append(&box_first);
    box_general.append(&box_second);

    box_general.insert_after(&dialog_box, None::<&Widget>);

    switch_ignore_folders.set_sensitive(false);
    let sif = switch_ignore_folders.clone();
    switch_scan_inside.connect_state_set(move |_, b| {
        sif.set_sensitive(b);
        Inhibit(false)
    });

    dialog.connect_response(move |dialog, response| {
        if response == ResponseType::Ok {
            let folders_to_check = folders_to_check.clone();
            let shared_result_entries = shared_result_entries.clone();
            let list_store = get_list_store_from_tree_view(&tree_view_results);
            let ignore_folders = switch_ignore_folders.is_active();
            let check_folders_inside = switch_scan_inside.is_active();
            {
                let mut result_entries = shared_result_entries.borrow_mut();
                add_folders_to_check(folders_to_check, &list_store, &mut result_entries, check_folders_inside, ignore_folders);
            }
            update_records(&tree_view_results, &shared_result_entries, &rules, &UpdateMode::FileAdded, &label_files_folders);
        }
        dialog.close();
    });
}
