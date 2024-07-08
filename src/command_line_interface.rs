use indexmap::IndexMap;
use std::{env, process::exit, vec};

use super::image_handler::ImageHandler;

pub struct Argument {
    pub name: String,
    pub short_name: String,
    pub description: String,
    pub group: String,
    pub value: Option<String>,
    pub passed: bool,
}

pub struct CommandLineInterace {
    args: Vec<Argument>,
    passed_args: Vec<String>,
    read_args: bool,
}

impl CommandLineInterace {
    pub fn new() -> Self {
        let args = vec![
            ("help", "h", "Prints this help page", "General"),
            ("version", "v", "Shows this help page", "General"),
            ("no-config", "", "Disables using the configuration file found under HOME/.rustfetch/config.toml", "General"),
            ("hide-cpu", "", "Hides the cpu information", "Hides"),
            ("hide-memory", "", "Hides the memory information", "Hides"),
            ("hide-uptime", "", "Hides the uptime information", "Hides"),
            ("hide-os", "", "Hides the os information", "Hides"),
            ("hide-host", "", "Hides the host information", "Hides"),
            ("hide-kernel", "", "Hides the kernel information", "Hides"),
            ("hide-shell", "", "Hides the shell information", "Hides"),
            ("hide-colors", "", "Hides the color display", "Hides"),
        ];

        let mut arg_structs: Vec<Argument> = vec![];

        for (name, short_name, description, group) in args {
            arg_structs.push(Argument {
                name: name.to_string(),
                short_name: short_name.to_string(),
                description: description.to_string(),
                group: group.to_string(),
                value: None,
                passed: false,
            });
        }

        Self {
            args: arg_structs,
            passed_args: Vec::new(),
            read_args: false,
        }
    }

    pub fn get_args(&mut self) -> &Vec<Argument> {
        if !self.read_args {
            self.parse_args();
        }

        &self.args
    }

    pub fn check_for_help(&mut self) {
        if !self.read_args {
            self.parse_args();
        }

        let help_arg = self.args.iter().find(|a| a.name == "help").unwrap();

        if help_arg.passed {
            self.send_help();
            exit(0);
        }
    }

    fn parse_args(&mut self) {
        self.read_args = true;
        self.passed_args = env::args().collect::<Vec<String>>();

        for (iter, arg) in self.passed_args[1..].iter().enumerate() {
            if arg.is_empty() {
                continue;
            }

            let arg_struct = self.args.iter_mut().find(|a| {
                arg == format!("-{}", a.short_name).as_str()
                    || arg == format!("--{}", a.name).as_str()
            }).unwrap_or_else(|| {
                eprintln!("Could not find argument {}. View all arguments with --help", arg);
                exit(1);
            });

            arg_struct.passed = true;

            if let Some(next_arg) = self.passed_args.get(iter + 1) {
                if !next_arg.starts_with("-") {
                    arg_struct.value = Some(next_arg.to_string());
                }
            }
        }
    }

    fn send_help(&self) {
        let mut out = String::new();
        let image_handler = ImageHandler::new(super::art::rust_fetch());
        let rust_fetch_information = "\x1b[32mRustFetch@RustFetch\x1b[0m
-------------------

A command line high-performence fetcher for all platforms.
Made by Surge, Hazel and the RustFetch contributors.
https://github.com/RustFetch/RustFetch

NOTE: Currently in active development, features may be missing or broken.
Submit bug reports and feature requests using issues in the repository above."
            .to_string();

        let lines = image_handler.interpolate_image(rust_fetch_information);

        for image_line in lines {
            out.push_str(format!("{}\n", image_line).as_str());
        }

        out.push_str("\n");

        let mut arg_map: IndexMap<&String, Vec<&Argument>> = IndexMap::new();

        for arg in self.args.iter() {
            arg_map.entry(&arg.group).or_insert_with(Vec::new).push(arg);
        }

        for (group, group_args) in arg_map {
            out.push_str(format!("{}\n{}\n", &group, "-".repeat(group.len())).as_str());

            for arg in group_args {
                if !arg.short_name.is_empty() {
                    out.push_str(
                        format!(
                            "    \x1b[32m--{} / -{}\x1b[0m - \x1b[33m{}\x1b[0m\n",
                            arg.name, arg.short_name, arg.description
                        )
                        .as_str(),
                    )
                } else {
                    out.push_str(
                        format!(
                            "    \x1b[32m--{}\x1b[0m - \x1b[33m{}\x1b[0m\n",
                            arg.name, arg.description
                        )
                        .as_str(),
                    )
                }
            }
        }

        print!("{}", out);
    }
}
