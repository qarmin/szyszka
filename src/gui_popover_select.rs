use gtk::prelude::*;

#[derive(Clone)]
pub struct GuiPopoverSelect {
    pub popover_select: gtk::Popover,
    pub button_select_all: gtk::Button,
    pub button_select_reverse: gtk::Button,
    pub button_select_custom: gtk::Button,
    pub button_unselect_custom: gtk::Button,
    pub button_select_changed: gtk::Button,
    pub button_unselect_changed: gtk::Button,
}

impl GuiPopoverSelect {
    pub fn create_from_builder(builder: &gtk::Builder) -> Self {
        let popover_select: gtk::Popover = builder.object("popover_select").unwrap();
        let button_select_all: gtk::Button = builder.object("button_select_all").unwrap();
        let button_select_reverse: gtk::Button = builder.object("button_select_reverse").unwrap();
        let button_select_custom: gtk::Button = builder.object("button_select_custom").unwrap();
        let button_unselect_custom: gtk::Button = builder.object("button_unselect_custom").unwrap();
        let button_select_changed: gtk::Button = builder.object("button_select_changed").unwrap();
        let button_unselect_changed: gtk::Button = builder.object("button_unselect_changed").unwrap();
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
