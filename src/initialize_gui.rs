use crate::create_tree_view::{create_tree_view_results, create_tree_view_rules};
use crate::example_fields::update_examples;
use crate::gui_data::GuiData;
use crate::help_function::{ColumnsResults, CHARACTER};
use crate::language_functions::LANGUAGES_ALL;
use crate::notebook_enum::EXAMPLE_NAME;
use glib::Type;
use gtk::prelude::*;
use gtk::{ScrolledWindow, SelectionMode};

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

        let col_types: [Type; 7] = [
            glib::types::Type::STRING,
            glib::types::Type::STRING,
            glib::types::Type::STRING,
            glib::types::Type::STRING,
            glib::types::Type::U64,
            glib::types::Type::U64,
            glib::types::Type::U64,
        ];

        let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

        let tree_view: gtk::TreeView = gui_data.results.tree_view_results.clone();
        tree_view.set_model(Some(&list_store));

        tree_view.selection().set_mode(SelectionMode::Multiple);

        tree_view.connect_button_press_event(|tree_view, event| {
            if event.event_type() == gdk::EventType::DoubleButtonPress && event.button() == 1 {
                common_open_function(tree_view, OpenMode::PathAndName);
            } else if event.event_type() == gdk::EventType::DoubleButtonPress && event.button() == 3 {
                common_open_function(tree_view, OpenMode::OnlyPath);
            }
            gtk::Inhibit(false)
        });

        create_tree_view_results(&tree_view);

        scrolled_window_results.add(&tree_view);
        scrolled_window_results.show_all();
    }
    // Create TreeView in Rules
    {
        let scrolled_window_rules: ScrolledWindow = gui_data.rules_bottom_panel.scrolled_window_rules.clone();

        let col_types: [Type; 3] = [glib::types::Type::STRING, glib::types::Type::STRING, glib::types::Type::STRING];

        let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

        let tree_view: gtk::TreeView = gui_data.rules_bottom_panel.tree_view_window_rules.clone();
        tree_view.set_model(Some(&list_store));

        // tree_view.selection().set_mode(SelectionMode::Multiple);

        create_tree_view_rules(&tree_view);

        scrolled_window_rules.add(&tree_view);
        scrolled_window_rules.show_all();
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

pub fn common_open_function(tree_view: &gtk::TreeView, opening_mode: OpenMode) {
    let selection = tree_view.selection();
    let (selection_rows, tree_model) = selection.selected_rows();

    for tree_path in selection_rows.iter().rev() {
        let end_path;
        let current_name = tree_model.value(&tree_model.iter(tree_path).unwrap(), ColumnsResults::CurrentName as i32).get::<String>().unwrap();
        let path = tree_model.value(&tree_model.iter(tree_path).unwrap(), ColumnsResults::Path as i32).get::<String>().unwrap();

        match opening_mode {
            OpenMode::OnlyPath => {
                end_path = path;
            }
            OpenMode::PathAndName => {
                end_path = format!("{}{}{}", path, CHARACTER, current_name);
            }
        }

        open::that_in_background(&end_path);
        // if open::that(&end_path).is_err() {
        //     println!("Failed to open {}", end_path);
        // }
    }
}
