pub fn show_help() {
    println!(
        r#"
envman - Environment Manager

Purpose:
  envman loads environment definitions from ~/.config/envman/config.yaml,
  applies those definitions to its working environment, and then executes
  a specified program within this newly configured environment.

Usage:
  envman [ENVIRONMENTS, SPACE DELIMITED] [--] <executable> [<args>...]

Arguments:
  [ENVIRONMENTS, SPACE DELIMITED]
    One or more environment names defined in your config.yaml, separated by spaces.
    These environments will be applied in the order they are listed.

  --
    A mandatory separator between environment names and the executable.
    Everything after '--' is treated as the command to execute.

  <executable>
    The path to the program you wish to run within the managed environment.

  [<args>...]
    Any arguments to pass to the executable.

Configuration:
  Environment definitions are loaded from: ~/.config/envman/config.yaml

Examples:
  # Run 'my_script.sh' with the 'dev' environment applied
  envman dev -- my_script.sh

  # Run 'python' with 'test' and 'debug' environments, passing arguments to python
  envman test debug -- python my_app.py --verbose

  # Run 'ls -la' without any specific environments (still uses envman's execution context)
  envman -- ls -la

  # Display this help message
  envman --help
"#
    );
}
