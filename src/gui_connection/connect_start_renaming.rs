use std::cell::RefCell;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::rc::Rc;

use crate::fls;
use gtk4::prelude::*;
use gtk4::Label;
use gtk4::{Dialog, DialogFlags, ListStore, ScrolledWindow, TextView, Widget, Window};

use crate::gui_data_things::gui_data::GuiData;
use crate::help_function::{
    count_rows_in_tree_view, create_message_window, get_dialog_box_child, get_list_store_from_tree_view, to_dir_file_from_u8, ColumnsResults, DirFileType, ResultEntries, CHARACTER,
};
use crate::localizer::generate_translation_hashmap;

pub fn connect_start_renaming(gui_data: &GuiData) {
    let button_start_rename = gui_data.upper_buttons.button_start_rename.clone();
    let window_main = gui_data.window_main.clone();

    let tree_view_results = gui_data.results.tree_view_results.clone();
    let rules = gui_data.rules.clone();

    let shared_result_entries = gui_data.shared_result_entries.clone();

    let list_store = get_list_store_from_tree_view(&tree_view_results);

    button_start_rename.connect_clicked(move |_e| {
        let number_of_renamed_files = count_rows_in_tree_view(&tree_view_results);
        if number_of_renamed_files == 0 {
            create_message_window(&window_main, &fls!("renaming_missing_files"), &fls!("renaming_require_missing_files"));
            return;
        }
        let rules = rules.borrow();
        if rules.rules.is_empty() {
            create_message_window(&window_main, &fls!("renaming_missing_rules"), &fls!("renaming_require_missing_rules"));
            return;
        }

        if !rules.updated {
            let chooser_update = Dialog::with_buttons(
                Some(fls!("dialog_outdated_results")),
                Some(&window_main),
                DialogFlags::DESTROY_WITH_PARENT,
                &[
                    (&fls!("dialog_button_ok"), gtk4::ResponseType::Ok),
                    (&fls!("dialog_button_cancel"), gtk4::ResponseType::Cancel),
                ],
            );

            let question_label = Label::new(Some(&fls!("renaming_some_records_not_updated")));

            let chooser_box = get_dialog_box_child(&chooser_update);
            chooser_box.insert_child_after(&question_label, None::<&Widget>);
            chooser_box.set_margin_top(5);
            chooser_box.set_margin_bottom(5);
            chooser_box.set_margin_start(5);
            chooser_box.set_margin_end(5);

            chooser_update.connect_response(move |dialog, _| {
                dialog.close();
                dialog.hide();
            });
        }

        let chooser = Dialog::with_buttons(
            Some(fls!("dialog_confirm_renaming")),
            Some(&window_main),
            DialogFlags::DESTROY_WITH_PARENT,
            &[
                (&fls!("dialog_button_ok"), gtk4::ResponseType::Ok),
                (&fls!("dialog_button_cancel"), gtk4::ResponseType::Cancel),
            ],
        );

        let label_name = fls!(
            "renaming_question",
            generate_translation_hashmap(vec![("number_of_renamed_files", number_of_renamed_files.to_string())])
        );
        let question_label = Label::new(Some(label_name.as_str()));

        let chooser_box = get_dialog_box_child(&chooser);
        chooser_box.insert_child_after(&question_label, None::<&Widget>);
        chooser_box.set_margin_top(5);
        chooser_box.set_margin_bottom(5);
        chooser_box.set_margin_start(5);
        chooser_box.set_margin_end(5);

        chooser_box.show();

        chooser.show();
        connect_renaming_response(&chooser, &shared_result_entries, &list_store, &window_main);
    });
}

