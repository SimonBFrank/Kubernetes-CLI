use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct File {
    pub file_path: String,
}
