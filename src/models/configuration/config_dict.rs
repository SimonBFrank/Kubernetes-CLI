use serde::Deserialize;
use std::ops::Index;

use crate::models::configuration::env_config::EnvConfig;

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct Config {
    config: Vec<EnvConfig>,
}

impl Config {
    pub fn new(path: &str) -> Config {
        let file_contents = Config::read_file_to_str(path);
        Config::str_to_struct(&file_contents)
    }

    fn read_file_to_str(path: &str) -> String {
        let contents: String = match std::fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => {
                println!("File does not exist or can't be read, error: {e}");
                std::process::exit(1);
            }
        };
        return contents;
    }

    fn str_to_struct(s: &str) -> Config {
        let json: Config = match serde_json::from_str(s) {
            Ok(val) => val,
            Err(e) => {
                println!("The file's format is incorrect, error: {e}");
                std::process::exit(1);
            }
        };
        return json;
    }

    #[allow(dead_code)]
    pub fn get_keys(&self) -> Vec<String> {
        let mut keys = Vec::new();
        for env_config in &self.config {
            keys.push(env_config.env.clone())
        }
        return keys;
    }
}

impl Index<String> for Config {
    type Output = EnvConfig;
    fn index(&self, index: String) -> &Self::Output {
        for env_config in &self.config {
            if env_config.env == index {
                return env_config;
            }
        }
        panic!("Key does not exist in object");
    }
}
