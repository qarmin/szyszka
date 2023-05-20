use gtk4::prelude::*;
use gtk4::TreeView;
use std::cell::RefCell;
use std::rc::Rc;

use crate::fls;
use crate::rule::rules::MultipleRules;

#[derive(Clone)]
pub struct GuiRulesBottomPanel {
    pub button_add_rule: gtk4::Button,
    pub button_edit_rule: gtk4::Button,
    pub button_remove_rule: gtk4::Button,
    pub button_rule_one_up: gtk4::Button,
    pub button_rule_one_down: gtk4::Button,
    pub button_save_rules: gtk4::Button,
    pub menu_button_load_rules: gtk4::MenuButton,
    pub scrolled_window_rules: gtk4::ScrolledWindow,
    pub tree_view_window_rules: TreeView,
    pub label_rules: gtk4::Label,

    pub imported_rules: Rc<RefCell<Vec<MultipleRules>>>,
}

impl GuiRulesBottomPanel {
    pub fn create_from_builder(builder: &gtk4::Builder) -> Self {
        let button_add_rule: gtk4::Button = builder.object("button_add_rule").unwrap();
        let button_edit_rule: gtk4::Button = builder.object("button_edit_rule").unwrap();
        let button_remove_rule: gtk4::Button = builder.object("button_remove_rule").unwrap();
        let button_rule_one_up: gtk4::Button = builder.object("button_rule_one_up").unwrap();
        let button_rule_one_down: gtk4::Button = builder.object("button_rule_one_down").unwrap();
        let scrolled_window_rules: gtk4::ScrolledWindow = builder.object("scrolled_window_rules").unwrap();
        let button_save_rules: gtk4::Button = builder.object("button_save_rules").unwrap();
        let menu_button_load_rules: gtk4::MenuButton = builder.object("menu_button_load_rules").unwrap();
        let label_rules: gtk4::Label = builder.object("label_rules").unwrap();

        let tree_view_window_rules: TreeView = TreeView::new();

        let imported_rules = Rc::new(RefCell::new(Vec::new()));

        Self {
            button_add_rule,
            button_edit_rule,
            button_remove_rule,
            button_rule_one_up,
            button_rule_one_down,
            button_save_rules,
            menu_button_load_rules,
            scrolled_window_rules,
            tree_view_window_rules,
            label_rules,
            imported_rules,
        }
    }
    pub fn update_language(&self) {
        self.button_add_rule.set_label(&fls!("bottom_rule_add_button"));
        self.button_edit_rule.set_label(&fls!("bottom_rule_edit_button"));
        self.button_remove_rule.set_label(&fls!("bottom_rule_remove_button"));
        self.button_rule_one_up.set_label(&fls!("bottom_rule_one_up_button"));
        self.button_rule_one_down.set_label(&fls!("bottom_rule_one_down_button"));
        self.button_save_rules.set_label(&fls!("bottom_rule_save_rules_button"));
        self.menu_button_load_rules.set_label(&fls!("bottom_rule_load_rules_button"));
        self.label_rules.set_label(&fls!("bottom_rule_label_rules"));

        let columns = self.tree_view_window_rules.columns();
        columns[0].set_title(&fls!("tree_view_bottom_tool_type"));
        columns[1].set_title(&fls!("tree_view_bottom_usage_name"));
        columns[2].set_title(&fls!("tree_view_bottom_description"));
    }
}
