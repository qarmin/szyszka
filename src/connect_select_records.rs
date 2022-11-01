use crate::gui_data::GuiData;
use crate::help_function::{get_all_boxes_from_widget, get_list_store_from_tree_view, regex_check, ColumnsResults};
use gtk4::prelude::*;
use gtk4::{PositionType, TreeIter};

pub fn connect_select_records(gui_data: &GuiData) {
    let popover_select = gui_data.popover_select.popover_select.clone();

    let button_select = gui_data.upper_buttons.menu_button_select_popup.clone();
    button_select.set_popover(Some(&popover_select));
}
pub fn connect_select_all(gui_data: &GuiData) {
    let popover_select = gui_data.popover_select.popover_select.clone();
    let button_select_all = gui_data.popover_select.button_select_all.clone();

    let tree_view = gui_data.results.tree_view_results.clone();

    button_select_all.connect_clicked(move |_e| {
        let selection = tree_view.selection();

        selection.select_all();
        popover_select.popdown();
    });
}

pub fn connect_select_reverse(gui_data: &GuiData) {
    let popover_select = gui_data.popover_select.popover_select.clone();
    let button_select_reverse = gui_data.popover_select.button_select_reverse.clone();

    let tree_view = gui_data.results.tree_view_results.clone();

    button_select_reverse.connect_clicked(move |_e| {
        let selection = tree_view.selection();

        let (vector_tree_path, tree_model) = selection.selected_rows();

        if vector_tree_path.is_empty() {
            selection.select_all();
        } else {
            let tree_iter_all = tree_model.iter_first().unwrap(); // Never should be available button where there is no available records

            let mut current_path_index = 0;
            let mut tree_iter_selected: TreeIter;
            loop {
                if current_path_index >= vector_tree_path.len() {
                    selection.select_iter(&tree_iter_all);
                } else {
                    tree_iter_selected = tree_model.iter(vector_tree_path.get(current_path_index).unwrap()).unwrap();
                    if tree_model.path(&tree_iter_all) == tree_model.path(&tree_iter_selected) {
                        selection.unselect_iter(&tree_iter_selected);
                        current_path_index += 1;
                    } else {
                        selection.select_iter(&tree_iter_all);
                    }
                }
                if !tree_model.iter_next(&tree_iter_all) {
                    break;
                }
            }
        }

        popover_select.popdown();
    });
}
pub fn connect_select_changed(gui_data: &GuiData) {
    let popover_select = gui_data.popover_select.popover_select.clone();
    let button_select_changed = gui_data.popover_select.button_select_changed.clone();

    let tree_view = gui_data.results.tree_view_results.clone();

    button_select_changed.connect_clicked(move |_e| {
        let selection = tree_view.selection();
        let model = get_list_store_from_tree_view(&tree_view);

        if let Some(iter) = model.iter_first() {
            loop {
                let old_name = model.get::<String>(&iter, ColumnsResults::CurrentName as i32);
                let new_name = model.get::<String>(&iter, ColumnsResults::FutureName as i32);

                if new_name != old_name {
                    selection.select_iter(&iter);
                }

                if !model.iter_next(&iter) {
                    break;
                }
            }
        }
        popover_select.popdown();
    });
}
pub fn connect_unselect_changed(gui_data: &GuiData) {
    let popover_select = gui_data.popover_select.popover_select.clone();
    let button_unselect_changed = gui_data.popover_select.button_unselect_changed.clone();

    let tree_view = gui_data.results.tree_view_results.clone();

    button_unselect_changed.connect_clicked(move |_e| {
        let selection = tree_view.selection();
        let model = get_list_store_from_tree_view(&tree_view);

        if let Some(iter) = model.iter_first() {
            loop {
                let old_name = model.get::<String>(&iter, ColumnsResults::CurrentName as i32);
                let new_name = model.get::<String>(&iter, ColumnsResults::FutureName as i32);

                if new_name != old_name {
                    selection.unselect_iter(&iter);
                }

                if !model.iter_next(&iter) {
                    break;
                }
            }
        }
        popover_select.popdown();
    });
}

