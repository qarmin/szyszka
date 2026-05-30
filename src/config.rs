use std::fs;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use directories_next::ProjectDirs;
use log::error;
use serde::{Deserialize, Serialize};

use crate::rule::rules::MultipleRules;

pub const CUSTOM_TEXT_FILE_NAME: &str = "custom_text_names.txt";
pub const RULES_FILE_NAME: &str = "rules_settings.json";
pub const SETTINGS_FILE_NAME: &str = "settings.json";

const BASIC_CUSTOM_COMMANDS: &str = "FILE_$(N).$(EXT)
FILE_$(K).$(EXT)
$(PARENT) $(N).$(EXT)
$(PARENT) $(K).$(EXT)
";

const BASIC_RULE_CONTENT: &str = "[]";

#[derive(Serialize, Deserialize)]
struct SettingsJson {
    dark_theme: bool,
    language: String,
}

impl Default for SettingsJson {
    fn default() -> Self {
        Self {
            dark_theme: true,
            language: "English".to_string(),
        }
    }
}

pub fn get_config_path() -> Option<PathBuf> {
    ProjectDirs::from("pl", "Qarmin", "Szyszka").map(|p| PathBuf::from(p.config_dir()))
}

pub fn get_settings_file() -> Option<PathBuf> {
    get_config_path().map(|p| p.join(SETTINGS_FILE_NAME))
}

pub fn get_custom_text_config_file() -> Option<PathBuf> {
    get_config_path().map(|p| p.join(CUSTOM_TEXT_FILE_NAME))
}

pub fn get_rules_config_file() -> Option<PathBuf> {
    get_config_path().map(|p| p.join(RULES_FILE_NAME))
}

fn load_settings() -> SettingsJson {
    let Some(path) = get_settings_file() else {
        return SettingsJson::default();
    };
    if !path.is_file() {
        return SettingsJson::default();
    }
    let Ok(file) = fs::File::open(&path) else {
        return SettingsJson::default();
    };
    serde_json::from_reader(BufReader::new(file)).unwrap_or_default()
}

fn save_settings(settings: &SettingsJson) {
    let Some(path) = get_settings_file() else {
        return;
    };
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    if let Ok(serialized) = serde_json::to_string_pretty(settings) {
        let _ = fs::write(path, serialized);
    }
}

pub fn load_dark_theme_config_or_create() -> bool {
    load_settings().dark_theme
}

pub fn save_dark_theme(is_dark_theme: bool) {
    let mut s = load_settings();
    s.dark_theme = is_dark_theme;
    save_settings(&s);
}

pub fn load_saved_language() -> String {
    load_settings().language
}

pub fn save_language(combo_text: &str) {
    let mut s = load_settings();
    s.language = combo_text.to_string();
    save_settings(&s);
}

pub fn load_custom_rules() -> Vec<String> {
    if let Some(custom_file) = get_custom_text_config_file() {
        create_custom_text_file_if_needed();
        match fs::read_to_string(custom_file) {
            Ok(content) => {
                return content
                    .lines()
                    .filter_map(|s| {
                        let t = s.trim().to_string();
                        if t.is_empty() {
                            None
                        } else {
                            Some(t)
                        }
                    })
                    .collect();
            }
            Err(e) => {
                error!("Error while reading file with custom texts {e}");
            }
        }
    }
    vec![]
}

pub fn save_custom_rules(rules: &[String]) {
    if let Some(custom_file) = get_custom_text_config_file() {
        create_custom_text_file_if_needed();
        let joined = rules.join("\n");
        if let Err(e) = fs::write(custom_file, joined) {
            error!("Failed to save custom texts: {e}");
        }
    }
}

pub fn load_rules() -> Vec<MultipleRules> {
    if let Some(custom_file) = get_rules_config_file() {
        create_rules_file_if_needed();

        let Ok(file_handler) = fs::File::open(custom_file) else {
            return vec![];
        };
        let reader = BufReader::new(file_handler);
        match serde_json::from_reader(reader) {
            Ok(t) => return t,
            Err(e) => {
                error!("Failed to load rules, reason {e}");
                return vec![];
            }
        }
    }
    vec![]
}

pub fn save_rules_to_file(rules: &[MultipleRules]) {
    if let Some(custom_file) = get_rules_config_file() {
        create_rules_file_if_needed();

        let serialized = match serde_json::to_string_pretty(rules) {
            Ok(s) => s,
            Err(e) => {
                error!("Failed to serialize rules, reason {e}");
                return;
            }
        };
        if let Err(e) = fs::write(custom_file, serialized) {
            error!("Failed to save rules, reason {e}");
        }
    }
}

pub fn create_custom_text_file_if_needed() {
    if let Some(custom_file) = get_custom_text_config_file() {
        if !Path::new(&custom_file).is_file() {
            if let Some(parent) = Path::new(&custom_file).parent() {
                let _ = fs::create_dir_all(parent);
            }
            if let Err(e) = fs::write(&custom_file, BASIC_CUSTOM_COMMANDS) {
                error!("Failed to create file, reason {e}");
            }
        }
    }
}

pub fn create_rules_file_if_needed() {
    if let Some(custom_file) = get_rules_config_file() {
        if !Path::new(&custom_file).is_file() {
            if let Some(parent) = Path::new(&custom_file).parent() {
                let _ = fs::create_dir_all(parent);
            }
            if let Err(e) = fs::write(&custom_file, BASIC_RULE_CONTENT) {
                error!("Failed to create file, reason {e}");
            }
        }
    }
}
