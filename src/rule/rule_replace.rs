use crate::help_function::split_file_name;
use crate::rule::rules::*;
use std::path::Path;

pub fn rule_replace(data_to_change: &str, rule: &SingleRule) -> String {
    // No data to change
    if rule.rule_data.text_to_remove.is_empty() {
        return data_to_change.to_string();
    }

    let (name, extension) = split_file_name(Path::new(data_to_change));
    let mut return_string = data_to_change.to_string();
    let is_empty_extension_and_dot_at_the_end = extension.is_empty() && data_to_change.ends_with('.');

    let data_to_change_lowercase = data_to_change.to_lowercase();
    let name_lowercase = name.to_lowercase();
    let extension_lowercase = extension.to_lowercase();
    let text_to_remove = rule.rule_data.text_to_remove.clone();
    let text_to_remove_lowercase = text_to_remove.to_lowercase();
    let text_to_replace = rule.rule_data.text_to_replace.clone();

    match rule.rule_type {
        RuleType::Replace => match rule.rule_place {
            RulePlace::Name => {
                if rule.rule_data.case_sensitive && name.contains(&text_to_remove) {
                    return_string = data_to_change.replace(text_to_remove.as_str(), text_to_replace.as_str());
                } else if !rule.rule_data.case_sensitive && name_lowercase.contains(&text_to_remove_lowercase) {
                    let mut name = name;
                    let mut start_index = 0;
                    while let Some(index) = name[start_index..].to_lowercase().find(&text_to_remove_lowercase) {
                        start_index += index;
                        name = format!("{}{}{}", &name[..start_index], text_to_replace, &name[start_index + text_to_remove_lowercase.len()..]);
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
                if rule.rule_data.case_sensitive && extension.contains(&text_to_remove) {
                    return_string = data_to_change.replace(text_to_remove.as_str(), text_to_replace.as_str());
                } else if !rule.rule_data.case_sensitive && extension_lowercase.contains(&text_to_remove_lowercase) {
                    let mut extension = extension;
                    let mut start_index = 0;
                    while let Some(index) = extension[start_index..].to_lowercase().find(&text_to_remove_lowercase) {
                        start_index += index;
                        extension = format!("{}{}{}", &extension[..start_index], text_to_replace, &extension[start_index + text_to_remove_lowercase.len()..]);
                        start_index = (text_to_replace.len() as isize + start_index as isize) as usize;
                    }
                    return_string = format!("{}.{}", name, extension);
                }
            }
            RulePlace::ExtensionAndName => {
                if rule.rule_data.case_sensitive && data_to_change.contains(&text_to_remove) {
                    return_string = data_to_change.replace(text_to_remove.as_str(), text_to_replace.as_str());
                } else if !rule.rule_data.case_sensitive && data_to_change_lowercase.contains(&text_to_remove_lowercase) {
                    let mut data_to_change = data_to_change.to_string();
                    let mut start_index = 0;
                    while let Some(index) = data_to_change[start_index..].to_lowercase().find(&text_to_remove_lowercase) {
                        start_index += index;
                        data_to_change = format!("{}{}{}", &data_to_change[..start_index], text_to_replace, &data_to_change[start_index + text_to_remove_lowercase.len()..]);
                        start_index = (text_to_replace.len() as isize + start_index as isize) as usize;
                    }
                    return_string = data_to_change;
                }
            }
            _ => {
                panic!("Not implemented function");
            }
        },
        _ => {
            panic!("Not implemented function");
        }
    }

    return_string
}

#[cfg(test)]
mod test {
    use crate::rule::rule_replace::rule_replace;
    use crate::rule::rules::{RulePlace, RuleType, SingleRule};

    #[test]
    fn test_replace() {
        let mut rule = SingleRule::new();
        rule.rule_type = RuleType::Replace;

        rule.rule_place = RulePlace::Name;
        rule.rule_data.text_to_remove = "konstantynopolitańczykiewiczówna".to_string();
        rule.rule_data.text_to_replace = "rar".to_string();
        rule.rule_data.case_sensitive = false;
        assert_eq!(rule_replace("QKonstantynopolitańczykiewiczówna.txt", &rule), "Qrar.txt");
        rule.rule_data.case_sensitive = true;
        assert_eq!(rule_replace("QKonstantynopolitańczykiewiczówna.txt", &rule), "QKonstantynopolitańczykiewiczówna.txt");

        rule.rule_place = RulePlace::ExtensionAndName;
        rule.rule_data.text_to_remove = "qw.".to_string();
        rule.rule_data.text_to_replace = "tw".to_string();
        rule.rule_data.case_sensitive = false;
        assert_eq!(rule_replace("QQw.Qw.txt", &rule), "Qtwtwtxt");
        rule.rule_data.case_sensitive = true;
        assert_eq!(rule_replace("QQw.txt", &rule), "QQw.txt");

        rule.rule_place = RulePlace::ExtensionAndName;
        rule.rule_data.text_to_remove = "rrra".to_string();
        rule.rule_data.text_to_replace = "rr".to_string();
        rule.rule_data.case_sensitive = false;
        assert_eq!(rule_replace("Qsr.RrRa", &rule), "Qsr.rr");
        rule.rule_data.case_sensitive = true;
        assert_eq!(rule_replace("Qsr.RrRarrra", &rule), "Qsr.RrRarr");

        rule.rule_place = RulePlace::ExtensionAndName;
        rule.rule_data.text_to_remove = "a".to_string();
        rule.rule_data.text_to_replace = "aa".to_string();
        rule.rule_data.case_sensitive = false;
        assert_eq!(rule_replace("aaa", &rule), "aaaaaa");
        rule.rule_data.case_sensitive = true;
        assert_eq!(rule_replace("aaa", &rule), "aaaaaa");
    }
}
