use crate::class_gui_data::GuiData;
use crate::help_function::{count_rows_in_tree_view, create_message_window, get_list_store_from_tree_view, ColumnsResults, CHARACTER};
use gtk::prelude::*;
use gtk::{DialogFlags, ScrolledWindow, TextTagTable, TextView};
use std::fs;
use std::path::Path;

pub fn connect_start_renaming(gui_data: &GuiData) {
    let button_start_rename = gui_data.upper_buttons.button_start_rename.clone();
    let window_main = gui_data.window_main.clone();

    let tree_view_results = gui_data.results.tree_view_results.clone();
    let rules = gui_data.rules.clone();

    let list_store = get_list_store_from_tree_view(&tree_view_results);

    button_start_rename.connect_clicked(move |_e| {
        let number_of_renamed_files = count_rows_in_tree_view(&tree_view_results);
        if number_of_renamed_files == 0 {
            create_message_window(&window_main, "Missing Files", "You need to use at least 1 file");
            return;
        }
        let rules = rules.borrow();
        if rules.rules.is_empty() {
            create_message_window(&window_main, "Missing Rules", "You need to use at least 1 rule");
            return;
        }

        let chooser = gtk::Dialog::with_buttons(Some("Confirm renaming"), Some(&window_main), DialogFlags::DESTROY_WITH_PARENT, &[("Ok", gtk::ResponseType::Ok), ("Close", gtk::ResponseType::Cancel)]);

        let question_label = gtk::Label::new(Some(format!("Are you sure that you want to rename {} files", number_of_renamed_files).as_str()));

        let chooser_box = chooser.children()[0].clone().downcast::<gtk::Box>().unwrap();
        chooser_box.add(&question_label);
        chooser_box.show_all();

        // Before renaming, After possible renaming, Cause
        let mut failed_renames: Vec<(String, String, String)> = Vec::new();
        let mut properly_renamed = 0;
        let mut ignored = 0;

        let response_type = chooser.run();
        if response_type == gtk::ResponseType::Ok {
            let tree_iter = list_store.iter_first().unwrap();
            loop {
                let path = list_store.value(&tree_iter, ColumnsResults::Path as i32).get::<String>().unwrap();
                let old_name = format!("{}{}{}", path, CHARACTER, list_store.value(&tree_iter, ColumnsResults::CurrentName as i32).get::<String>().unwrap());
                let new_name = format!("{}{}{}", path, CHARACTER, list_store.value(&tree_iter, ColumnsResults::FutureName as i32).get::<String>().unwrap());

                // TODO Find method to not overwrite new function
                #[allow(clippy::collapsible_else_if)]
                if new_name == old_name {
                    ignored += 1
                } else if Path::new(&new_name).exists() {
                    failed_renames.push((old_name, new_name, "Destination file already exists.".to_string()));
                } else {
                    if let Err(e) = fs::rename(&old_name, &new_name) {
                        failed_renames.push((old_name, new_name, e.to_string()));
                    } else {
                        properly_renamed += 1;
                    }
                }

                if !list_store.iter_next(&tree_iter) {
                    break;
                }
            }
        }

        chooser.close();
        chooser.hide(); // Workaround, 2 dialogs cannot be run one after one

        // Print results
        create_results_dialog(&window_main, properly_renamed, ignored, failed_renames);

        list_store.clear();
    });
}

fn create_results_dialog(window_main: &gtk::Window, properly_renamed: u32, ignored: u32, failed_vector: Vec<(String, String, String)>) {
    let chooser = gtk::Dialog::with_buttons(Some("Results of renaming"), Some(window_main), DialogFlags::DESTROY_WITH_PARENT, &[("Ok", gtk::ResponseType::Ok)]);

    let label_good = gtk::Label::new(Some(format!("Properly renamed {} files", properly_renamed).as_str()));
    let label_ignored = gtk::Label::new(Some(format!("Ignored {} files, because the name before and after the change are the same.", ignored).as_str()));

    let chooser_box = chooser.children()[0].clone().downcast::<gtk::Box>().unwrap();
    chooser_box.add(&label_good);
    chooser_box.add(&label_ignored);

    let label_bad = gtk::Label::new(Some(format!("Failed to rename {} files", failed_vector.len()).as_str()));
    chooser_box.add(&label_bad);

    if !failed_vector.is_empty() {
        chooser.set_default_size(800, 200);
        let label_info_bad = gtk::Label::new(Some("List of all failing renames"));
        label_info_bad.set_margin_top(10);
        chooser_box.add(&label_info_bad);

        let scrolled_window = ScrolledWindow::new::<gtk::Adjustment, gtk::Adjustment>(None, None);
        let text_view = TextView::new();
        text_view.set_expand(true);
        let buffer = gtk::TextBuffer::new::<TextTagTable>(None);
        text_view.set_buffer(Some(&buffer));
        let mut text = "".to_string();

        for i in failed_vector {
            text.push_str(i.0.as_str());
            text.push_str(" -> ");
            text.push_str(i.1.as_str());
            text.push_str(", error: ");
            text.push_str(i.2.as_str());
            text.push('\n');
        }
        buffer.set_text(&text);

        scrolled_window.add(&text_view);
        // TODO Created Scrolled Window with explanation about each failed example of renaming
        chooser_box.add(&scrolled_window);
    }

    chooser_box.show_all();

    chooser.run();
    chooser.close();
}
