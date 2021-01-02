use crate::class_results::Results;
use crate::class_rules::Rules;
use crate::class_status::Status;
use crate::class_upper_buttons::*;
use gtk::prelude::*;
use gtk::Builder;

#[derive(Clone)]
pub struct GuiData {
    // Glade builder
    // pub glade_src: String,
    // pub builder: Builder,

    // Subcategories
    pub upper_buttons: UpperButtons,
    pub results: Results,
    pub rules: Rules,
    pub status: Status,

    // Window
    pub window_main: gtk::Window,
}

impl GuiData {
    pub fn new() -> Self {
        //// Loading glade file content and build with it help UI
        let glade_src = include_str!("../szyszka.glade").to_string();
        let builder = Builder::from_string(glade_src.as_str());

        let upper_buttons: UpperButtons = UpperButtons::create_from_builder(&builder);
        let results: Results = Results::create_from_builder(&builder);
        let rules: Rules = Rules::create_from_builder(&builder);
        let status: Status = Status::create_from_builder(&builder);

        //// Windows
        let window_main: gtk::Window = builder.get_object("window_main").unwrap();
        window_main.show_all();
        window_main.set_title("Szyszka");

        Self {
            // glade_src,
            // builder,
            upper_buttons,
            results,
            rules,
            status,
            window_main,
        }
    }
}
