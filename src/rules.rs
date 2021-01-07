use crate::rule_change_size_letters::rule_change_size_letters;

pub struct Rules {
    pub rule_types: Vec<RuleType>,
    pub rule_place: Vec<RulePlace>,
    pub rules_number: usize,
}

impl Rules {
    pub fn new() -> Self {
        Rules {
            rule_types: vec![],
            rule_place: vec![],
            rules_number: 0,
        }
    }
    pub fn add_rule(&mut self, rule_type: RuleType, rule_place: RulePlace) {
        self.rule_types.push(rule_type);
        self.rule_place.push(rule_place);
        self.rules_number += 1;
    }
    pub fn apply_all_rules_to_item(&mut self, mut item: String) -> String {
        for rule_number in 0..self.rules_number {
            match self.rule_types[rule_number] {
                RuleType::UpperCase | RuleType::LowerCase => {
                    item = rule_change_size_letters(item.as_str(), &self.rule_types[rule_number], &self.rule_place[rule_number]);
                }
            }
        }
        item
    }
}

pub enum RuleType {
    UpperCase,
    LowerCase,
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
}
pub fn rule_type_to_string(rule_type: &RuleType) -> String {
    match rule_type {
        RuleType::UpperCase => "UpperCase",
        RuleType::LowerCase => "LowerCase",
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
    }
    .to_string()
}
