use crate::class_dialog_rule_add_text::GUIAddText;
use crate::class_dialog_rule_purge::GUIPurge;
use crate::class_dialog_rule_size_letters::GUISizeLetters;
use crate::class_dialog_rule_trim::GUITrim;
use gtk::prelude::*;
use gtk::{Button, Entry, Label};

#[derive(Clone)]
pub struct GUIDialogRules {
    pub notebook_choose_rule: gtk::Notebook,

    pub dialog_with_rules: gtk::Dialog,
    pub button_dialog_close: gtk::Button,
    pub button_dialog_ok: gtk::Button,

    pub size_letters: GUISizeLetters,
    pub purge: GUIPurge,
    pub add_text: GUIAddText,
    pub trim: GUITrim,

    pub entry_example_before: Entry,
    pub label_example_after: Label,
    pub button_example_reset: Button,
}

impl GUIDialogRules {
    pub fn create_from_builder(builder: &gtk::Builder) -> Self {
        let notebook_choose_rule: gtk::Notebook = builder.get_object("notebook_choose_rule").unwrap();

        let dialog_with_rules: gtk::Dialog = builder.get_object("dialog_rules").unwrap();
        let button_dialog_close: gtk::Button = builder.get_object("button_dialog_close").unwrap();
        let button_dialog_ok: gtk::Button = builder.get_object("button_dialog_ok").unwrap();

        let size_letters: GUISizeLetters = GUISizeLetters::create_from_builder(&builder);
        let purge: GUIPurge = GUIPurge::create_from_builder(&builder);
        let add_text: GUIAddText = GUIAddText::create_from_builder(&builder);
        let trim: GUITrim = GUITrim::create_from_builder(&builder);

        let entry_example_before: gtk::Entry = builder.get_object("entry_example_before").unwrap();
        let label_example_after: gtk::Label = builder.get_object("label_example_after").unwrap();
        let button_example_reset: gtk::Button = builder.get_object("button_example_reset").unwrap();

        Self {
            notebook_choose_rule,
            dialog_with_rules,
            button_dialog_close,
            button_dialog_ok,
            size_letters,
            purge,
            add_text,
            trim,
            entry_example_before,
            label_example_after,
            button_example_reset,
        }
    }
}
