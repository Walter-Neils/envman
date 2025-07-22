use anyhow::Result;
use std::collections::HashMap;

// Executes a command with the specified executable, arguments, and environment variables.
// The environment is absolute, which means the executable's environment is strictly defined by
// the provided environment map, and any existing environment variables are cleared before execution.
pub fn execute(
    executable: &str,
    args: &[String],
    environment: &HashMap<String, String>,
) -> Result<()> {
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
