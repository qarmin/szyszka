use crate::class_dialog_rule_add_number::GuiAddNumber;
use crate::class_dialog_rule_add_text::GuiAddText;
use crate::class_dialog_rule_custom::GuiCustom;
use crate::class_dialog_rule_normalize::GuiNormalize;
use crate::class_dialog_rule_purge::GuiPurge;
use crate::class_dialog_rule_replace::GuiReplace;
use crate::class_dialog_rule_size_letters::GuiSizeLetters;
use crate::class_dialog_rule_trim::GuiTrim;
use gtk::prelude::*;
use gtk::{Button, Entry, Label, WindowPosition};

#[derive(Clone)]
pub struct GuiDialogRules {
    pub notebook_choose_rule: gtk::Notebook,

    pub window_with_rules: gtk::Window,
    pub button_rule_window_add: gtk::Button,

    pub size_letters: GuiSizeLetters,
    pub purge: GuiPurge,
    pub add_text: GuiAddText,
    pub trim: GuiTrim,
    pub custom: GuiCustom,
    pub replace: GuiReplace,
    pub add_number: GuiAddNumber,
    pub normalize: GuiNormalize,

    pub entry_example_before: Entry,
    pub label_example_after: Label,
    pub button_example_reset: Button,
}

impl GuiDialogRules {
    pub fn create_from_builder(builder: &gtk::Builder) -> Self {
        let notebook_choose_rule: gtk::Notebook = builder.object("notebook_choose_rule").unwrap();

        let window_with_rules: gtk::Window = builder.object("window_with_rules").unwrap();
        window_with_rules.set_position(WindowPosition::Center);
        let button_rule_window_add: gtk::Button = builder.object("button_rule_window_add").unwrap();

        let size_letters: GuiSizeLetters = GuiSizeLetters::create_from_builder(&builder);
        let purge: GuiPurge = GuiPurge::create_from_builder(&builder);
        let add_text: GuiAddText = GuiAddText::create_from_builder(&builder);
        let trim: GuiTrim = GuiTrim::create_from_builder(&builder);
        let custom: GuiCustom = GuiCustom::create_from_builder(&builder);
        let replace: GuiReplace = GuiReplace::create_from_builder(&builder);
        let add_number: GuiAddNumber = GuiAddNumber::create_from_builder(&builder);
        let normalize: GuiNormalize = GuiNormalize::create_from_builder(&builder);

        let entry_example_before: gtk::Entry = builder.object("entry_example_before").unwrap();
        let label_example_after: gtk::Label = builder.object("label_example_after").unwrap();
        let button_example_reset: gtk::Button = builder.object("button_example_reset").unwrap();

        Self {
            notebook_choose_rule,
            window_with_rules,
            button_rule_window_add,
            size_letters,
            purge,
            add_text,
            trim,
            custom,
            replace,
            add_number,
            normalize,
            entry_example_before,
            label_example_after,
            button_example_reset,
        }
    }
}
