use gtk::prelude::*;
use gtk::TreeView;

#[derive(Clone)]
pub struct GuiResults {
    pub scrolled_window_results: gtk::ScrolledWindow,
    pub tree_view_results: gtk::TreeView,
}

impl GuiResults {
    pub fn create_from_builder(builder: &gtk::Builder) -> Self {
        let scrolled_window_results: gtk::ScrolledWindow = builder.object("scrolled_window_results").unwrap();
        let tree_view_results = TreeView::new();

        Self { scrolled_window_results, tree_view_results }
    }
    pub fn update_language(&self) {}
}
