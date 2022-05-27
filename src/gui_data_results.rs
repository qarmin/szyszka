use gtk4::TreeView;

#[derive(Clone)]
pub struct GuiResults {
    pub scrolled_window_results: gtk4::ScrolledWindow,
    pub tree_view_results: gtk4::TreeView,
}

impl GuiResults {
    pub fn create_from_builder(builder: &gtk4::Builder) -> Self {
        let scrolled_window_results: gtk4::ScrolledWindow = builder.object("scrolled_window_results").unwrap();
        let tree_view_results = TreeView::new();

        Self { scrolled_window_results, tree_view_results }
    }
    pub fn update_language(&self) {}
}
