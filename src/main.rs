#![feature(iter_intersperse)]
use crate::{
    apply_env::apply_env, command_line_info::get_command_line_info, config::load_config_file,
    execute::execute, help::show_help,
};
use anyhow::Result;

mod apply_env;
mod command_line_info;
mod config;
mod execute;
mod help;

fn main() -> Result<()> {
    {
        // TODO: Move this logic somewhere else, pretty it up
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
            return Err(anyhow::anyhow!(
                "Attempted to reference environment '{}', but no such environment exists in the configuration.",
                target
            ));
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
