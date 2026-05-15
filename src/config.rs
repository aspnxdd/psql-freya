use crate::models::ConnectionConfig;
use dirs::home_dir;
use std::fs;
use std::path::PathBuf;

pub fn config_path() -> PathBuf {
    let mut path = home_dir().expect("Could not find home directory");
    path.push(".psql-freya");
    fs::create_dir_all(&path).ok();
    path.push("config.json");
    path
}

pub fn load_config() -> Vec<ConnectionConfig> {
    let path = config_path();
    if path.exists() {
        let contents = fs::read_to_string(path).unwrap_or_default();
        serde_json::from_str(&contents).unwrap_or_default()
    } else {
        vec![]
    }
}

pub fn save_config(connections: &[ConnectionConfig]) {
    let path = config_path();
    if let Ok(json) = serde_json::to_string_pretty(connections) {
        fs::write(path, json).ok();
    }
}
