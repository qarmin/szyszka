use gtk4::prelude::*;
use gtk4::{CellRendererText, TreeView, TreeViewColumn};

use crate::help_function::{ColumnsResults, ColumnsRules};

pub fn create_tree_view_results(tree_view: &TreeView) {
    // TODO maybe enable sorting, but for now GTK crashes when using it
    create_default_column(tree_view, ColumnsResults::TypeString as i32, Some(None), "Type");
    create_default_column(tree_view, ColumnsResults::CurrentName as i32, Some(None), "Current Name");
    create_default_column(tree_view, ColumnsResults::FutureName as i32, Some(None), "Future Name");
    create_default_column(tree_view, ColumnsResults::Path as i32, Some(None), "Path");

    tree_view.set_vexpand(true);
}

pub fn create_tree_view_rules(tree_view: &TreeView) {
    create_default_column(tree_view, ColumnsRules::RuleType as i32, None, "Tool Type");
    create_default_column(tree_view, ColumnsRules::UsageType as i32, None, "Usage Name");
    create_default_column(tree_view, ColumnsRules::Description as i32, None, "Description");

    tree_view.set_vexpand(true);
}

#[allow(clippy::option_option)]
fn create_default_column(tree_view: &TreeView, column_id: i32, _sort_column_id: Option<Option<i32>>, name: &str) -> (CellRendererText, TreeViewColumn) {
    let renderer = CellRendererText::new();
    let column: TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title(name);
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", column_id);

    // TODO sorting is disabled, because broke list_store swaps, which is needed to move items in list
    // Consider to add manual sorting feature

    // if let Some(sort_column_id) = sort_column_id {
    //     if let Some(sort_column_id) = sort_column_id {
    //         column.set_sort_column_id(sort_column_id);
    //     } else {
    //         column.set_sort_column_id(column_id);
    //     }
    // }
    tree_view.append_column(&column);
    (renderer, column)
}
