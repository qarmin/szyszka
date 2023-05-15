use gtk4::{Button, Entry, MenuButton};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct GuiCustom {
    pub imported_custom_rules: Rc<RefCell<Vec<String>>>,
    pub entry_custom_text_to_change: Entry,
    pub menu_button_load_custom_rule: MenuButton,
    pub button_save_custom_rule: Button,
}

impl GuiCustom {
    pub fn create_from_builder(builder: &gtk4::Builder) -> Self {
        let imported_custom_rules = Rc::new(RefCell::new(Vec::new()));
        let entry_custom_text_to_change: Entry = builder.object("entry_custom_text_to_change").unwrap();
        let menu_button_load_custom_rule: MenuButton = builder.object("menu_button_load_custom_rule").unwrap();
        let button_save_custom_rule: Button = builder.object("button_save_custom_rule").unwrap();

        Self {
            imported_custom_rules,
            entry_custom_text_to_change,
            menu_button_load_custom_rule,
            button_save_custom_rule,
        }
    }
}
