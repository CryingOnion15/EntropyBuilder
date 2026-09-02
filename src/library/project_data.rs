use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct EntropyProject {
    pub project_name: String,
}

impl EntropyProject {
    pub fn new() -> EntropyProject {
        Self {
            project_name: String::new(),
        }
    }

    pub fn update_fields(&mut self, updates: Value) {
        let mut current = serde_json::to_value(&self).unwrap();

        if let (Value::Object(current), Value::Object(updates)) = (&mut current, updates) {
            current.extend(updates);
        }

        *self = serde_json::from_value(current).unwrap();
    }

    pub fn load_from_build_file(&mut self, ebuild_path: &String) {
        let build_str = fs::read_to_string(ebuild_path).expect("Unable to read ebuild file.");

        *self = Self::deserialize_ebuild(build_str);
    }

    pub fn serialize(&self) -> String {
        return serde_json::to_string_pretty(self).unwrap();
    }

    pub fn deserialize_ebuild(build_str: String) -> EntropyProject {
        return serde_json::from_str(&build_str).unwrap();
    }
}
