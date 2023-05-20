use gtk4::prelude::*;
use gtk4::{Button, Entry, Label, MenuButton};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct GuiCustom {
    pub imported_custom_rules: Rc<RefCell<Vec<String>>>,
    pub entry_custom_text_to_change: Entry,
    pub menu_button_load_custom_rule: MenuButton,
    pub button_save_custom_rule: Button,
    pub label_custom_instruction: Label,
}

impl GuiCustom {
    pub fn create_from_builder(builder: &gtk4::Builder) -> Self {
        let imported_custom_rules = Rc::new(RefCell::new(Vec::new()));
        let entry_custom_text_to_change: Entry = builder.object("entry_custom_text_to_change").unwrap();
        let menu_button_load_custom_rule: MenuButton = builder.object("menu_button_load_custom_rule").unwrap();
        let button_save_custom_rule: Button = builder.object("button_save_custom_rule").unwrap();
        let label_custom_instruction: Label = builder.object("label_custom_instruction").unwrap();

        Self {
            imported_custom_rules,
            entry_custom_text_to_change,
            menu_button_load_custom_rule,
            button_save_custom_rule,
            label_custom_instruction,
        }
    }
    pub fn update_language(&self) {
        self.label_custom_instruction.set_label(&crate::fls!("label_custom_instruction"));
        self.menu_button_load_custom_rule.set_label(&crate::fls!("menu_button_load_custom_rule"));
        self.button_save_custom_rule.set_label(&crate::fls!("button_save_custom_rule"));
    }
}
