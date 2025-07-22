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

Examples:
  # Run 'my_script.sh' with the 'dev' environment applied
  envman dev -- my_script.sh

  # Run 'python' with 'test' and 'debug' environments, passing arguments to python
  envman test debug -- python my_app.py --verbose

  # Run 'ls -la' without any specific environments (still uses envman's execution context)
  envman -- ls -la

  # Display this help message
  envman --help


Configuration:
  Environment definitions are loaded from: ~/.config/envman/config.yaml
An example configuration file is shown below:

```yaml
environments:
  framegen:
    variables:
      LSFG_LEGACY: !Override 1
      PROTON_USE_DXVK: !Default 1
      ENABLE_LSFG: !Override 1
      LSFG_MULTIPLIER: !Default 3
      LSFG_DLL_PATH: !Default "/mnt/bulk-array/SteamLibrary/steamapps/common/Lossless Scaling/Lossless.dll"
  "proton-common":
    variables:
      FSR4_UPGRADE: !Default 1
      PROTON_ENABLE_HDR: !Default 1
      PROTON_USE_NTSYNC: !Default 1
      PROTON_ENABLE_WAYLAND: !Default 0
  "proton-wayland":
    variables:
      PROTON_ENABLE_WAYLAND: !Override 1
```

There are several environment variable options.
    Default(value)
        The most common use case, either respects the existing value of the environment variable if set,
        or uses the specified default value if the variable is not already set in the environment.
    Clear
      Clears the environment variable, removing it from the environment.
    Override(value)
        Overrides the environment variable with the specified value, replacing any existing value.
    StringList(StringList)
        Manages variables which represent some form of list, such as PATH or LD_LIBRARY_PATH.
        The StringList structure allows for additional items to be added, discarded items to be removed,
        mode of insertion (Append, Prepend, Replace) to be specified, delimiter to be set, and additional behaviors
        such as removing duplicates to be applied.
    Required
        Represents a variable that MUST have a value, but may not be set in the configuration.

Full StringList structure:
root: !StringList
    additional_items: Vec<String> # Items which will be added to the list
    discarded_items: Option<Vec<String>> # Items which will be removed from the list
    delimiter: String # The delimiter used to separate items in the list. PATH uses ':', LD_LIBRARY_PATH uses ':', etc.
    insert_mode: StringListMode # The mode of insertion, either Append, Prepend, or Replace.
                                # Using replace will replace the entire list with the additional items, making discarded_items irrelevant.
    additional_behavior: Option<Vec<StringListBehavior>> # Additional behaviors to apply to the list, such as removing duplicates.
                                                         # Currently, the only supported behavior is RemoveDuplicates.
                                                         # Additional behaviors execute AFTER all other operations 
