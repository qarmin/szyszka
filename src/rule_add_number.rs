use crate::help_function::split_file_name;
use crate::rules::*;
use std::cmp::min;
use std::path::Path;

pub fn rule_add_number(data_to_change: &str, rule_type: &RuleType, rule_place: &RulePlace, rule_data: &RuleData, rule_number: u64) -> String {
    let (name, extension) = split_file_name(Path::new(data_to_change));
    let mut return_string;
    let is_empty_extension_and_dot_at_the_end = extension.is_empty() && data_to_change.ends_with('.');

    let start_number = rule_data.number_start;
    let step_number = rule_data.number_step;
    let fill_with_zeros = rule_data.fill_with_zeros;

    match rule_type {
        RuleType::AddNumber => {
            // TODO think about putting it to docs or explaining it somewhere that bigger values will crash entire app
            let fill_with_zeros = min(fill_with_zeros, 50);

            let mut number: i64;
            if step_number.checked_mul(rule_number as i64).is_none() {
                number = 0;
            } else {
                number = step_number * rule_number as i64;
            }

            number = number.checked_add(start_number).unwrap_or(0);

            let mut text_to_add = number.to_string();

            let mut zeros: String = "".to_string();
            if text_to_add.len() < fill_with_zeros as usize {
                for _i in 0..(fill_with_zeros - text_to_add.len() as i64) {
                    zeros.push('0');
                }
                text_to_add = zeros + text_to_add.as_str();
            }

            match rule_place {
                RulePlace::BeforeName => {
                    return_string = text_to_add + name.as_str();
                }
                RulePlace::AfterName => {
                    return_string = name + text_to_add.as_str();
                }
                _ => {
                    panic!("Not implemented function");
                }
            }

            if !extension.is_empty() {
                return_string = return_string + "." + extension.as_str();
            } else if is_empty_extension_and_dot_at_the_end {
                return_string += ".";
            }
        }
        _ => {
            panic!("Not implemented function");
        }
    }

    return_string
}

#[cfg(test)]
mod test {
    use crate::rule_add_number::rule_add_number;
    use crate::rules::{RuleData, RulePlace, RuleType};

    #[test]
    fn test_add_number() {
        let mut rule_data: RuleData = RuleData::new();

        rule_data.number_start = 10;
        rule_data.number_step = 5;
        rule_data.fill_with_zeros = 4;
        assert_eq!(rule_add_number("Roman.txt", &RuleType::Trim, &RulePlace::FromExtensionEndReverse, &rule_data, 0), "Roman.");
        assert_eq!(rule_add_number("Roman.txt", &RuleType::Trim, &RulePlace::FromExtensionEndReverse, &rule_data, 0), "Roman.");
    }
}
