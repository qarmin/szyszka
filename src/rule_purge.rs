use crate::rules::*;

// TODO Add tests

pub fn rule_purge(data_to_change: &str, rule_type: &RuleType, rule_place: &RulePlace) -> String {
    let split = data_to_change.split('.').map(|e| e.to_string()).collect::<Vec<String>>();
    let name = split[0].clone();
    let extension = if split.len() > 1 { split[1].clone() } else { "".to_string() };
    let return_name;

    match rule_type {
        RuleType::Purge => match rule_place {
            RulePlace::Name => {
                return_name = extension;
            }
            RulePlace::ExtensionAndName => {
                return_name = "".to_string();
            }
            RulePlace::Extension => {
                return_name = name;
            }
            _ => {
                panic!("Not implemented function");
            }
        },
        _ => panic!("Invalid Rule Type for purge rule"),
    }

    return_name
}
