use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct File {
    pub file_path: String,
}
