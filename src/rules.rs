use crate::rule_add_number::rule_add_number;
use crate::rule_add_text::rule_add_text;
use crate::rule_change_size_letters::rule_change_size_letters;
use crate::rule_custom::rule_custom;
use crate::rule_purge::rule_purge;
use crate::rule_replace::rule_replace;
use crate::rule_trim::rule_trim;

#[derive(Clone)]
pub struct SingleRule {
    pub rule_type: RuleType,
    pub rule_place: RulePlace,
    pub rule_data: RuleData,
    pub rule_description: String,
}
impl SingleRule {
    pub fn new() -> Self {
        SingleRule {
            rule_type: RuleType::Custom,
            rule_place: RulePlace::None,
            rule_data: RuleData::new(),
            rule_description: "".to_string(),
        }
    }
    pub fn create_rule(rule_type: RuleType, rule_place: RulePlace, rule_data: RuleData, rule_description: String) -> Self {
        SingleRule {
            rule_type,
            rule_place,
            rule_data,
            rule_description,
        }
    }
}

pub struct Rules {
    pub rules: Vec<SingleRule>,
    pub edit_mode: Option<usize>, // Used to store index of changed rule
    pub updated: bool,            // Used to warn user if records needs to be updated
}

impl Rules {
    pub fn new() -> Self {
        Rules { rules: vec![], edit_mode: None, updated: true }
    }
    pub fn add_rule(&mut self, rule_type: RuleType, rule_place: RulePlace, rule_data: RuleData, rule_description: String) {
        self.rules.push(SingleRule::create_rule(rule_type, rule_place, rule_data, rule_description));
    }
    pub fn remove_rule(&mut self, index: usize) {
        self.rules.remove(index);
    }
    pub fn apply_all_rules_to_item(&mut self, mut item: String, current_index: u64, file_data: (u64, u64, u64)) -> String {
        for rule in &self.rules {
            match rule.rule_type {
                RuleType::CaseSize => {
                    item = rule_change_size_letters(item.as_str(), rule);
                }
                RuleType::Purge => {
                    item = rule_purge(item.as_str(), rule);
                }
                RuleType::AddText => {
                    item = rule_add_text(item.as_str(), rule);
                }
                RuleType::Trim => {
                    item = rule_trim(item.as_str(), rule);
                }
                RuleType::Custom => {
                    item = rule_custom(item.as_str(), rule, current_index, Some(file_data));
                }
                RuleType::Replace => {
                    item = rule_replace(item.as_str(), rule);
                }
                RuleType::AddNumber => {
                    item = rule_add_number(item.as_str(), rule, current_index);
                }
            }
        }
        item
    }
}

#[derive(Clone)]
pub enum RuleType {
    Custom = 0,
    CaseSize,
    Purge,
    AddNumber,
    AddText,
    Replace,
    Trim,
}
#[allow(dead_code)]
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum RulePlace {
    None = 0,
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
        RuleType::CaseSize => "Case Size",
        RuleType::Purge => "Purge",
        RuleType::AddText => "Add Text",
        RuleType::Trim => "Trim",
        RuleType::Replace => "Replace",
        RuleType::AddNumber => "Add Number",
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

#[derive(Clone)]
pub struct RuleData {
    pub add_text_text: String,
    pub trim_text: String,
    pub to_lowercase: bool,
    pub case_sensitive: bool,
    pub custom_text: String,

    pub number_start: i64,
    pub number_step: i64,
    pub fill_with_zeros: i64,

    pub text_to_remove: String,
    pub text_to_replace: String,
}
impl RuleData {
    // A little wasteful, but rules will be max 10 most of time, so this is not necessary to optimize
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
            text_to_remove: "".to_string(),
            text_to_replace: "".to_string(),
        }
    }
}
