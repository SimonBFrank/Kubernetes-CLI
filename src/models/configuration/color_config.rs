use serde::Deserialize;

use crate::models::configuration::file::File;

#[derive(Debug, Deserialize, Clone)]
pub struct ColorConfig {
    pub color: String,
    pub file: File,
}

impl ColorConfig {
    #[allow(dead_code)]
    pub fn get_file_path(&self) -> &String {
        return &self.file.file_path;
    }
}