fn connect_renaming_response(chooser: &Dialog, shared_result_entries: &Rc<RefCell<ResultEntries>>, list_store: &ListStore, window_main: &Window) {
    let shared_result_entries = shared_result_entries.clone();
    let list_store = list_store.clone();
    let window_main = window_main.clone();

    chooser.connect_response(move |chooser, response_type| {
        if response_type == gtk4::ResponseType::Ok {
            let mut shared_result_entries = shared_result_entries.borrow_mut();
            let shared_result_entries = &mut *shared_result_entries;
            // Before renaming, After possible renaming, Cause
            let mut failed_renames: Vec<(String, String, String)> = Vec::new();
            let mut properly_renamed = 0;
            let mut ignored = 0;

            let tree_iter = list_store.iter_first().unwrap();
            let mut file_renames: Vec<(String, String)> = Vec::new();
            let mut folder_renames: BTreeMap<usize, Vec<(String, String)>> = Default::default();

            loop {
                let path = list_store.get::<String>(&tree_iter, ColumnsResults::Path as i32);
                let old_name = format!("{}{}{}", path, CHARACTER, list_store.get::<String>(&tree_iter, ColumnsResults::CurrentName as i32));
                let new_name = format!("{}{}{}", path, CHARACTER, list_store.get::<String>(&tree_iter, ColumnsResults::FutureName as i32));
                let typ = to_dir_file_from_u8(list_store.get::<u8>(&tree_iter, ColumnsResults::Type as i32));

                match typ {
                    DirFileType::Directory => {
                        let how_much = old_name.matches(CHARACTER).count();
                        folder_renames.entry(how_much).or_insert_with(Vec::new);
                        folder_renames.get_mut(&how_much).unwrap().push((old_name, new_name));
                    }
                    DirFileType::File => {
                        file_renames.push((old_name, new_name));
                    }
                }

                if !list_store.iter_next(&tree_iter) {
                    break;
                }
            }

            for (old_name, new_name) in file_renames {
                rename_items(old_name, new_name, &mut ignored, &mut properly_renamed, &mut failed_renames);
            }
            for (_size, vec) in folder_renames.iter().rev() {
                for (old_name, new_name) in vec {
                    rename_items(old_name.clone(), new_name.clone(), &mut ignored, &mut properly_renamed, &mut failed_renames);
                }
            }
            // Print results
            create_results_dialog(&window_main, properly_renamed, ignored, failed_renames);

            // TODO not properly converted items should not be cleared
            list_store.clear();
            shared_result_entries.files.clear();
        }
        chooser.close();
    });
}

fn rename_items(old_name: String, new_name: String, ignored: &mut u32, properly_renamed: &mut u32, failed_renames: &mut Vec<(String, String, String)>) {
    if new_name == old_name {
        *ignored += 1;
    } else if Path::new(&new_name).exists() {
        failed_renames.push((old_name, new_name, fls!("renaming_destination_file_exists")));
    } else if let Err(e) = fs::rename(&old_name, &new_name) {
        failed_renames.push((old_name, new_name, e.to_string()));
    } else {
        *properly_renamed += 1;
    }
}

fn create_results_dialog(window_main: &Window, properly_renamed: u32, ignored: u32, failed_vector: Vec<(String, String, String)>) {
    let chooser = Dialog::with_buttons(
        Some(fls!("dialog_results_of_renaming")),
        Some(window_main),
        DialogFlags::DESTROY_WITH_PARENT,
        &[(&fls!("dialog_button_ok"), gtk4::ResponseType::Ok)],
    );

    let label_good_name = fls!(
        "renaming_renamed_files",
        generate_translation_hashmap(vec![("properly_renamed", properly_renamed.to_string())])
    );
    let label_ignored_name = fls!("renaming_ignored_files", generate_translation_hashmap(vec![("ignored", ignored.to_string())]));

    let label_good = Label::new(Some(label_good_name.as_str()));
    let label_ignored = Label::new(Some(label_ignored_name.as_str()));

    let chooser_box = get_dialog_box_child(&chooser);
    chooser_box.set_margin_top(5);
    chooser_box.set_margin_bottom(5);
    chooser_box.set_margin_start(5);
    chooser_box.set_margin_end(5);

    let label_good_name = fls!(
        "renaming_failed_files",
        generate_translation_hashmap(vec![("failed_vector", failed_vector.len().to_string())])
    );
    let label_bad = Label::new(Some(label_good_name.as_str()));

    chooser_box.insert_child_after(&label_good, None::<&Widget>);
    chooser_box.insert_child_after(&label_ignored, Some(&label_good));
    chooser_box.insert_child_after(&label_bad, Some(&label_ignored));

    if !failed_vector.is_empty() {
        chooser.set_default_size(800, 200);
        let label_info_bad = Label::new(Some(fls!("renaming_list_of_failed_to_rename").as_str()));
        label_info_bad.set_margin_top(10);
        chooser_box.insert_child_after(&label_info_bad, Some(&label_bad));

        let txt_op1: Option<&gtk4::TextTagTable> = None;

        let scrolled_window = ScrolledWindow::new();
        let text_view = TextView::new();
        text_view.set_hexpand(true);
        text_view.set_vexpand(true);
        let buffer = gtk4::TextBuffer::new(txt_op1);
        text_view.set_buffer(Some(&buffer));
        let mut text = String::new();

        let text_err = fls!("renaming_error");

        for i in failed_vector {
            text.push_str(format!("{} -> {}, {text_err}: {}\n", i.0, i.1, i.2).as_str());
        }
        buffer.set_text(&text);

        scrolled_window.set_child(Some(&text_view));
        // TODO Created Scrolled Window with explanation about each failed example of renaming
        chooser_box.insert_child_after(&scrolled_window, Some(&label_info_bad));
    }

    chooser_box.show();

    chooser.connect_response(|chooser, _| chooser.close());
    chooser.show();
}
