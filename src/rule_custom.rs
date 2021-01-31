use crate::help_function::split_file_name;
use crate::rules::*;
use chrono::NaiveDateTime;
use humansize::{file_size_opts as options, FileSize};
use std::fs::File;
use std::path::Path;
use std::time::UNIX_EPOCH;

pub fn rule_custom(data_to_change: &str, rule_type: &RuleType, rule_place: &RulePlace, rule_data: &RuleData, rule_number: u64, example: bool) -> String {
    let (name, extension) = split_file_name(Path::new(data_to_change));
    let mut return_string = rule_data.custom_text.clone();

    let mut creation_date: String = "".to_string();
    let mut modification_date: String = "".to_string();
    let mut size: String = "".to_string();

    #[allow(clippy::collapsible_if)]
    // Random data to visualize typical
    if example {
        creation_date = "2021-01-31 08:42:12".to_string();
        modification_date = "2015-11-15 14:24:55".to_string();
        size = "2 KB".to_string();
    } else {
        if let Ok(file) = File::open(data_to_change) {
            if let Ok(metadata) = file.metadata() {
                if let Ok(system_time) = metadata.created() {
                    if let Ok(creation) = system_time.duration_since(UNIX_EPOCH) {
                        creation_date = NaiveDateTime::from_timestamp(creation.as_secs() as i64, 0).to_string();
                    }
                }
                if let Ok(system_time) = metadata.modified() {
                    if let Ok(modified) = system_time.duration_since(UNIX_EPOCH) {
                        modification_date = NaiveDateTime::from_timestamp(modified.as_secs() as i64, 0).to_string();
                    }
                }

                size = metadata.len().file_size(options::BINARY).unwrap()
            }
        }
    }

    match rule_type {
        RuleType::Custom => match rule_place {
            RulePlace::None => {
                return_string = return_string.replace("$(NAME)", name.as_str());
                return_string = return_string.replace("$(EXT)", extension.as_str());
                return_string = return_string.replace("$(SIZE)", size.as_str());
                return_string = return_string.replace("$(CREAT)", creation_date.as_str());
                return_string = return_string.replace("$(MODIF)", modification_date.as_str());
                return_string = return_string.replace("$(CURR)", data_to_change);

                // Part with numbers
                if return_string.find("$(N:").is_some() {
                    let mut index = 0;
                    while let Some(mut start_index) = return_string[index..].find("$(N:") {
                        start_index += index + 4;
                        if let Some(mut first_double) = return_string[start_index..].find(':') {
                            first_double += start_index;
                            if let Some(mut second_double) = return_string[first_double + 1..].find(':') {
                                second_double += first_double + 1;
                                if let Some(mut end_index) = return_string[second_double..].find(')') {
                                    end_index += second_double;

                                    let str_start_number = return_string[start_index..first_double].to_string();
                                    let str_step_number = return_string[first_double + 1..second_double].to_string();
                                    let str_fill_zeros = return_string[second_double + 1..end_index].to_string();

                                    if let Ok(start_number) = str_start_number.parse::<u64>() {
                                        if let Ok(step_number) = str_step_number.parse::<u64>() {
                                            if let Ok(fill_zeros) = str_fill_zeros.parse::<u64>() {
                                                let mut number;
                                                if step_number.checked_mul(rule_number).is_none() {
                                                    number = 0;
                                                } else {
                                                    number = step_number * rule_number;
                                                }

                                                if number.checked_add(start_number).is_none() {
                                                    number = 0;
                                                } else {
                                                    number += start_number;
                                                }

                                                let mut text_to_replace = number.to_string();

                                                let mut zeros: String = "".to_string();
                                                if text_to_replace.len() < fill_zeros as usize {
                                                    for _i in 0..(fill_zeros - text_to_replace.len() as u64) {
                                                        zeros.push('0');
                                                    }
                                                    text_to_replace = zeros + text_to_replace.as_str();
                                                }

                                                return_string = format!("{}{}{}", &return_string[..start_index - 4], text_to_replace, &return_string[end_index + 1..]);

                                                index = start_index - 4 + text_to_replace.len();
                                                continue;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        // Some things was not parsed successfully, so we need to start 4 positions later(after first "$(N:")
                        index = start_index;
                    }
                }
            }
            _ => panic!("Invalid Rule Place for Custom"),
        },
        _ => panic!("Invalid Rule Type for Custom"),
    }

    return_string
}

#[cfg(test)]
mod test {
    use crate::rule_custom::rule_custom;
    use crate::rules::{RuleData, RulePlace, RuleType};
    use std::fs;
    use std::fs::OpenOptions;
    use std::io::Write;
    use std::path::Path;

    #[test]
    fn test_custom() {
        let mut rule_data: RuleData = RuleData::new();

        let mut file_handler = OpenOptions::new().truncate(true).write(true).create(true).open(Path::new("wombat.txt")).unwrap();
        write!(file_handler, "50").unwrap();
        file_handler.flush().unwrap();

        rule_data.custom_text = "$(CURR)".to_string();
        assert_eq!(rule_custom("wombat.txt", &RuleType::Custom, &RulePlace::None, &rule_data, 0, false), "wombat.txt");

        rule_data.custom_text = "$(NAME)".to_string();
        assert_eq!(rule_custom("wombat.txt", &RuleType::Custom, &RulePlace::None, &rule_data, 0, false), "wombat");

        rule_data.custom_text = "$(EXT)".to_string();
        assert_eq!(rule_custom("wombat.txt", &RuleType::Custom, &RulePlace::None, &rule_data, 0, false), "txt");

        rule_data.custom_text = "$(N:)".to_string();
        assert_eq!(rule_custom("wombat.txt", &RuleType::Custom, &RulePlace::None, &rule_data, 0, false), "$(N:)");
        rule_data.custom_text = "$(N:20:22:)".to_string();
        assert_eq!(rule_custom("wombat.txt", &RuleType::Custom, &RulePlace::None, &rule_data, 0, false), "$(N:20:22:)");
        rule_data.custom_text = "$(20::22)".to_string();
        assert_eq!(rule_custom("wombat.txt", &RuleType::Custom, &RulePlace::None, &rule_data, 0, false), "$(20::22)");
        rule_data.custom_text = "$(N:::22)".to_string();
        assert_eq!(rule_custom("wombat.txt", &RuleType::Custom, &RulePlace::None, &rule_data, 0, false), "$(N:::22)");
        rule_data.custom_text = "$(:::)".to_string();
        assert_eq!(rule_custom("wombat.txt", &RuleType::Custom, &RulePlace::None, &rule_data, 0, false), "$(:::)");
        rule_data.custom_text = "$(N:20:22:4)".to_string();
        assert_eq!(rule_custom("wombat.txt", &RuleType::Custom, &RulePlace::None, &rule_data, 0, false), "0020");
        assert_eq!(rule_custom("wombat.txt", &RuleType::Custom, &RulePlace::None, &rule_data, 1, false), "0042");
        assert_eq!(rule_custom("wombat.txt", &RuleType::Custom, &RulePlace::None, &rule_data, 2, false), "0064");
        rule_data.custom_text = "$(N:1:10:3)$(N:2:10:4)".to_string();
        assert_eq!(rule_custom("wombat.txt", &RuleType::Custom, &RulePlace::None, &rule_data, 0, false), "0010002");
        assert_eq!(rule_custom("wombat.txt", &RuleType::Custom, &RulePlace::None, &rule_data, 1, false), "0110012");
        rule_data.custom_text = "$(N:0:2:5)".to_string();
        assert_eq!(rule_custom("wombat.txt", &RuleType::Custom, &RulePlace::None, &rule_data, 0, false), "00000");
        assert_eq!(rule_custom("wombat.txt", &RuleType::Custom, &RulePlace::None, &rule_data, 1, false), "00002");
        assert_eq!(rule_custom("wombat.txt", &RuleType::Custom, &RulePlace::None, &rule_data, 2, false), "00004");
        rule_data.custom_text = "$(N:10:5:1)".to_string();
        assert_eq!(rule_custom("wombat.txt", &RuleType::Custom, &RulePlace::None, &rule_data, 0, false), "10");
        assert_eq!(rule_custom("wombat.txt", &RuleType::Custom, &RulePlace::None, &rule_data, 1, false), "15");
        assert_eq!(rule_custom("wombat.txt", &RuleType::Custom, &RulePlace::None, &rule_data, 2, false), "20");
        rule_data.custom_text = "$(EXT)$(())$(($(N:10:5:1)".to_string();
        assert_eq!(rule_custom("wombat.txt", &RuleType::Custom, &RulePlace::None, &rule_data, 0, false), "txt$(())$((10");

        // This depends on files which doesn't exists, so result should be an empty string

        rule_data.custom_text = "$(SIZE)".to_string();
        assert_eq!(rule_custom("wombat.txt", &RuleType::Custom, &RulePlace::None, &rule_data, 0, false), "2 B");

        rule_data.custom_text = "$(MODIF)".to_string();
        let text = rule_custom("wombat.txt", &RuleType::Custom, &RulePlace::None, &rule_data, 0, false);
        assert!(text.contains('-') && text.contains("20"));

        rule_data.custom_text = "$(CREAT)".to_string();
        let text = rule_custom("wombat.txt", &RuleType::Custom, &RulePlace::None, &rule_data, 0, false);
        assert!(text.contains('-') && text.contains("20"));

        fs::remove_file("wombat.txt").unwrap();
    }
}
