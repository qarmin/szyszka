use crate::class_gui_data::GuiData;
use crate::create_tree_view::{create_tree_view_results, create_tree_view_rules};
use crate::example_fields::update_examples;
use crate::help_function::{ColumnsResults, CHARACTER};
use crate::notebook_enum::EXAMPLE_NAME;
use glib::prelude::*;
use glib::Type;
use gtk::prelude::*;
use gtk::{ScrolledWindow, SelectionMode, TreeView};

pub fn initialize_gui(gui_data: &mut GuiData) {
    // Create TreeView in Scrolled Window
    {
        let scrolled_window_results: ScrolledWindow = gui_data.results.scrolled_window_results.clone();

        let col_types: [Type; 6] = [
            glib::types::Type::String,
            glib::types::Type::String,
            glib::types::Type::String,
            glib::types::Type::U64,
            glib::types::Type::U64,
            glib::types::Type::U64,
        ];

        let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

        let tree_view: gtk::TreeView = TreeView::with_model(&list_store);

        tree_view.get_selection().set_mode(SelectionMode::Multiple);

        tree_view.connect_button_press_event(|tree_view, event| {
            if event.get_event_type() == gdk::EventType::DoubleButtonPress && event.get_button() == 1 {
                common_open_function(tree_view, OpenMode::PathAndName);
            } else if event.get_event_type() == gdk::EventType::DoubleButtonPress && event.get_button() == 3 {
                common_open_function(tree_view, OpenMode::OnlyPath);
            }
            gtk::Inhibit(false)
        });

        create_tree_view_results(&tree_view);

        scrolled_window_results.add(&tree_view);
        scrolled_window_results.show_all();

        gui_data.results.tree_view_results = tree_view;
    }
    // Create TreeView in Rules
    {
        gui_data.rules_bottom_panel.button_one_up.hide();
        gui_data.rules_bottom_panel.button_one_down.hide();
        gui_data.rules_bottom_panel.button_to_top.hide();
        gui_data.rules_bottom_panel.button_to_bottom.hide();

        let scrolled_window_rules: ScrolledWindow = gui_data.rules_bottom_panel.scrolled_window_rules.clone();

        let col_types: [Type; 3] = [glib::types::Type::String, glib::types::Type::String, glib::types::Type::String];

        let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

        let tree_view: gtk::TreeView = TreeView::with_model(&list_store);

        // tree_view.get_selection().set_mode(SelectionMode::Multiple);

        create_tree_view_rules(&tree_view);

        scrolled_window_rules.add(&tree_view);
        scrolled_window_rules.show_all();

        gui_data.rules_bottom_panel.tree_view_window_rules = tree_view;
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
    let selection = tree_view.get_selection();
    let (selection_rows, tree_model) = selection.get_selected_rows();

    for tree_path in selection_rows.iter().rev() {
        let end_path;
        let current_name = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(), ColumnsResults::CurrentName as i32).get::<String>().unwrap().unwrap();
        let path = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(), ColumnsResults::Path as i32).get::<String>().unwrap().unwrap();

        match opening_mode {
            OpenMode::OnlyPath => {
                end_path = path;
            }
            OpenMode::PathAndName => {
                end_path = format!("{}{}{}", path, CHARACTER, current_name);
            }
        }

        match open::that(&end_path) {
            Ok(t) => {
                if !t.success() {
                    println!("Failed to open {}, status {:?}", end_path, t.code());
                }
            }
            Err(_) => {
                println!("Failed to open {}", end_path);
            }
        }
    }
}
