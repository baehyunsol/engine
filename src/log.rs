use yaml_rust::Yaml;
use crate::yaml_hash;

#[derive(Clone)]
pub struct Log {
    pub file_from: String,
    pub file_to: String,
    pub metadata: Yaml
}

impl Log {
    pub fn new(file_from: &str, file_to: &str, metadata: Option<Yaml>) -> Self {
        let metadata = match metadata {
            Some(y) => match y.as_hash() {
                None => yaml_hash::new(),
                _ => y
            },
            _ => yaml_hash::new(),
        };

        Log {
            file_from: file_from.to_string(),
            file_to: file_to.to_string(),
            metadata
        }
    }
}
