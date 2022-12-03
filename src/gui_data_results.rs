use gtk4::prelude::*;
use gtk4::GestureClick;
use gtk4::TreeView;

#[derive(Clone)]
pub struct GuiResults {
    pub scrolled_window_results: gtk4::ScrolledWindow,
    pub tree_view_results: TreeView,
    pub gc_tree_view_results: GestureClick,
}

impl GuiResults {
    pub fn create_from_builder(builder: &gtk4::Builder) -> Self {
        let scrolled_window_results: gtk4::ScrolledWindow = builder.object("scrolled_window_results").unwrap();
        let tree_view_results = TreeView::new();
        let gc_tree_view_results: GestureClick = GestureClick::new();
        tree_view_results.add_controller(&gc_tree_view_results);

        Self {
            scrolled_window_results,
            tree_view_results,
            gc_tree_view_results,
        }
    }
    pub fn update_language(&self) {}
}
