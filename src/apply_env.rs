use anyhow::Result;
use std::collections::HashMap;

use crate::config::{self, Environment};

pub fn apply_env<'a>(environments: Vec<&'a Environment>) -> Result<HashMap<String, String>> {
    let mut env_map = HashMap::new();
    for var in std::env::vars() {
        env_map.insert(var.0, var.1);
    }
    for env in environments {
        for (variable_key, variable_value) in &env.variables {
            match variable_value {
                config::EnvironmentVariable::Clear => {
                    env_map.remove(variable_key);
                }
                config::EnvironmentVariable::String(value) => {
                    env_map.insert(variable_key.to_string(), value.clone());
                }
                config::EnvironmentVariable::StringList(list) => {
                    let mut result = String::new();
                    let mut values = Vec::new();
                    let current_value = env_map.get(variable_key).cloned().unwrap_or_default();
                    current_value.split(&list.delimiter).for_each(|s| {
                        if !s.is_empty() {
                            values.push(s.to_string());
                        }
                    });
                    match list.mode {
                        config::StringListMode::Append => {
                            values.extend(list.items.iter().cloned());
                        }
                        config::StringListMode::Prepend => {
                            let mut new_values = list.items.clone();
                            new_values.extend(values);
                            values = new_values;
                        }
                        config::StringListMode::Replace => {
                            values = list.items.clone();
                        }
                    }

                    values
                        .iter()
                        .intersperse_with(|| &list.delimiter)
                        .for_each(|s| result.push_str(s));
                    env_map.insert(variable_key.to_string(), result);
                }
                config::EnvironmentVariable::Required => {
                    if !env_map.contains_key(variable_key) {
                        return Err(anyhow::anyhow!(
                            "Required environment variable '{}' is not set.",
                            variable_key
                        ));
                    }
                }
                config::EnvironmentVariable::Default(default_value) => {
                    env_map
                        .entry(variable_key.to_string())
                        .or_insert(default_value.clone());
                }
            }
        }
    }
    Ok(env_map)
}

