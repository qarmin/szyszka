use crate::fls;
use gtk4::prelude::*;
use gtk4::{Button, Entry, Label, Widget};

use crate::gui_data_things::class_dialog_rule_add_number::GuiAddNumber;
use crate::gui_data_things::class_dialog_rule_add_text::GuiAddText;
use crate::gui_data_things::class_dialog_rule_custom::GuiCustom;
use crate::gui_data_things::class_dialog_rule_normalize::GuiNormalize;
use crate::gui_data_things::class_dialog_rule_purge::GuiPurge;
use crate::gui_data_things::class_dialog_rule_replace::GuiReplace;
use crate::gui_data_things::class_dialog_rule_size_letters::GuiSizeLetters;
use crate::gui_data_things::class_dialog_rule_trim::GuiTrim;
use crate::help_function::{get_all_direct_children, set_icon_of_button, NotebookRules};

pub const SZY_ICON_RESET: &[u8] = include_bytes!("../../icons/szy_reset.svg");

#[derive(Clone)]
pub struct GuiDialogRules {
    pub notebook_choose_rule: gtk4::Notebook,

    pub window_with_rules: gtk4::Window,
    pub button_rule_window_add: Button,

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

    pub label_example: Label,
    pub label_example_text_before: Label,
    pub label_example_text_after: Label,
}

impl GuiDialogRules {
    pub fn create_from_builder(builder: &gtk4::Builder) -> Self {
        let notebook_choose_rule: gtk4::Notebook = builder.object("notebook_choose_rule").unwrap();

        let window_with_rules: gtk4::Window = builder.object("window_with_rules").unwrap();
        let button_rule_window_add: Button = builder.object("button_rule_window_add").unwrap();

        let size_letters: GuiSizeLetters = GuiSizeLetters::create_from_builder(builder);
        let purge: GuiPurge = GuiPurge::create_from_builder(builder);
        let add_text: GuiAddText = GuiAddText::create_from_builder(builder);
        let trim: GuiTrim = GuiTrim::create_from_builder(builder);
        let custom: GuiCustom = GuiCustom::create_from_builder(builder);
        let replace: GuiReplace = GuiReplace::create_from_builder(builder);
        let add_number: GuiAddNumber = GuiAddNumber::create_from_builder(builder);
        let normalize: GuiNormalize = GuiNormalize::create_from_builder(builder);

        let entry_example_before: Entry = builder.object("entry_example_before").unwrap();
        let button_example_reset: Button = builder.object("button_example_reset").unwrap();

        let label_example: Label = builder.object("label_example").unwrap();
        let label_example_text_before: Label = builder.object("label_example_text_before").unwrap();
        let label_example_text_after: Label = builder.object("label_example_text_after").unwrap();
        let label_example_after: Label = builder.object("label_example_after").unwrap();

        set_icon_of_button(&button_example_reset, SZY_ICON_RESET);

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
            label_example,
            label_example_text_before,
            label_example_text_after,
        }
    }
    pub fn update_language(&self) {
        self.button_rule_window_add.set_label(&fls!("button_rule_window_add"));
        self.label_example.set_label(&fls!("label_example"));
        self.label_example_text_before.set_label(&fls!("label_example_text_before"));
        self.label_example_text_after.set_label(&fls!("label_example_text_after"));

        let vec_children: Vec<Widget> = get_all_direct_children(&self.notebook_choose_rule);
        let vec_children: Vec<Widget> = get_all_direct_children(&vec_children[1]);

        for (main_enum, fl_thing) in [
            (NotebookRules::Custom as usize, fls!("notebook_tab_custom")),
            (NotebookRules::UpperLowerCases as usize, fls!("notebook_tab_case_size")),
            (NotebookRules::Purge as usize, fls!("notebook_tab_purge")),
            (NotebookRules::AddNumber as usize, fls!("notebook_tab_add_number")),
            (NotebookRules::AddText as usize, fls!("notebook_tab_add_text")),
            (NotebookRules::Replace as usize, fls!("notebook_tab_replace")),
            (NotebookRules::Trim as usize, fls!("notebook_tab_trim")),
            (NotebookRules::Normalize as usize, fls!("notebook_tab_normalize")),
        ] {
            self.notebook_choose_rule
                .tab_label(&vec_children[main_enum])
                .unwrap()
                .downcast::<Label>()
                .unwrap()
                .set_text(&fl_thing);
        }
    }
}
