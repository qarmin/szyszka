use glib::Type;
use gtk4::prelude::*;
use gtk4::{GestureClick, ListStore, ScrolledWindow, SelectionMode, TreeView};

use crate::create_tree_view::{create_tree_view_results, create_tree_view_rules};
use crate::example_fields::update_examples;
use crate::gui_data_things::gui_data::GuiData;
use crate::help_function::{ColumnsResults, CHARACTER};
use crate::language_functions::LANGUAGES_ALL;
use crate::notebook_enum::EXAMPLE_NAME;

pub fn initialize_gui(gui_data: &GuiData) {
    setup_languages(gui_data);
    create_tree_view_in_scrolled_window(gui_data);
    create_tree_view_in_rules_window(gui_data);
    set_example_name(gui_data);
}

#[derive(Copy, Clone)]
pub enum OpenMode {
    OnlyPath,
    PathAndName,
}

fn setup_languages(gui_data: &GuiData) {
    let combo_box_settings_language = gui_data.settings.combo_box_settings_language.clone();
    for lang in LANGUAGES_ALL {
        combo_box_settings_language.append_text(lang.combo_box_text);
    }
    combo_box_settings_language.set_active(Some(0));
}

fn create_tree_view_in_scrolled_window(gui_data: &GuiData) {
    let scrolled_window_results: ScrolledWindow = gui_data.results.scrolled_window_results.clone();

    let col_types: [Type; 8] = [Type::STRING, Type::U8, Type::STRING, Type::STRING, Type::STRING, Type::U64, Type::U64, Type::U64];

    let list_store: ListStore = ListStore::new(&col_types);

    let tree_view: TreeView = gui_data.results.tree_view_results.clone();
    let gc_tree_view: GestureClick = gui_data.results.gc_tree_view_results.clone();
    tree_view.set_model(Some(&list_store));

    tree_view.selection().set_mode(SelectionMode::Multiple);

    gc_tree_view.set_button(0);
    gc_tree_view.connect_pressed(|gesture_click: &GestureClick, number_of_clicks: i32, _b: f64, _c: f64| {
        let tree_view = gesture_click
            .widget()
            .expect("Cannot find child widget")
            .downcast::<TreeView>()
            .expect("Cannot downcast to TreeView");

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

fn create_tree_view_in_rules_window(gui_data: &GuiData) {
    let scrolled_window_rules: ScrolledWindow = gui_data.rules_bottom_panel.scrolled_window_rules.clone();

    let col_types: [Type; 3] = [Type::STRING, Type::STRING, Type::STRING];

    let list_store: ListStore = ListStore::new(&col_types);

    let tree_view: TreeView = gui_data.rules_bottom_panel.tree_view_window_rules.clone();
    tree_view.set_model(Some(&list_store));

    // tree_view.selection().set_mode(SelectionMode::Multiple);

    create_tree_view_rules(&tree_view);

    scrolled_window_rules.set_child(Some(&tree_view));
    scrolled_window_rules.show();
}

fn set_example_name(gui_data: &GuiData) {
    let entry_example_before = gui_data.window_rules.entry_example_before.clone();
    entry_example_before.set_text(EXAMPLE_NAME);

    update_examples(&gui_data.window_rules, None);
}

fn common_open_function(tree_view: &TreeView, column_name: i32, column_path: i32, opening_mode: &OpenMode) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_full_name_from_path_name() {
        let path = "path";
        let name = "name";
        let result = get_full_name_from_path_name(path, name);
        assert_eq!(result, "path/name");
        assert_eq!(result.len(), 9);
        assert_eq!(result.capacity(), 9);
    }
}
