use gtk4::prelude::CheckButtonExt;

#[derive(Clone)]
pub struct GuiAddNumber {
    pub check_button_add_number_before_name: gtk4::CheckButton,
    pub check_button_add_number_after_name: gtk4::CheckButton,

    pub entry_add_number_start_number: gtk4::Entry,
    pub entry_add_number_step: gtk4::Entry,
    pub entry_add_number_zeros: gtk4::Entry,
}

impl GuiAddNumber {
    pub fn create_from_builder(builder: &gtk4::Builder) -> Self {
        let check_button_add_number_before_name: gtk4::CheckButton = builder.object("check_button_add_number_before_name").unwrap();
        let check_button_add_number_after_name: gtk4::CheckButton = builder.object("check_button_add_number_after_name").unwrap();

        check_button_add_number_after_name.set_group(Some(&check_button_add_number_before_name));

        let entry_add_number_start_number: gtk4::Entry = builder.object("entry_add_number_start_number").unwrap();
        let entry_add_number_step: gtk4::Entry = builder.object("entry_add_number_step").unwrap();
        let entry_add_number_zeros: gtk4::Entry = builder.object("entry_add_number_zeros").unwrap();

        Self {
            check_button_add_number_before_name,
            check_button_add_number_after_name,
            entry_add_number_start_number,
            entry_add_number_step,
            entry_add_number_zeros,
        }
    }
}
