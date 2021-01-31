use crate::rule_add_text::rule_add_text;
use crate::rule_change_size_letters::rule_change_size_letters;
use crate::rule_custom::rule_custom;
use crate::rule_purge::rule_purge;
use crate::rule_trim::rule_trim;

pub struct Rules {
    pub rule_types: Vec<RuleType>,
    pub rule_place: Vec<RulePlace>,
    pub rules_number: usize,
    pub rule_data: Vec<RuleData>,
}

impl Rules {
    pub fn new() -> Self {
        Rules {
            rule_types: vec![],
            rule_place: vec![],
            rules_number: 0,
            rule_data: vec![],
        }
    }
    pub fn add_rule(&mut self, rule_type: RuleType, rule_place: RulePlace, rule_data: RuleData) {
        self.rule_types.push(rule_type);
        self.rule_place.push(rule_place);
        self.rule_data.push(rule_data);
        self.rules_number += 1;
    }
    pub fn apply_all_rules_to_item(&mut self, mut item: String) -> String {
        for rule_number in 0..self.rules_number {
            match self.rule_types[rule_number] {
                RuleType::CaseSize => {
                    item = rule_change_size_letters(item.as_str(), &self.rule_types[rule_number], &self.rule_place[rule_number], &self.rule_data[rule_number]);
                }
                RuleType::Purge => {
                    item = rule_purge(item.as_str(), &self.rule_types[rule_number], &self.rule_place[rule_number]);
                }
                RuleType::AddText => {
                    item = rule_add_text(item.as_str(), &self.rule_types[rule_number], &self.rule_place[rule_number], &self.rule_data[rule_number]);
                }
                RuleType::Trim => {
                    item = rule_trim(item.as_str(), &self.rule_types[rule_number], &self.rule_place[rule_number], &self.rule_data[rule_number]);
                }
                RuleType::Custom => {
                    item = rule_custom(item.as_str(), &self.rule_types[rule_number], &self.rule_place[rule_number], &self.rule_data[rule_number], rule_number as u64, false);
                }
            }
        }
        item
    }
}

pub enum RuleType {
    Custom,
    CaseSize,
    Purge,
    AddText,
    Trim,
}
#[allow(dead_code)]
pub enum RulePlace {
    None,
    Extension,
    Name,
    ExtensionAndName,
    BeforeExtension,
    AfterExtension,
    BeforeName,
    AfterName,
    FromNameStart,
    FromNameEndReverse,
    FromExtensionStart,
    FromExtensionEndReverse,
}
pub fn rule_type_to_string(rule_type: &RuleType) -> String {
    match rule_type {
        RuleType::Custom => "Custom",
        RuleType::CaseSize => "CaseSize",
        RuleType::Purge => "Purge",
        RuleType::AddText => "Add Text",
        RuleType::Trim => "Trim",
    }
    .to_string()
}
pub fn rule_place_to_string(rule_type: &RulePlace) -> String {
    match rule_type {
        RulePlace::None => "N/A",
        RulePlace::Extension => "Only Extension",
        RulePlace::Name => "Only Name",
        RulePlace::ExtensionAndName => "Extension and Name",
        RulePlace::BeforeExtension => "Before Extension",
        RulePlace::AfterExtension => "After Extension",
        RulePlace::BeforeName => "Before Name",
        RulePlace::AfterName => "After Name",
        RulePlace::FromNameStart => "From Start",
        RulePlace::FromNameEndReverse => "From Name End to Start",
        RulePlace::FromExtensionStart => "From Extension Start",
        RulePlace::FromExtensionEndReverse => "From Extension End to Start",
    }
    .to_string()
}

pub struct RuleData {
    pub add_text_text: String,
    pub trim_text: String,
    pub to_lowercase: bool,
    pub case_sensitive: bool,
    pub custom_text: String,

    pub number_start: i64,
    pub number_step: i64,
    pub fill_with_zeros: i64,
}
impl RuleData {
    pub fn new() -> Self {
        RuleData {
            add_text_text: "".to_string(),
            trim_text: "".to_string(),
            to_lowercase: false,
            case_sensitive: false,
            custom_text: "".to_string(),
            number_start: 0,
            number_step: 0,
            fill_with_zeros: 0,
        }
    }
}
