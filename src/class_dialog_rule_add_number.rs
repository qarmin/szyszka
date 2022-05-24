use gtk::prelude::*;

#[derive(Clone)]
pub struct GuiAddNumber {
    pub check_button_add_number_before_name: gtk::CheckButton,
    pub check_button_add_number_after_name: gtk::CheckButton,

    pub entry_add_number_start_number: gtk::Entry,
    pub entry_add_number_step: gtk::Entry,
    pub entry_add_number_zeros: gtk::Entry,
}

impl GuiAddNumber {
    pub fn create_from_builder(builder: &gtk::Builder) -> Self {
        let check_button_add_number_before_name: gtk::CheckButton = builder.object("check_button_add_number_before_name").unwrap();
        let check_button_add_number_after_name: gtk::CheckButton = builder.object("check_button_add_number_after_name").unwrap();

        let entry_add_number_start_number: gtk::Entry = builder.object("entry_add_number_start_number").unwrap();
        let entry_add_number_step: gtk::Entry = builder.object("entry_add_number_step").unwrap();
        let entry_add_number_zeros: gtk::Entry = builder.object("entry_add_number_zeros").unwrap();

        Self {
            check_button_add_number_before_name,
            check_button_add_number_after_name,
            entry_add_number_start_number,
            entry_add_number_step,
            entry_add_number_zeros,
        }
    }
}
