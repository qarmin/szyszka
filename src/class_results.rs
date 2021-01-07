use gtk::prelude::*;
use gtk::TreeView;

#[derive(Clone)]
pub struct GUIResults {
    pub scrolled_window_results: gtk::ScrolledWindow,
    pub tree_view_results: gtk::TreeView,
}

impl GUIResults {
    pub fn create_from_builder(builder: &gtk::Builder) -> Self {
        let scrolled_window_results: gtk::ScrolledWindow = builder.get_object("scrolled_window_results").unwrap();
        let tree_view_results = TreeView::new();

        Self { scrolled_window_results, tree_view_results }
    }
}
