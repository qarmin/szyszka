#[derive(Clone)]
pub struct GuiCustom {
    pub entry_custom_text_to_change: gtk4::Entry,
}

impl GuiCustom {
    pub fn create_from_builder(builder: &gtk4::Builder) -> Self {
        let entry_custom_text_to_change: gtk4::Entry = builder.object("entry_custom_text_to_change").unwrap();

        Self { entry_custom_text_to_change }
    }
}
