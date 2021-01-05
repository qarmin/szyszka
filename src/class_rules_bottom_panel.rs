use gtk::prelude::*;
use gtk::TreeView;

#[derive(Clone)]
pub struct RulesBottomPanel {
    pub button_add_rule: gtk::Button,
    pub button_remove_rule: gtk::Button,
    pub button_one_up: gtk::Button,
    pub button_one_down: gtk::Button,
    pub button_to_top: gtk::Button,
    pub button_to_bottom: gtk::Button,
    pub scrolled_window_rules: gtk::ScrolledWindow,
    pub tree_view_window_rules: gtk::TreeView,
}

impl RulesBottomPanel {
    pub fn create_from_builder(builder: &gtk::Builder) -> Self {
        let button_add_rule: gtk::Button = builder.get_object("button_add_rule").unwrap();
        let button_remove_rule: gtk::Button = builder.get_object("button_remove_rule").unwrap();
        let button_one_up: gtk::Button = builder.get_object("button_one_up").unwrap();
        let button_one_down: gtk::Button = builder.get_object("button_one_down").unwrap();
        let button_to_top: gtk::Button = builder.get_object("button_to_top").unwrap();
        let button_to_bottom: gtk::Button = builder.get_object("button_to_bottom").unwrap();
        let scrolled_window_rules: gtk::ScrolledWindow = builder.get_object("scrolled_window_rules").unwrap();

        let tree_view_window_rules: gtk::TreeView = TreeView::new();

        Self {
            button_add_rule,
            button_remove_rule,
            button_one_up,
            button_one_down,
            button_to_top,
            button_to_bottom,
            scrolled_window_rules,
            tree_view_window_rules,
        }
    }
}
