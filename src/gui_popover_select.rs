#[derive(Clone)]
pub struct GuiPopoverSelect {
    pub popover_select: gtk4::Popover,
    pub button_select_all: gtk4::Button,
    pub button_select_reverse: gtk4::Button,
    pub button_select_custom: gtk4::Button,
    pub button_unselect_custom: gtk4::Button,
    pub button_select_changed: gtk4::Button,
    pub button_unselect_changed: gtk4::Button,
}

impl GuiPopoverSelect {
    pub fn create_from_builder(builder: &gtk4::Builder) -> Self {
        let popover_select: gtk4::Popover = builder.object("popover_select").unwrap();
        let button_select_all: gtk4::Button = builder.object("button_select_all").unwrap();
        let button_select_reverse: gtk4::Button = builder.object("button_select_reverse").unwrap();
        let button_select_custom: gtk4::Button = builder.object("button_select_custom").unwrap();
        let button_unselect_custom: gtk4::Button = builder.object("button_unselect_custom").unwrap();
        let button_select_changed: gtk4::Button = builder.object("button_select_changed").unwrap();
        let button_unselect_changed: gtk4::Button = builder.object("button_unselect_changed").unwrap();
        Self {
            popover_select,
            button_select_all,
            button_select_reverse,
            button_select_custom,
            button_unselect_custom,
            button_select_changed,
            button_unselect_changed,
        }
    }
}
