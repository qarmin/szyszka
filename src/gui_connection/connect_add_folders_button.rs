use std::path::PathBuf;

use glib::signal::Inhibit;
use gtk4::prelude::*;
use gtk4::{Orientation, ResponseType};

use crate::add_files_folders::add_folders_to_check;
use crate::fls;
use crate::gui_data_things::gui_data::GuiData;
use crate::help_function::{get_all_boxes_from_widget, get_list_store_from_tree_view, get_selected_folders_files_in_dialog};
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
            .title(fls!("dialog_name_files_to_include"))
            .action(gtk4::FileChooserAction::SelectFolder)
            .transient_for(&window_main)
            .modal(true)
            .build();
        chooser.add_button(&fls!("dialog_button_ok"), ResponseType::Ok);
        chooser.add_button(&fls!("dialog_button_cancel"), ResponseType::Cancel);

        chooser.set_select_multiple(true);
        {
            // Adds recursive button to FileDialog
            let box_pack = gtk4::Box::new(Orientation::Horizontal, 0);

            let switch_scan_inside = gtk4::Switch::new();
            box_pack.append(&switch_scan_inside);

            let label_scan_inside = gtk4::Label::builder().label(fls!("dialog_scan_inside")).margin_end(5).build();
            box_pack.append(&label_scan_inside);

            let switch_ignore_folders = gtk4::Switch::new();
            switch_ignore_folders.set_active(true);
            box_pack.append(&switch_ignore_folders);

            let label_ignore_folders = gtk4::Label::builder().label(fls!("dialog_ignore_folders")).margin_end(5).build();
            box_pack.append(&label_ignore_folders);

            let internal_box = get_all_boxes_from_widget(&chooser)[0].clone();
            internal_box.append(&box_pack);

            switch_ignore_folders.set_sensitive(false);
            let sif = switch_ignore_folders.clone(); //  TODO GTK 4
            switch_scan_inside.connect_state_set(move |_, b| {
                sif.set_sensitive(b);
                Inhibit(false)
            });

            chooser.set_title(Some(&fls!("dialog_name_folders_to_include")));
            chooser.show();

            let shared_result_entries = shared_result_entries.clone();
            let label_files_folders = label_files_folders.clone();
            let tree_view_results = tree_view_results.clone();
            let rules = rules.clone();

            chooser.connect_response(move |chooser, response| {
                if response == ResponseType::Ok {
                    let list_store = get_list_store_from_tree_view(&tree_view_results);

                    let folders_to_check: Vec<PathBuf> = get_selected_folders_files_in_dialog(chooser);

                    let ignore_folders = switch_ignore_folders.is_active();
                    let check_folders_inside = switch_scan_inside.is_active();
                    {
                        let mut result_entries = shared_result_entries.borrow_mut();
                        add_folders_to_check(folders_to_check, &list_store, &mut result_entries, check_folders_inside, ignore_folders);
                    }
                    update_records(&tree_view_results, &shared_result_entries, &rules, &UpdateMode::FileAdded, &label_files_folders);
                }

                chooser.close();
            });
        };
    });
}
