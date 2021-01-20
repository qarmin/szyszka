use crate::rule_add_text::rule_add_text;
use crate::rule_change_size_letters::rule_change_size_letters;
use crate::rule_purge::rule_purge;
use crate::rule_trim::rule_trim;

pub struct Rules {
    pub rule_types: Vec<RuleType>,
    pub rule_place: Vec<RulePlace>,
    pub rule_case_sensitivity: Vec<RuleCaseSensitivity>,
    pub rules_number: usize,
    pub rule_data: Vec<RuleData>,
}

impl Rules {
    pub fn new() -> Self {
        Rules {
            rule_types: vec![],
            rule_place: vec![],
            rule_case_sensitivity: vec![],
            rules_number: 0,
            rule_data: vec![],
        }
    }
    pub fn add_rule(&mut self, rule_type: RuleType, rule_place: RulePlace, rule_case_sensitivity: RuleCaseSensitivity) {
        self.rule_types.push(rule_type);
        self.rule_place.push(rule_place);
        self.rule_case_sensitivity.push(rule_case_sensitivity);
        self.rules_number += 1;
    }
    pub fn apply_all_rules_to_item(&mut self, mut item: String) -> String {
        for rule_number in 0..self.rules_number {
            match self.rule_types[rule_number] {
                RuleType::UpperCase | RuleType::LowerCase => {
                    item = rule_change_size_letters(item.as_str(), &self.rule_types[rule_number], &self.rule_place[rule_number]);
                }
                RuleType::Purge => {
                    item = rule_purge(item.as_str(), &self.rule_types[rule_number], &self.rule_place[rule_number]);
                }
                RuleType::AddText => {
                    item = rule_add_text(item.as_str(), &self.rule_types[rule_number], &self.rule_place[rule_number], &self.rule_data[rule_number]);
                }
                RuleType::Trim => {
                    item = rule_trim(item.as_str(), &self.rule_types[rule_number], &self.rule_place[rule_number], &self.rule_case_sensitivity[rule_number], &self.rule_data[rule_number]);
                }
            }
        }
        item
    }
}

#[derive(Eq, PartialEq)]
pub enum RuleCaseSensitivity {
    Sensitive,
    Insensitive,
}

pub enum RuleType {
    UpperCase,
    LowerCase,
    Purge,
    AddText,
    Trim,
}
#[allow(dead_code)]
pub enum RulePlace {
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
        RuleType::UpperCase => "UpperCase",
        RuleType::LowerCase => "LowerCase",
        RuleType::Purge => "Purge",
        RuleType::AddText => "Add Text",
        RuleType::Trim => "Trim",
    }
    .to_string()
}
pub fn rule_place_to_string(rule_type: &RulePlace) -> String {
    match rule_type {
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
}
impl RuleData {
    pub fn new() -> Self {
        RuleData {
            add_text_text: "".to_string(),
            trim_text: "".to_string(),
        }
    }
}
