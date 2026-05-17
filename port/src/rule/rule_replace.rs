use regex::Regex;
use std::path::Path;

use crate::rule::rules::{split_file_name, RulePlace, RuleType, SingleRule};

pub fn rule_replace(data_to_change: &str, rule: &SingleRule, regex: Option<&Regex>) -> String {
    if rule.rule_data.text_to_find.is_empty() {
        return data_to_change.to_string();
    }

    let (name, extension) = split_file_name(Path::new(data_to_change));
    let mut return_string = data_to_change.to_string();
    let is_empty_extension_and_dot_at_the_end = extension.is_empty() && data_to_change.ends_with('.');

    let data_to_change_lowercase = data_to_change.to_lowercase();
    let name_lowercase = name.to_lowercase();
    let extension_lowercase = extension.to_lowercase();
    let text_to_find = rule.rule_data.text_to_find.clone();
    let text_to_find_lowercase = text_to_find.to_lowercase();
    let text_to_replace = rule.rule_data.text_to_replace.clone();

    match rule.rule_type {
        RuleType::Replace => match rule.rule_place {
            RulePlace::None => {
                if let Some(regex) = regex.as_ref() {
                    if rule.rule_data.regex_replace_all {
                        return_string = regex.replace_all(data_to_change, text_to_replace.as_str()).to_string();
                    } else {
                        return_string = regex.replace(data_to_change, text_to_replace.as_str()).to_string();
                    }
                } else {
                    return_string = data_to_change.to_string();
                }
            }
            RulePlace::Name => {
                if rule.rule_data.case_sensitive && name.contains(&text_to_find) {
                    return_string = data_to_change.replace(text_to_find.as_str(), text_to_replace.as_str());
                } else if !rule.rule_data.case_sensitive && name_lowercase.contains(&text_to_find_lowercase) {
                    let mut name = name;
                    let mut start_index = 0;
                    while let Some(index) = name[start_index..].to_lowercase().find(&text_to_find_lowercase) {
                        start_index += index;
                        name = format!("{}{}{}", &name[..start_index], text_to_replace, &name[start_index + text_to_find_lowercase.len()..]);
                        start_index = (text_to_replace.len() as isize + start_index as isize) as usize;
                    }
                    return_string = name;

                    if !extension.is_empty() {
                        return_string += format!(".{}", extension.as_str()).as_str();
                    } else if is_empty_extension_and_dot_at_the_end {
                        return_string += ".";
                    }
                }
            }
            RulePlace::Extension => {
                if rule.rule_data.case_sensitive && extension.contains(&text_to_find) {
                    return_string = data_to_change.replace(text_to_find.as_str(), text_to_replace.as_str());
                } else if !rule.rule_data.case_sensitive && extension_lowercase.contains(&text_to_find_lowercase) {
                    let mut extension = extension;
                    let mut start_index = 0;
                    while let Some(index) = extension[start_index..].to_lowercase().find(&text_to_find_lowercase) {
                        start_index += index;
                        extension = format!(
                            "{}{}{}",
                            &extension[..start_index],
                            text_to_replace,
                            &extension[start_index + text_to_find_lowercase.len()..]
                        );
                        start_index = (text_to_replace.len() as isize + start_index as isize) as usize;
                    }
                    return_string = format!("{name}.{extension}");
                }
            }
            RulePlace::ExtensionAndName => {
                if rule.rule_data.case_sensitive && data_to_change.contains(&text_to_find) {
                    return_string = data_to_change.replace(text_to_find.as_str(), text_to_replace.as_str());
                } else if !rule.rule_data.case_sensitive && data_to_change_lowercase.contains(&text_to_find_lowercase) {
                    let mut data_to_change = data_to_change.to_string();
                    let mut start_index = 0;
                    while let Some(index) = data_to_change[start_index..].to_lowercase().find(&text_to_find_lowercase) {
                        start_index += index;
                        data_to_change = format!(
                            "{}{}{}",
                            &data_to_change[..start_index],
                            text_to_replace,
                            &data_to_change[start_index + text_to_find_lowercase.len()..]
                        );
                        start_index = (text_to_replace.len() as isize + start_index as isize) as usize;
                    }
                    return_string = data_to_change;
                }
            }
            _ => panic!("Not implemented function"),
        },
        _ => panic!("Not implemented function"),
    }

    return_string
}
