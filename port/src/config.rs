use std::fs;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use directories_next::ProjectDirs;

use crate::rule::rules::MultipleRules;

pub const CUSTOM_TEXT_FILE_NAME: &str = "custom_text_names.txt";
pub const RULES_FILE_NAME: &str = "rules_settings.json";
pub const LANGUAGE_FILE_NAME: &str = "language.txt";
pub const DARK_THEME_FILE_NAME: &str = "dark_theme.txt";

const BASIC_CUSTOM_COMMANDS: &str = r"FILE_$(N).$(EXT)
FILE_$(K).$(EXT)
$(PARENT) $(N).$(EXT)
$(PARENT) $(K).$(EXT)
";

const BASIC_RULE_CONTENT: &str = r"[]";

pub fn get_dark_theme_config_path() -> Option<PathBuf> {
    ProjectDirs::from("pl", "Qarmin", "SzyszkaSlint").map(|p| PathBuf::from(p.config_dir()).join(DARK_THEME_FILE_NAME))
}

pub fn load_dark_theme_config_or_create() -> bool {
    if let Some(path) = get_dark_theme_config_path() {
        if !Path::new(&path).is_file() {
            if let Some(parent) = path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            let _ = fs::write(&path, "true");
        }
        if let Ok(thing) = fs::read_to_string(&path) {
            return thing.trim().parse().unwrap_or(true);
        }
    }
    true
}

pub fn save_dark_theme(is_dark_theme: bool) {
    if let Some(path) = get_dark_theme_config_path() {
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let _ = fs::write(path, is_dark_theme.to_string());
    }
}

pub fn get_language_config_path() -> Option<PathBuf> {
    ProjectDirs::from("pl", "Qarmin", "SzyszkaSlint").map(|p| PathBuf::from(p.config_dir()).join(LANGUAGE_FILE_NAME))
}

pub fn get_config_path() -> Option<PathBuf> {
    ProjectDirs::from("pl", "Qarmin", "SzyszkaSlint").map(|p| PathBuf::from(p.config_dir()))
}

pub fn get_custom_text_config_file() -> Option<PathBuf> {
    get_config_path().map(|p| p.join(CUSTOM_TEXT_FILE_NAME))
}

pub fn get_rules_config_file() -> Option<PathBuf> {
    get_config_path().map(|p| p.join(RULES_FILE_NAME))
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
                        if t.is_empty() { None } else { Some(t) }
                    })
                    .collect();
            }
            Err(e) => {
                eprintln!("Error while reading file with custom texts {e}");
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
            eprintln!("Failed to save custom texts: {e}");
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
                eprintln!("Failed to load rules, reason {e}");
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
                eprintln!("Failed to serialize rules, reason {e}");
                return;
            }
        };
        if let Err(e) = fs::write(custom_file, serialized) {
            eprintln!("Failed to save rules, reason {e}");
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
                eprintln!("Failed to create file, reason {e}");
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
                eprintln!("Failed to create file, reason {e}");
            }
        }
    }
}
