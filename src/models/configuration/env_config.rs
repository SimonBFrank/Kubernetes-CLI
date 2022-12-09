use serde::Deserialize;
use std::ops::Index;

use crate::models::configuration::color_config::ColorConfig;

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct EnvConfig {
    pub env: String,
    pub color_configs: Vec<ColorConfig>,
}

impl EnvConfig {
    #[allow(dead_code)]
    pub fn get_keys(&self) -> Vec<String> {
        let mut keys = Vec::new();
        for env_config in &self.color_configs {
            keys.push(env_config.color.clone())
        }
        return keys;
    }
}

impl Index<String> for EnvConfig {
    type Output = ColorConfig;
    fn index(&self, index: String) -> &Self::Output {
        for color_config in &self.color_configs {
            if color_config.color == index {
                return color_config;
            }
        }
        panic!("Key does not exist in object");
    }
}
