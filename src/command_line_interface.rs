use std::env;
use indexmap::IndexMap;

pub struct CommandLineInterace {
    args: IndexMap<String, String>,
}

impl CommandLineInterace {
    pub fn new() -> Self {
        let args = IndexMap::from([
            ("--help".to_string(), "Prints help information".to_string()),
            ("--no-config".to_string(), "Disables the use of a configuration file".to_string()),
            ("--hide-cpu".to_string(), "Hides the CPU information".to_string()),
            ("--hide-memory".to_string(), "Hides the memory information".to_string()),
            ("--hide-uptime".to_string(), "Hides the uptime information".to_string()),
            ("--hide-os".to_string(), "Hides the OS information".to_string()),
            ("--hide-host".to_string(), "Hides the host information".to_string()),
            ("--hide-kernel".to_string(), "Hides the kernel information".to_string()),
            ("--hide-shell".to_string(), "Hides the shell information".to_string()),
        ]);

        Self {args}
    }

    pub fn parse_args(&self) -> Vec<String> {
        env::args().collect()
    }

    pub fn check_for_help(&self, args: Vec<String>) {
        if args.contains(&"--help".to_string()) {
            Self::send_help();
        }
    }

    fn send_help() {

    }
}