pub fn connect_select_custom(gui_data: &GuiData) {
    let popover_select = gui_data.popover_select.popover_select.clone();
    let button_select_custom = gui_data.popover_select.button_select_custom.clone();

    let tree_view = gui_data.results.tree_view_results.clone();
    let gui_data = gui_data.clone();

    button_select_custom.connect_clicked(move |_e| {
        popover_select.popdown();

        enum WildcardType {
            Path,
            CurrentName,
            FutureName,
            PathCurrentName,
            PathFutureName,
            IsDir,
        }

        // Accept Dialog
        {
            let window_main = gui_data.window_main.clone();
            let confirmation_dialog_delete = gtk4::Dialog::with_buttons(Some("Select custom"), Some(&window_main), gtk4::DialogFlags::MODAL, &[("Ok", gtk4::ResponseType::Ok), ("Close", gtk4::ResponseType::Cancel)]);
            let label: gtk4::Label = gtk4::Label::new(Some("Usage: */folder-nr*/* or name-version-*.txt"));

            let radio_path = gtk4::CheckButton::with_label("Path");
            let radio_current_name = gtk4::CheckButton::with_label("Current Name");
            let radio_future_name = gtk4::CheckButton::with_label("Future Name");
            let radio_current_name_path = gtk4::CheckButton::with_label("Path + Current Name");
            let radio_future_name_path = gtk4::CheckButton::with_label("Path + Future Name");
            let radio_is_dir = gtk4::CheckButton::with_label("Directory/File");

            let entry_path = gtk4::Entry::new();
            let entry_current_name = gtk4::Entry::new();
            let entry_future_name = gtk4::Entry::new();
            let entry_current_name_path = gtk4::Entry::new();
            let entry_future_name_path = gtk4::Entry::new();
            let check_button_is_dir = gtk4::CheckButton::new();

            check_button_is_dir.set_label(Some("Select Directory"));

            label.set_margin_bottom(5);
            label.set_margin_end(5);
            label.set_margin_start(5);

            // TODO Label should have const width, and rest should fill entry, but for now is 50%-50%
            let grid = gtk4::Grid::new();
            grid.set_row_homogeneous(true);
            grid.set_column_homogeneous(true);

            grid.attach(&label, 0, 0, 2, 1);

            grid.attach(&radio_path, 0, 1, 1, 1);
            grid.attach(&radio_current_name, 0, 2, 1, 1);
            grid.attach(&radio_future_name, 0, 3, 1, 1);
            grid.attach(&radio_current_name_path, 0, 4, 1, 1);
            grid.attach(&radio_future_name_path, 0, 5, 1, 1);
            grid.attach(&radio_is_dir, 0, 6, 1, 1);

            grid.attach(&entry_path, 1, 1, 1, 1);
            grid.attach(&entry_current_name, 1, 2, 1, 1);
            grid.attach(&entry_future_name, 1, 3, 1, 1);
            grid.attach(&entry_current_name_path, 1, 4, 1, 1);
            grid.attach(&entry_future_name_path, 1, 5, 1, 1);
            grid.attach(&check_button_is_dir, 1, 6, 1, 1);

            get_all_boxes_from_widget(&confirmation_dialog_delete)[0].clone().append(&grid);

            confirmation_dialog_delete.show();

            let tree_view = tree_view.clone();
            confirmation_dialog_delete.connect_response(move |_chooser, response_type| {
                let wildcard_type: WildcardType;
                let wildcard: String;
                if response_type == gtk4::ResponseType::Ok {
                    if radio_path.is_active() {
                        wildcard_type = WildcardType::Path;
                        wildcard = entry_path.text().to_string();
                    } else if radio_current_name.is_active() {
                        wildcard_type = WildcardType::CurrentName;
                        wildcard = entry_current_name.text().to_string();
                    } else if radio_future_name.is_active() {
                        wildcard_type = WildcardType::FutureName;
                        wildcard = entry_future_name.text().to_string();
                    } else if radio_current_name_path.is_active() {
                        wildcard_type = WildcardType::PathCurrentName;
                        wildcard = entry_current_name_path.text().to_string();
                    } else if radio_future_name_path.is_active() {
                        wildcard_type = WildcardType::PathFutureName;
                        wildcard = entry_future_name_path.text().to_string();
                    } else if radio_is_dir.is_active() {
                        wildcard_type = WildcardType::IsDir;
                        wildcard = match check_button_is_dir.is_active() {
                            true => "Dir".to_string(),
                            false => "File".to_string(),
                        };
                    } else {
                        panic!("Non handled option in select wildcard");
                    }

                    if !wildcard.is_empty() {
                        let wildcard = wildcard.trim();

                        #[cfg(target_family = "windows")]
                        let wildcard = wildcard.replace("/", "\\");
                        #[cfg(target_family = "windows")]
                        let wildcard = wildcard.as_str();

                        let selection = tree_view.selection();
                        let tree_model = tree_view.model().unwrap();

                        let tree_iter = tree_model.iter_first().unwrap(); // Never should be available button where there is no available records

                        loop {
                            let typ = tree_model.get::<String>(&tree_iter, ColumnsResults::Type as i32);
                            let path = tree_model.get::<String>(&tree_iter, ColumnsResults::Path as i32);
                            let current_name = tree_model.get::<String>(&tree_iter, ColumnsResults::CurrentName as i32);
                            let future_name = tree_model.get::<String>(&tree_iter, ColumnsResults::CurrentName as i32);
                            match wildcard_type {
                                WildcardType::Path => {
                                    if regex_check(wildcard, path) {
                                        selection.select_iter(&tree_iter);
                                    }
                                }
                                WildcardType::CurrentName => {
                                    if regex_check(wildcard, current_name) {
                                        selection.select_iter(&tree_iter);
                                    }
                                }
                                WildcardType::FutureName => {
                                    if regex_check(wildcard, future_name) {
                                        selection.select_iter(&tree_iter);
                                    }
                                }
                                WildcardType::PathCurrentName => {
                                    if regex_check(wildcard, format!("{}/{}", path, current_name)) {
                                        selection.select_iter(&tree_iter);
                                    }
                                }
                                WildcardType::PathFutureName => {
                                    if regex_check(wildcard, format!("{}/{}", path, future_name)) {
                                        selection.select_iter(&tree_iter);
                                    }
                                }
                                WildcardType::IsDir => {
                                    if wildcard == "Dir" {
                                        if typ == "Dir" {
                                            selection.select_iter(&tree_iter);
                                        }
                                    } else if wildcard == "File" {
                                        if typ == "File" {
                                            selection.select_iter(&tree_iter);
                                        }
                                    } else {
                                        panic!();
                                    }
                                }
                            }

                            if !tree_model.iter_next(&tree_iter) {
                                break;
                            }
                        }
                    }
                }
            });
        }
    });
}

