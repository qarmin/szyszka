use regex::Regex;
use std::path::Path;

use crate::help_function::split_file_name;
use crate::rule::rules::*;

pub fn rule_replace(data_to_change: &str, rule: &SingleRule, regex: &Option<Regex>) -> String {
    // No data to change
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
                    return_string = regex.replace_all(data_to_change, text_to_replace.as_str()).to_string();
                } else {
                    return_string = data_to_change.to_string(); // Regex is broken, do not change content
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
                        extension = format!("{}{}{}", &extension[..start_index], text_to_replace, &extension[start_index + text_to_find_lowercase.len()..]);
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
                        data_to_change = format!("{}{}{}", &data_to_change[..start_index], text_to_replace, &data_to_change[start_index + text_to_find_lowercase.len()..]);
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
    use regex::Regex;

    #[test]
    fn test_replace() {
        let mut rule = SingleRule::new();
        rule.rule_type = RuleType::Replace;

        rule.rule_place = RulePlace::Name;
        rule.rule_data.text_to_find = "konstantynopolitańczykiewiczówna".to_string();
        rule.rule_data.text_to_replace = "rar".to_string();
        rule.rule_data.case_sensitive = false;
        assert_eq!(rule_replace("QKonstantynopolitańczykiewiczówna.txt", &rule, &None), "Qrar.txt");
        rule.rule_data.case_sensitive = true;
        assert_eq!(rule_replace("QKonstantynopolitańczykiewiczówna.txt", &rule, &None), "QKonstantynopolitańczykiewiczówna.txt");

        rule.rule_place = RulePlace::ExtensionAndName;
        rule.rule_data.text_to_find = "qw.".to_string();
        rule.rule_data.text_to_replace = "tw".to_string();
        rule.rule_data.case_sensitive = false;
        assert_eq!(rule_replace("QQw.Qw.txt", &rule, &None), "Qtwtwtxt");
        rule.rule_data.case_sensitive = true;
        assert_eq!(rule_replace("QQw.txt", &rule, &None), "QQw.txt");

        rule.rule_place = RulePlace::ExtensionAndName;
        rule.rule_data.text_to_find = "rrra".to_string();
        rule.rule_data.text_to_replace = "rr".to_string();
        rule.rule_data.case_sensitive = false;
        assert_eq!(rule_replace("Qsr.RrRa", &rule, &None), "Qsr.rr");
        rule.rule_data.case_sensitive = true;
        assert_eq!(rule_replace("Qsr.RrRarrra", &rule, &None), "Qsr.RrRarr");

        rule.rule_place = RulePlace::ExtensionAndName;
        rule.rule_data.text_to_find = "a".to_string();
        rule.rule_data.text_to_replace = "aa".to_string();
        rule.rule_data.case_sensitive = false;
        assert_eq!(rule_replace("aaa", &rule, &None), "aaaaaa");
        rule.rule_data.case_sensitive = true;
        assert_eq!(rule_replace("aaa", &rule, &None), "aaaaaa");
    }
    #[test]
    fn test_replace_regex() {
        let mut rule = SingleRule::new();
        rule.rule_type = RuleType::Replace;
        rule.rule_place = RulePlace::None;
        let mut regex;

        rule.rule_data.text_to_find = "roman_staszek".to_string();
        rule.rule_data.text_to_replace = "RRR".to_string();
        regex = Regex::new(&rule.rule_data.text_to_find).unwrap();
        assert_eq!(rule_replace("ABCD_roman_staszek_BSDE", &rule, &Some(regex)), "ABCD_RRR_BSDE");

        rule.rule_data.text_to_find = "([a-z]+)".to_string();
        rule.rule_data.text_to_replace = "PRP".to_string();
        regex = Regex::new(&rule.rule_data.text_to_find).unwrap();
        assert_eq!(rule_replace("ABCD_roman_staszek_BSDE", &rule, &Some(regex)), "ABCD_PRP_PRP_BSDE");

        rule.rule_data.text_to_find = "([a-z]+)".to_string();
        rule.rule_data.text_to_replace = "PRP".to_string();
        regex = Regex::new(&rule.rule_data.text_to_find).unwrap();
        dbg!(&regex.captures("ABCD_roman_staszek_BSDE"));
        assert_eq!(rule_replace("ABCD_roman_staszek_BSDE", &rule, &Some(regex)), "ABCD_PRP_PRP_BSDE");
    }
}
