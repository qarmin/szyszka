#[derive(Clone)]
pub struct GuiAddText {
    pub check_button_add_text_before_name: gtk4::CheckButton,
    pub check_button_add_text_after_name: gtk4::CheckButton,

    pub entry_add_text_text_to_add: gtk4::Entry,
}

impl GuiAddText {
    pub fn create_from_builder(builder: &gtk4::Builder) -> Self {
        let check_button_add_text_before_name: gtk4::CheckButton = builder.object("check_button_add_text_before_name").unwrap();
        let check_button_add_text_after_name: gtk4::CheckButton = builder.object("check_button_add_text_after_name").unwrap();

        let entry_add_text_text_to_add: gtk4::Entry = builder.object("entry_add_text_text_to_add").unwrap();

        Self {
            check_button_add_text_before_name,
            check_button_add_text_after_name,
            entry_add_text_text_to_add,
        }
    }
}
