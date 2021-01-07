use crate::rules::*;

// TODO Add tests

pub fn rule_change_size_letters(data_to_change: &str, rule_type: &RuleType, rule_place: &RulePlace) -> String {
    let split = data_to_change.split('.').map(|e| e.to_string()).collect::<Vec<String>>();
    let mut name = split[0].clone();
    let mut extension = if split.len() > 1 { split[1].clone() } else { "".to_string() };

    match rule_type {
        RuleType::UpperCase => match rule_place {
            RulePlace::Name => {
                name = name.to_uppercase();
            }
            RulePlace::ExtensionAndName => {
                name = name.to_uppercase();
                extension = extension.to_uppercase();
            }
            RulePlace::Extension => {
                extension = extension.to_uppercase();
            }
            _ => {
                panic!("Not implemented function");
            }
        },
        RuleType::LowerCase => {
            match rule_place {
                RulePlace::Name => {
                    name = name.to_lowercase();
                }
                RulePlace::ExtensionAndName => {
                    name = name.to_lowercase();
                    extension = extension.to_lowercase();
                }
                RulePlace::Extension => {
                    extension = extension.to_lowercase();
                }
                _ => {
                    panic!("Not implemented function");
                }
            }
            //_ => panic!("Invalid Rule Type for change size of letter");
        }
    }

    // Handle also situation when e.g. file is "file." where there is not extension, but
    if !extension.is_empty() || data_to_change.contains('.') {
        format!("{}.{}", name, extension)
    } else {
        name
    }
}
