use crate::class_dialog_rule_add_text::GUIAddText;
use crate::class_dialog_rule_purge::GUIPurge;
use crate::class_dialog_rule_size_letters::GUISizeLetters;
use crate::class_dialog_rule_trim::GUITrim;
use gtk::prelude::*;
use gtk::{Button, Entry, Label};

#[derive(Clone)]
pub struct GUIDialogRules {
    pub notebook_choose_rule: gtk::Notebook,

    pub window_with_rules: gtk::Window,
    pub button_rule_window_add: gtk::Button,

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

        let window_with_rules: gtk::Window = builder.get_object("window_with_rules").unwrap();
        let button_rule_window_add: gtk::Button = builder.get_object("button_rule_window_add").unwrap();

        let size_letters: GUISizeLetters = GUISizeLetters::create_from_builder(&builder);
        let purge: GUIPurge = GUIPurge::create_from_builder(&builder);
        let add_text: GUIAddText = GUIAddText::create_from_builder(&builder);
        let trim: GUITrim = GUITrim::create_from_builder(&builder);

        let entry_example_before: gtk::Entry = builder.get_object("entry_example_before").unwrap();
        let label_example_after: gtk::Label = builder.get_object("label_example_after").unwrap();
        let button_example_reset: gtk::Button = builder.get_object("button_example_reset").unwrap();

        Self {
            notebook_choose_rule,
            window_with_rules,
            button_rule_window_add,
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
