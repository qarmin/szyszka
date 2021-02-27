use crate::help_function::{ColumnsResults, ColumnsRules};
use gtk::*;

pub fn create_tree_view_results(tree_view: &gtk::TreeView) {
    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Current Name");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsResults::CurrentName as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Future Name");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsResults::FutureName as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Path");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsResults::Path as i32);
    tree_view.append_column(&column);

    tree_view.set_vexpand(true);
}
pub fn create_tree_view_rules(tree_view: &gtk::TreeView) {
    // let renderer = gtk::CellRendererText::new();
    // let column: gtk::TreeViewColumn = TreeViewColumn::new();
    // column.pack_start(&renderer, true);
    // column.set_title("Number");
    // column.set_resizable(true);
    // column.set_min_width(50);
    // column.add_attribute(&renderer, "text", ColumnsRules::RuleNumber as i32);
    // tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Tool Type");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsRules::RuleType as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Usage Name");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsRules::UsageType as i32);
    tree_view.append_column(&column);

    tree_view.set_vexpand(true);
}
