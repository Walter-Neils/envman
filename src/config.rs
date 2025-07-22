use std::{collections::HashMap, io::Read};

use anyhow::anyhow;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum EnvironmentVariable {
    // Clears the environment variable
    Clear,
    // Overrides the environment variable with a new value, replacing any existing value
    Override(String),
    // Manages variables which represent some form of list, such as PATH or LD_LIBRARY_PATH
    StringList(StringList),
    // Represents a variable that MUST have a value, but may not be set in the configuration
    Required,
    // Represents a variable that has a default value, which will be used if not already set
    Default(String),
}

#[derive(Serialize, Deserialize)]
pub enum StringListMode {
    Append,
    Prepend,
    Replace,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub enum StringListBehavior {
    RemoveDuplicates,
}

#[derive(Serialize, Deserialize)]
pub struct StringList {
    // The additional items to add to the list
    pub additional_items: Vec<String>,
    // Values that should be discarded from the list
    pub discarded_items: Option<Vec<String>>,
    pub delimiter: String,
    pub insert_mode: StringListMode,
    pub additional_behavior: Option<Vec<StringListBehavior>>,
}

#[derive(Serialize, Deserialize)]
pub struct Environment {
    pub variables: HashMap<String, EnvironmentVariable>,
}

#[derive(Serialize, Deserialize)]
pub struct ConfigurationFile {
    pub environments: HashMap<String, Environment>,
}

pub fn load_config_file() -> anyhow::Result<ConfigurationFile> {
    let mut config_directory =
        dirs::config_dir().ok_or(anyhow!("Failed to fetch config directory"))?;
    config_directory.push("envman");
    config_directory.push("config.yaml");
    let mut config_file = std::fs::File::open(config_directory)?;
    let mut contents = String::new();
    config_file.read_to_string(&mut contents)?;

    serde_yaml::from_str(&contents)
        .map_err(|e| anyhow!("Failed to parse configuration file: {}", e))
        .and_then(|config: ConfigurationFile| Ok(config))
}
