mod art;

use colored::Colorize;
use indexmap::IndexMap;
use pad::PadStr;
use regex::Regex;
use std::io::{stdout, Write};
use sysinfo::{System, Users};

fn get_os() -> String {
    let mut os_string = String::new();

    if let Some(os_name) = System::name() {
        os_string.push_str(os_name.as_str());
    }

    if let Some(os_kernel) = System::kernel_version() {
        os_string.push(' ');
        os_string.push_str(os_kernel.as_str());
    }

    if let Some(os_version) = System::os_version() {
        os_string.push(' ');
        os_string.push_str(os_version.as_str());
    }

    os_string.to_string()
}

fn get_user(sys: &System) -> String {
    let users = Users::new_with_refreshed_list();
    let user = users.get_user_by_id(
        sys.process(sysinfo::get_current_pid().expect("Unable to get PID of current process"))
            .expect("Unable to get current process")
            .user_id()
            .expect("Unable to get owner of current process"),
    );

    if let Some(user) = user {
        return user.name().to_string();
    }

    "".to_string()
}

fn get_host() -> String {
    if let Some(host_name) = System::host_name() {
        return host_name;
    }

    "".to_string()
}

fn get_device_name() -> String {
    if let Some(device_name) = System::host_name() {
        return device_name;
    }

    "".to_string()
}

fn get_kernel() -> String {
    if let Some(os_kernel) = System::kernel_version() {
        return os_kernel;
    }

    "".to_string()
}

fn get_uptime() -> String {
    let uptime = System::uptime();

    let days = uptime / 86400;
    let hours = (uptime % 86400) / 3600;
    let minutes = (uptime % 3600) / 60;
    let seconds = uptime % 60;

    if days > 0 {
        return format!(
            "{} days {} hours {} minutes {} seconds",
            days, hours, minutes, seconds
        );
    }

    if hours > 0 {
        return format!("{} hours {} minutes {} seconds", hours, minutes, seconds);
    }

    if minutes > 0 {
        return format!("{} minutes {} seconds", minutes, seconds);
    }

    format!("{}s", seconds)
}

//fn get_packages() -> String {
//   "".to_string()
//}

fn get_cpu(sys: &System) -> Vec<&str> {
    let mut unique_cpu_names = vec![];

    for cpu in sys.cpus() {
        if !unique_cpu_names.contains(&cpu.brand()) {
            unique_cpu_names.push(cpu.brand());
        }
    }

    unique_cpu_names
}

fn get_memory(sys: &System) -> String {
    let used = sys.used_memory() / 1048576;
    let total = sys.total_memory() / 1048576;
    format!("{}MiB / {}MiB", used, total)
}

fn get_shell(sys: &System) -> String {
    let parent_process = sys
        .process(
            sys.process(sysinfo::get_current_pid().expect("Unable to get PID of current process"))
                .expect("Unable to get current process")
                .parent()
                .expect("Unable to get parent process"),
        )
        .expect("Unable to get parent process");

    parent_process.name().to_string()
}

fn get_colors() -> String {
    format!(
        "{}{}{}{}{}{}{}{}\n{}{}{}{}{}{}{}{}",
        "   ".on_black(),
        "   ".on_red(),
        "   ".on_green(),
        "   ".on_yellow(),
        "   ".on_blue(),
        "   ".on_magenta(),
        "   ".on_cyan(),
        "   ".on_white(),
        "   ".on_bright_black(),
        "   ".on_bright_red(),
        "   ".on_bright_green(),
        "   ".on_bright_yellow(),
        "   ".on_bright_blue(),
        "   ".on_bright_magenta(),
        "   ".on_bright_cyan(),
        "   ".on_bright_white(),
    )
}

