use crate::help_function::split_file_name;
use crate::rule::rules::*;
use std::path::Path;

pub fn rule_purge(data_to_change: &str, rule: &SingleRule) -> String {
    let (name, extension) = split_file_name(Path::new(data_to_change));
    let return_string;

    match rule.rule_type {
        RuleType::Purge => match rule.rule_place {
            RulePlace::Name => {
                return_string = extension;
            }
            RulePlace::ExtensionAndName => {
                return_string = "".to_string();
            }
            RulePlace::Extension => {
                return_string = name;
            }
            _ => {
                panic!("Not implemented function");
            }
        },
        _ => panic!("Invalid Rule Type for purge rule"),
    }

    return_string
}

#[cfg(test)]
mod test {
    use crate::rule::rule_purge::rule_purge;
    use crate::rule::rules::{RulePlace, RuleType, SingleRule};

    #[test]
    fn test_purge() {
        let mut rule = SingleRule::new();

        rule.rule_type = RuleType::Purge;

        rule.rule_place = RulePlace::Name;
        assert_eq!(rule_purge("Roman.txt", &rule), "txt");
        rule.rule_place = RulePlace::Extension;
        assert_eq!(rule_purge("Roman.txt", &rule), "Roman");
        rule.rule_place = RulePlace::ExtensionAndName;
        assert_eq!(rule_purge("Roman.txt", &rule), "");
    }
}
