use crate::rule::rules::{RulePlace, RuleType, SingleRule};

pub fn rule_normalize(data_to_change: &str, rule: &SingleRule) -> String {
    let return_string;

    match rule.rule_type {
        RuleType::Normalize => match rule.rule_place {
            RulePlace::ExtensionAndName => {
                return_string = slugmin::slugify_normal(data_to_change, !rule.rule_data.full_normalize);
            }
            _ => panic!("Invalid Rule Place for Normalization"),
        },
        _ => panic!("Invalid Rule Type for Normalization"),
    }

    return_string
}