fn main() {
    let mut sys = System::new_all();
    let mut lock = stdout().lock();
    let mut system_info = String::new();
    let image = art::get_image();
    let regexp = Regex::new(r"\x1b\[[0-9;]*m").unwrap();

    let mut primary_theme_color_string = "\x1b[32m";
    let mut secondary_theme_color_string = "\x1b[33m";

    let mut occourances = IndexMap::new();

    regexp
        .find_iter(&image)
        .for_each(|m| {
            let c = m.as_str();

            if occourances.contains_key(c) {
                occourances.insert(c, occourances.get(c).unwrap() + 1);
            } else {
                occourances.insert(c, 1);
            }
        });

    occourances.sort_by(|_, a, _, b| a.cmp(b));

    if let Some(first) = occourances.first() {
        if *first.0 != "\x1b[0m" {
            primary_theme_color_string = first.0;
        }

        if let Some(second) = occourances.get_index(1) {
            if *second.0 != "\x1b[0m" {
                secondary_theme_color_string = second.0;
            } else {
                secondary_theme_color_string = first.0;
            }
        }
    }

    sys.refresh_all();

    let user_string = get_user(&sys);
    let host_string = get_host();

    let name_string = format!(
        "{}{}\x1b[0m@{}{}\x1b[0m",
        primary_theme_color_string,
        user_string.as_str(),
        primary_theme_color_string,
        host_string.as_str()
    );

    let line = "─".repeat(user_string.len() + host_string.len() + 1);

    let header = format!("{}\n{}\n", name_string, line.as_str(),);

    system_info.push_str(&header);

    let mut info_map = IndexMap::new();
    info_map.insert("OS".to_string(), get_os());
    info_map.insert("Host".to_string(), get_device_name());
    info_map.insert("Kernel".to_string(), get_kernel());
    info_map.insert("Uptime".to_string(), get_uptime());
    info_map.insert("Shell".to_string(), get_shell(&sys));

    let cpus = get_cpu(&sys);

    if cpus.len() == 1 {
        info_map.insert("CPU".to_string(), cpus[0].to_string());
    } else {
        for (i, cpu) in cpus.iter().enumerate() {
            info_map.insert(format!("CPU {} ", i + 1), cpu.to_string());
        }
    }

    info_map.insert("Memory".to_string(), get_memory(&sys));

    for (info_key, info_value) in info_map.iter() {
        if info_value.is_empty() {
            continue;
        }

        system_info.push_str(&format!(
            "{}{}\x1b[0m: {}\n",
            secondary_theme_color_string, &info_key, info_value
        ));
    }

    system_info.push('\n');
    system_info.push_str(&get_colors());

    let mut image_lines: Vec<String> = image.lines().map(|line| line.to_string()).collect();
    let mut info_lines: Vec<&str> = system_info.lines().collect();
    let mut max_image_width = 0;

    image_lines.iter().for_each(|line| {
        let line_stripped = regexp.replace_all(&line.clone(), "").trim().to_string();

        if line_stripped.len() > max_image_width {
            max_image_width = line_stripped.len()
        }
    });

    image_lines = image_lines
        .iter()
        .map(|line| {
            let line_stripped = regexp.replace_all(&line.clone(), "").to_string();
            let line_diff = line.len() - line_stripped.len();

            if line_stripped.len() < max_image_width {
                return line.pad(
                    max_image_width + line_diff,
                    ' ',
                    pad::Alignment::Left,
                    false,
                );
            }

            line.to_string()
        })
        .collect();

    match image_lines.len() {
        i if i < info_lines.len() => {
            let line_diff = info_lines.len() - image_lines.len();
            let mut image_width = 0;

            if let Some(last_line) = image_lines.last() {
                image_width = last_line.len();
            }

            for _ in 0..(line_diff) {
                image_lines.push(" ".repeat(image_width));
            }
        },
        i if i > info_lines.len() => {
            let line_diff = image_lines.len() - info_lines.len();

            info_lines.resize(info_lines.len() + line_diff, "");
        },
        _ => {},
    }

    for (iter, image_line) in image_lines.iter().enumerate() {
        let info_line = info_lines[iter];
        writeln!(lock, "{}   {}", image_line, info_line.trim()).unwrap();
    }
}
