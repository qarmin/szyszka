use crate::class_dialog_rule_remove::GUIRemove;
use crate::class_dialog_rule_size_letters::GUISizeLetters;
use gtk::prelude::*;

#[derive(Clone)]
pub struct GUIDialogRules {
    pub notebook_choose_rule: gtk::Notebook,

    pub dialog_with_rules: gtk::Dialog,
    pub button_dialog_close: gtk::Button,
    pub button_dialog_ok: gtk::Button,

    pub size_letters: GUISizeLetters,
    pub remove: GUIRemove,
}

impl GUIDialogRules {
    pub fn create_from_builder(builder: &gtk::Builder) -> Self {
        let notebook_choose_rule: gtk::Notebook = builder.get_object("notebook_choose_rule").unwrap();

        let dialog_with_rules: gtk::Dialog = builder.get_object("dialog_rules").unwrap();
        let button_dialog_close: gtk::Button = builder.get_object("button_dialog_close").unwrap();
        let button_dialog_ok: gtk::Button = builder.get_object("button_dialog_ok").unwrap();

        let size_letters: GUISizeLetters = GUISizeLetters::create_from_builder(&builder);
        let remove: GUIRemove = GUIRemove::create_from_builder(&builder);

        Self {
            notebook_choose_rule,
            dialog_with_rules,
            button_dialog_close,
            button_dialog_ok,
            size_letters,
            remove,
        }
    }
}
