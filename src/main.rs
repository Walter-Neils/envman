#![feature(iter_intersperse)]
use std::collections::HashMap;

use crate::{
    config::{ConfigurationFile, Environment, StringList, load_config_file},
    help::show_help,
};
use anyhow::Result;

mod config;
mod help;

fn apply_env<'a>(environments: Vec<&'a Environment>) -> Result<HashMap<String, String>> {
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

struct CommandLineInfo {
    pub target_environments: Vec<String>,
    pub executable: String,
    pub arguments: Vec<String>,
}

fn get_command_line_info() -> CommandLineInfo {
    let mut result = CommandLineInfo {
        target_environments: Vec::new(),
        executable: String::new(),
        arguments: Vec::new(),
    };
    {
        let mut iter = std::env::args().into_iter();
        iter.next(); // Skip the first argument (the program name)
        for env_target in &mut iter {
            if env_target == "--" {
                break;
            } else {
                result.target_environments.push(env_target);
            }
        }
        result.executable = iter.next().unwrap_or_else(|| String::from("envman"));
        result.arguments = iter.collect();
    }

    result
}

fn demo() {
    let mut config = ConfigurationFile {
        environments: HashMap::new(),
    };
    let mut env = Environment {
        variables: HashMap::new(),
    };

    env.variables.insert(
        "EXAMPLE_STRING".to_string(),
        config::EnvironmentVariable::String("example_value".to_string()),
    );

    env.variables.insert(
        "EXAMPLE_LIST".to_string(),
        config::EnvironmentVariable::StringList(config::StringList {
            items: vec!["item1".to_string(), "item2".to_string()],
            delimiter: ':'.to_string(),
            mode: config::StringListMode::Append,
        }),
    );

    env.variables.insert(
        "MOZ_ENABLE_WAYLAND".to_string(),
        config::EnvironmentVariable::Clear,
    );

    env.variables.insert(
        "REQUIRED_VAR".to_string(),
        config::EnvironmentVariable::Required,
    );

    config.environments.insert("default".to_string(), env);

    println!("{}", serde_yaml::to_string(&config).unwrap());
}

fn execute(executable: &str, args: &[String], environment: &HashMap<String, String>) -> Result<()> {
    let mut command = std::process::Command::new(executable);
    command.args(args);
    command.env_clear();
    for (key, value) in environment {
        command.env(key, value);
    }
    let status = command.status()?;
    if !status.success() {
        return Err(anyhow::anyhow!(
            "Command '{}' failed with status: {}",
            executable,
            status
        ));
    }
    Ok(())
}

fn main() -> Result<()> {
    {
        let args = std::env::args().collect::<Vec<_>>();
        if args.len() > 1 && args[1] == "--help" {
            show_help();
            return Ok(());
        }
        if args.len() < 3 {
            show_help();
            return Err(anyhow::anyhow!(
                "Insufficient arguments provided. Please refer to the help message."
            ));
        }
    }
    let config = load_config_file()?;
    let command_line_info = get_command_line_info();

    let mut environments = Vec::new();
    for target in &command_line_info.target_environments {
        if let Some(env) = config.environments.get(target) {
            environments.push(env);
        } else {
            eprintln!(
                "Warning: Environment '{}' not found in configuration.",
                target
            );
        }
    }
    let environment = apply_env(environments)
        .map_err(|e| anyhow::anyhow!("Failed to apply environment variables: {}", e))?;
    if command_line_info.executable.is_empty() {
        eprintln!("No executable specified. Please provide an executable to run.");
        return Ok(());
    }
    execute(
        &command_line_info.executable,
        &command_line_info.arguments,
        &environment,
    )?;
    Ok(())
}
