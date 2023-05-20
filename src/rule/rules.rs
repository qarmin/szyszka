use crate::fls;
use crate::rule::rule_add_number::rule_add_number;
use crate::rule::rule_add_text::rule_add_text;
use crate::rule::rule_change_size_letters::rule_change_size_letters;
use crate::rule::rule_custom::rule_custom;
use crate::rule::rule_normalize::rule_normalize;
use crate::rule::rule_purge::rule_purge;
use crate::rule::rule_replace::rule_replace;
use crate::rule::rule_trim::rule_trim;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MultipleRules {
    pub name: String,
    pub rules: Vec<SingleRule>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SingleRule {
    pub rule_type: RuleType,
    pub rule_place: RulePlace,
    pub rule_data: RuleData,
    pub rule_description: String,
}

impl SingleRule {
    #[allow(dead_code)] // Used in tests for now
    pub fn new() -> Self {
        SingleRule {
            rule_type: RuleType::Custom,
            rule_place: RulePlace::None,
            rule_data: RuleData::new(),
            rule_description: String::new(),
        }
    }
    // pub fn create_rule(rule_type: RuleType, rule_place: RulePlace, rule_data: RuleData, rule_description: String) -> Self {
    //     SingleRule {
    //         rule_type,
    //         rule_place,
    //         rule_data,
    //         rule_description,
    //     }
    // }
}
#[derive(Clone, Debug)]
pub struct Rules {
    pub rules: Vec<SingleRule>,
    pub edit_mode: Option<usize>,
    // Used to store index of changed rule
    pub updated: bool, // Used to warn user if records needs to be updated
}

impl Rules {
    pub fn new() -> Self {
        Rules { rules: vec![], edit_mode: None, updated: true }
    }
    // pub fn add_rule(&mut self, rule_type: RuleType, rule_place: RulePlace, rule_data: RuleData, rule_description: String) {
    //     self.rules.push(SingleRule::create_rule(rule_type, rule_place, rule_data, rule_description));
    // }
    pub fn add_single_rule(&mut self, single_rule: SingleRule) {
        self.rules.push(single_rule);
    }
    pub fn remove_rule(&mut self, index: usize) {
        self.rules.remove(index);
    }
    pub fn apply_all_rules_to_item(&mut self, mut item: String, current_index: u64, current_index_in_folder: u32, file_data: (u64, u64, u64, &str), compiled_regexes: &[Option<Regex>]) -> String {
        debug_assert_eq!(self.rules.len(), compiled_regexes.len());
        for (rule, regex) in (self.rules.iter()).zip(compiled_regexes.iter()) {
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
                    item = rule_custom(item.as_str(), rule, current_index, current_index_in_folder as u64, Some(file_data));
                }
                RuleType::Replace => {
                    item = rule_replace(item.as_str(), rule, regex);
                }
                RuleType::AddNumber => {
                    item = rule_add_number(item.as_str(), rule, current_index);
                }
                RuleType::Normalize => {
                    item = rule_normalize(item.as_str(), rule);
                }
            }
        }
        item
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum RuleType {
    Custom = 0,
    CaseSize,
    Purge,
    AddNumber,
    AddText,
    Replace,
    Trim,
    Normalize,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Serialize, Deserialize)]
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
        RuleType::Custom => fls!("rule_type_custom"),
        RuleType::CaseSize => fls!("rule_type_case_size"),
        RuleType::Purge => fls!("rule_type_purge"),
        RuleType::AddText => fls!("rule_type_add_text"),
        RuleType::Trim => fls!("rule_type_trim"),
        RuleType::Replace => fls!("rule_type_replace"),
        RuleType::AddNumber => fls!("rule_type_add_number"),
        RuleType::Normalize => fls!("rule_type_normalize"),
    }
}

pub fn rule_place_to_string(rule_type: &RulePlace) -> String {
    match rule_type {
        RulePlace::None => fls!("rule_place_none"),
        RulePlace::Extension => fls!("rule_place_extension"),
        RulePlace::Name => fls!("rule_place_name"),
        RulePlace::ExtensionAndName => fls!("rule_place_extension_name"),
        RulePlace::BeforeExtension => fls!("rule_place_before_extension"),
        RulePlace::AfterExtension => fls!("rule_place_after_extension"),
        RulePlace::BeforeName => fls!("rule_place_before_name"),
        RulePlace::AfterName => fls!("rule_place_after_name"),
        RulePlace::FromNameStart => fls!("rule_place_from_name_start"),
        RulePlace::FromNameEndReverse => fls!("rule_place_from_name_end_reverse"),
        RulePlace::FromExtensionStart => fls!("rule_place_from_extension_start"),
        RulePlace::FromExtensionEndReverse => fls!("rule_place_from_extension_end_reverse"),
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RuleData {
    pub add_text_text: String,
    pub trim_text: String,
    pub to_lowercase: bool,
    pub case_sensitive: bool,
    pub custom_text: String,

    pub number_start: i64,
    pub number_step: i64,
    pub fill_with_zeros: i64,

    pub text_to_find: String,
    pub text_to_replace: String,
    pub use_regex: bool,
    pub regex_replace_all: bool,

    pub full_normalize: bool,
}

impl RuleData {
    // A little wasteful, but rules will be max 10 most of time, so this is not necessary to optimize
    pub fn new() -> Self {
        RuleData {
            add_text_text: String::new(),
            trim_text: String::new(),
            to_lowercase: false,
            case_sensitive: false,
            custom_text: String::new(),
            number_start: 0,
            number_step: 0,
            fill_with_zeros: 0,
            text_to_find: String::new(),
            text_to_replace: String::new(),
            use_regex: false,
            regex_replace_all: false,
            full_normalize: false,
        }
    }
}
