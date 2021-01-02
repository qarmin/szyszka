use gtk::prelude::*;

#[derive(Clone)]
pub struct Results {
    pub scrolled_window_results: gtk::ScrolledWindow,
}

impl Results {
    pub fn create_from_builder(builder: &gtk::Builder) -> Self {
        let scrolled_window_results: gtk::ScrolledWindow = builder.get_object("scrolled_window_results").unwrap();

        Self { scrolled_window_results }
    }
}