pub fn connect_unselect_custom(gui_data: &GuiData) {
    let popover_select = gui_data.popover_select.popover_select.clone();
    let button_unselect_custom = gui_data.popover_select.button_unselect_custom.clone();

    let tree_view = gui_data.results.tree_view_results.clone();
    let gui_data = gui_data.clone();

    button_unselect_custom.connect_clicked(move |_e| {
        popover_select.popdown();

        enum WildcardType {
            Path,
            CurrentName,
            FutureName,
            PathCurrentName,
            PathFutureName,
            IsDir,
        }

        // Accept Dialog
        {
            let window_main = gui_data.window_main.clone();
            let confirmation_dialog_delete = gtk4::Dialog::with_buttons(Some("Unselect custom"), Some(&window_main), gtk4::DialogFlags::MODAL, &[("Ok", gtk4::ResponseType::Ok), ("Close", gtk4::ResponseType::Cancel)]);
            let label: gtk4::Label = gtk4::Label::new(Some("Usage: */folder-nr*/* or name-version-*.txt"));

            let radio_path = gtk4::CheckButton::with_label("Path");
            let radio_current_name = gtk4::CheckButton::with_label("Current Name");
            let radio_future_name = gtk4::CheckButton::with_label("Future Name");
            let radio_current_name_path = gtk4::CheckButton::with_label("Path + Current Name");
            let radio_future_name_path = gtk4::CheckButton::with_label("Path + Future Name");
            let radio_is_dir = gtk4::CheckButton::with_label("Directory/File");

            let entry_path = gtk4::Entry::new();
            let entry_current_name = gtk4::Entry::new();
            let entry_future_name = gtk4::Entry::new();
            let entry_current_name_path = gtk4::Entry::new();
            let entry_future_name_path = gtk4::Entry::new();
            let check_button_is_dir = gtk4::CheckButton::new();

            check_button_is_dir.set_label(Some("Unselect Directory"));

            label.set_margin_bottom(5);
            label.set_margin_end(5);
            label.set_margin_start(5);

            // TODO Label should have const width, and rest should fill entry, but for now is 50%-50%
            let grid = gtk4::Grid::new();
            grid.set_row_homogeneous(true);
            grid.set_column_homogeneous(true);

            grid.attach(&label, 0, 0, 2, 1);

            grid.attach(&radio_path, 0, 1, 1, 1);
            grid.attach(&radio_current_name, 0, 2, 1, 1);
            grid.attach(&radio_future_name, 0, 3, 1, 1);
            grid.attach(&radio_current_name_path, 0, 4, 1, 1);
            grid.attach(&radio_future_name_path, 0, 5, 1, 1);
            grid.attach(&radio_is_dir, 0, 6, 1, 1);

            grid.attach(&entry_path, 1, 1, 1, 1);
            grid.attach(&entry_current_name, 1, 2, 1, 1);
            grid.attach(&entry_future_name, 1, 3, 1, 1);
            grid.attach(&entry_current_name_path, 1, 4, 1, 1);
            grid.attach(&entry_future_name_path, 1, 5, 1, 1);
            grid.attach(&check_button_is_dir, 1, 6, 1, 1);

            get_all_boxes_from_widget(&confirmation_dialog_delete)[0].clone().append(&grid);

            confirmation_dialog_delete.show();

            let tree_view = tree_view.clone();
            confirmation_dialog_delete.connect_response(move |_, response| {
                if response == gtk4::ResponseType::Ok {
                    let wildcard: String;
                    let wildcard_type: WildcardType;

                    if radio_path.is_active() {
                        wildcard_type = WildcardType::Path;
                        wildcard = entry_path.text().to_string();
                    } else if radio_current_name.is_active() {
                        wildcard_type = WildcardType::CurrentName;
                        wildcard = entry_current_name.text().to_string();
                    } else if radio_future_name.is_active() {
                        wildcard_type = WildcardType::FutureName;
                        wildcard = entry_future_name.text().to_string();
                    } else if radio_current_name_path.is_active() {
                        wildcard_type = WildcardType::PathCurrentName;
                        wildcard = entry_current_name_path.text().to_string();
                    } else if radio_future_name_path.is_active() {
                        wildcard_type = WildcardType::PathFutureName;
                        wildcard = entry_future_name_path.text().to_string();
                    } else if radio_is_dir.is_active() {
                        wildcard_type = WildcardType::IsDir;
                        wildcard = match check_button_is_dir.is_active() {
                            true => "Dir".to_string(),
                            false => "File".to_string(),
                        };
                    } else {
                        panic!("Non handled option in select wildcard");
                    }

                    if !wildcard.is_empty() {
                        let wildcard = wildcard.trim();

                        #[cfg(target_family = "windows")]
                        let wildcard = wildcard.replace("/", "\\");
                        #[cfg(target_family = "windows")]
                        let wildcard = wildcard.as_str();

                        let selection = tree_view.selection();
                        let tree_model = tree_view.model().unwrap();

                        let tree_iter = tree_model.iter_first().unwrap(); // Never should be available button where there is no available records

                        loop {
                            let typ = tree_model.get::<String>(&tree_iter, ColumnsResults::Type as i32);
                            let path = tree_model.get::<String>(&tree_iter, ColumnsResults::Path as i32);
                            let current_name = tree_model.get::<String>(&tree_iter, ColumnsResults::CurrentName as i32);
                            let future_name = tree_model.get::<String>(&tree_iter, ColumnsResults::CurrentName as i32);
                            match wildcard_type {
                                WildcardType::Path => {
                                    if regex_check(wildcard, path) {
                                        selection.unselect_iter(&tree_iter);
                                    }
                                }
                                WildcardType::CurrentName => {
                                    if regex_check(wildcard, current_name) {
                                        selection.unselect_iter(&tree_iter);
                                    }
                                }
                                WildcardType::FutureName => {
                                    if regex_check(wildcard, future_name) {
                                        selection.unselect_iter(&tree_iter);
                                    }
                                }
                                WildcardType::PathCurrentName => {
                                    if regex_check(wildcard, format!("{}/{}", path, current_name)) {
                                        selection.unselect_iter(&tree_iter);
                                    }
                                }
                                WildcardType::PathFutureName => {
                                    if regex_check(wildcard, format!("{}/{}", path, future_name)) {
                                        selection.unselect_iter(&tree_iter);
                                    }
                                }
                                WildcardType::IsDir => {
                                    if wildcard == "Dir" {
                                        if typ == "Dir" {
                                            selection.unselect_iter(&tree_iter);
                                        }
                                    } else if wildcard == "File" {
                                        if typ == "File" {
                                            selection.unselect_iter(&tree_iter);
                                        }
                                    } else {
                                        panic!();
                                    }
                                }
                            }

                            if !tree_model.iter_next(&tree_iter) {
                                break;
                            }
                        }
                    }
                }
            });
        }
    });
}
