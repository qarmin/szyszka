use gtk::prelude::*;

#[derive(Clone)]
pub struct GuiPurge {
    pub check_button_purge_name: gtk::CheckButton,
    pub check_button_purge_extension: gtk::CheckButton,
    pub check_button_purge_both: gtk::CheckButton,
}

impl GuiPurge {
    pub fn create_from_builder(builder: &gtk::Builder) -> Self {
        let check_button_purge_name: gtk::CheckButton = builder.object("check_button_purge_name").unwrap();
        let check_button_purge_extension: gtk::CheckButton = builder.object("check_button_purge_extension").unwrap();
        let check_button_purge_both: gtk::CheckButton = builder.object("check_button_purge_both").unwrap();

        Self {
            check_button_purge_name,
            check_button_purge_extension,
            check_button_purge_both,
        }
    }
}
