use glib::Type;
use gtk4::prelude::*;
use gtk4::{GestureClick, ScrolledWindow, SelectionMode};

use crate::create_tree_view::{create_tree_view_results, create_tree_view_rules};
use crate::example_fields::update_examples;
use crate::gui_data::GuiData;
use crate::help_function::{ColumnsResults, CHARACTER};
use crate::language_functions::LANGUAGES_ALL;
use crate::notebook_enum::EXAMPLE_NAME;

pub fn initialize_gui(gui_data: &GuiData) {
    // Setup Languages
    {
        let combo_box_settings_language = gui_data.settings.combo_box_settings_language.clone();
        for lang in LANGUAGES_ALL {
            combo_box_settings_language.append_text(lang.combo_box_text);
        }
        combo_box_settings_language.set_active(Some(0));
    }
    // Create TreeView in Scrolled Window
    {
        let scrolled_window_results: ScrolledWindow = gui_data.results.scrolled_window_results.clone();

        let col_types: [Type; 7] = [Type::STRING, Type::STRING, Type::STRING, Type::STRING, Type::U64, Type::U64, Type::U64];

        let list_store: gtk4::ListStore = gtk4::ListStore::new(&col_types);

        let tree_view: gtk4::TreeView = gui_data.results.tree_view_results.clone();
        let gc_tree_view: GestureClick = gui_data.results.gc_tree_view_results.clone();
        tree_view.set_model(Some(&list_store));

        tree_view.selection().set_mode(SelectionMode::Multiple);

        gc_tree_view.set_button(0);
        gc_tree_view.connect_pressed(|gesture_click: &GestureClick, number_of_clicks: i32, _b: f64, _c: f64| {
            let tree_view = gesture_click.widget().downcast::<gtk4::TreeView>().unwrap();

            if number_of_clicks == 2 {
                if gesture_click.current_button() == 1 {
                    common_open_function(&tree_view, ColumnsResults::CurrentName as i32, ColumnsResults::Path as i32, &OpenMode::PathAndName);
                } else if gesture_click.current_button() == 3 {
                    common_open_function(&tree_view, ColumnsResults::CurrentName as i32, ColumnsResults::Path as i32, &OpenMode::OnlyPath);
                }
            }
        });

        create_tree_view_results(&tree_view);

        scrolled_window_results.set_child(Some(&tree_view));
        scrolled_window_results.show();
    }
    // Create TreeView in Rules
    {
        let scrolled_window_rules: ScrolledWindow = gui_data.rules_bottom_panel.scrolled_window_rules.clone();

        let col_types: [Type; 3] = [Type::STRING, Type::STRING, Type::STRING];

        let list_store: gtk4::ListStore = gtk4::ListStore::new(&col_types);

        let tree_view: gtk4::TreeView = gui_data.rules_bottom_panel.tree_view_window_rules.clone();
        tree_view.set_model(Some(&list_store));

        // tree_view.selection().set_mode(SelectionMode::Multiple);

        create_tree_view_rules(&tree_view);

        scrolled_window_rules.set_child(Some(&tree_view));
        scrolled_window_rules.show();
    }
    // Set Example name
    {
        let entry_example_before = gui_data.window_rules.entry_example_before.clone();
        entry_example_before.set_text(EXAMPLE_NAME);

        update_examples(&gui_data.window_rules, None);
    }
}

pub enum OpenMode {
    OnlyPath,
    PathAndName,
}

fn common_open_function(tree_view: &gtk4::TreeView, column_name: i32, column_path: i32, opening_mode: &OpenMode) {
    let selection = tree_view.selection();
    let (selected_rows, tree_model) = selection.selected_rows();

    for tree_path in selected_rows.iter().rev() {
        let name = tree_model.get::<String>(&tree_model.iter(tree_path).unwrap(), column_name);
        let path = tree_model.get::<String>(&tree_model.iter(tree_path).unwrap(), column_path);

        let end_path = match opening_mode {
            OpenMode::OnlyPath => path,
            OpenMode::PathAndName => get_full_name_from_path_name(&path, &name),
        };

        if let Err(e) = open::that(&end_path) {
            println!("Failed to open file {end_path}, reason {e}");
        };
    }
}

pub fn get_full_name_from_path_name(path: &str, name: &str) -> String {
    let mut string = String::with_capacity(path.len() + name.len() + 1);
    string.push_str(path);
    string.push(CHARACTER);
    string.push_str(name);
    string
}
