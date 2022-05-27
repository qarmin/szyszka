use crate::fls;
use gtk4::prelude::*;
use gtk4::TreeView;

#[derive(Clone)]
pub struct GuiRulesBottomPanel {
    pub button_add_rule: gtk4::Button,
    pub button_edit_rule: gtk4::Button,
    pub button_remove_rule: gtk4::Button,
    pub button_rule_one_up: gtk4::Button,
    pub button_rule_one_down: gtk4::Button,
    pub scrolled_window_rules: gtk4::ScrolledWindow,
    pub tree_view_window_rules: gtk4::TreeView,
}

impl GuiRulesBottomPanel {
    pub fn create_from_builder(builder: &gtk4::Builder) -> Self {
        let button_add_rule: gtk4::Button = builder.object("button_add_rule").unwrap();
        let button_edit_rule: gtk4::Button = builder.object("button_edit_rule").unwrap();
        let button_remove_rule: gtk4::Button = builder.object("button_remove_rule").unwrap();
        let button_rule_one_up: gtk4::Button = builder.object("button_rule_one_up").unwrap();
        let button_rule_one_down: gtk4::Button = builder.object("button_rule_one_down").unwrap();
        let scrolled_window_rules: gtk4::ScrolledWindow = builder.object("scrolled_window_rules").unwrap();

        let tree_view_window_rules: gtk4::TreeView = TreeView::new();

        Self {
            button_add_rule,
            button_edit_rule,
            button_remove_rule,
            button_rule_one_up,
            button_rule_one_down,
            scrolled_window_rules,
            tree_view_window_rules,
        }
    }
    pub fn update_language(&self) {
        self.button_add_rule.set_label(&fls!("bottom_rule_add_button"));
        self.button_edit_rule.set_label(&fls!("bottom_rule_edit_button"));
        self.button_remove_rule.set_label(&fls!("bottom_rule_remove_button"));
        self.button_rule_one_up.set_label(&fls!("bottom_rule_one_up_button"));
        self.button_rule_one_down.set_label(&fls!("bottom_rule_one_down_button"));
    }
}
