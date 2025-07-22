pub struct CommandLineInfo {
    pub target_environments: Vec<String>,
    pub executable: String,
    pub arguments: Vec<String>,
}

pub fn get_command_line_info() -> CommandLineInfo {
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
