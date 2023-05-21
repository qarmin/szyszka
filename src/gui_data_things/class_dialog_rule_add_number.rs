use crate::fls;
use gtk4::prelude::CheckButtonExt;

#[derive(Clone)]
pub struct GuiAddNumber {
    pub check_button_add_number_before_name: gtk4::CheckButton,
    pub check_button_add_number_after_name: gtk4::CheckButton,

    pub entry_add_number_start_number: gtk4::Entry,
    pub entry_add_number_step: gtk4::Entry,
    pub entry_add_number_zeros: gtk4::Entry,

    pub label_add_number_place: gtk4::Label,
    pub label_add_number_settings: gtk4::Label,
    pub label_number_start_number: gtk4::Label,
    pub label_number_step: gtk4::Label,
    pub label_number_fill_zeros: gtk4::Label,
}

impl GuiAddNumber {
    pub fn create_from_builder(builder: &gtk4::Builder) -> Self {
        let check_button_add_number_before_name: gtk4::CheckButton = builder.object("check_button_add_number_before_name").unwrap();
        let check_button_add_number_after_name: gtk4::CheckButton = builder.object("check_button_add_number_after_name").unwrap();

        check_button_add_number_after_name.set_group(Some(&check_button_add_number_before_name));

        let entry_add_number_start_number: gtk4::Entry = builder.object("entry_add_number_start_number").unwrap();
        let entry_add_number_step: gtk4::Entry = builder.object("entry_add_number_step").unwrap();
        let entry_add_number_zeros: gtk4::Entry = builder.object("entry_add_number_zeros").unwrap();

        let label_add_number_place: gtk4::Label = builder.object("label_add_number_place").unwrap();
        let label_add_number_settings: gtk4::Label = builder.object("label_add_number_settings").unwrap();
        let label_number_start_number: gtk4::Label = builder.object("label_number_start_number").unwrap();
        let label_number_step: gtk4::Label = builder.object("label_number_step").unwrap();
        let label_number_fill_zeros: gtk4::Label = builder.object("label_number_fill_zeros").unwrap();

        Self {
            check_button_add_number_before_name,
            check_button_add_number_after_name,
            entry_add_number_start_number,
            entry_add_number_step,
            entry_add_number_zeros,
            label_add_number_place,
            label_add_number_settings,
            label_number_start_number,
            label_number_step,
            label_number_fill_zeros,
        }
    }
    pub fn update_language(&self) {
        self.label_number_fill_zeros.set_label(&fls!("label_number_fill_zeros"));
        self.label_number_step.set_label(&fls!("label_number_step"));
        self.label_number_start_number.set_label(&fls!("label_number_start_number"));
        self.label_add_number_settings.set_label(&fls!("label_add_number_settings"));
        self.label_add_number_place.set_label(&fls!("label_add_number_place"));
        self.check_button_add_number_after_name.set_label(Some(&fls!("check_button_add_number_after_name")));
        self.check_button_add_number_before_name.set_label(Some(&fls!("check_button_add_number_before_name")));
    }
}
