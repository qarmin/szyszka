use gtk4::prelude::*;

#[derive(Clone)]
pub struct GuiPurge {
    pub check_button_purge_name: gtk4::CheckButton,
    pub check_button_purge_extension: gtk4::CheckButton,
    pub check_button_purge_both: gtk4::CheckButton,
    pub label_purge_tool_type: gtk4::Label,
}

impl GuiPurge {
    pub fn create_from_builder(builder: &gtk4::Builder) -> Self {
        let check_button_purge_name: gtk4::CheckButton = builder.object("check_button_purge_name").unwrap();
        let check_button_purge_extension: gtk4::CheckButton = builder.object("check_button_purge_extension").unwrap();
        let check_button_purge_both: gtk4::CheckButton = builder.object("check_button_purge_both").unwrap();

        let label_purge_tool_type: gtk4::Label = builder.object("label_purge_tool_type").unwrap();

        check_button_purge_both.set_group(Some(&check_button_purge_name));
        check_button_purge_extension.set_group(Some(&check_button_purge_name));

        Self {
            check_button_purge_name,
            check_button_purge_extension,
            check_button_purge_both,
            label_purge_tool_type,
        }
    }
    pub fn update_language(&self) {
        self.label_purge_tool_type.set_label(&crate::fls!("label_purge_tool_type"));
        self.check_button_purge_name.set_label(Some(&crate::fls!("check_button_purge_name")));
        self.check_button_purge_extension.set_label(Some(&crate::fls!("check_button_purge_extension")));
        self.check_button_purge_both.set_label(Some(&crate::fls!("check_button_purge_both")));
    }
}
