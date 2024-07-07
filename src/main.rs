mod art;
mod image_handler;
mod system_information;

use image_handler::ImageHandler;
use indexmap::IndexMap;
use std::env;
use std::io::{stdout, Write};
use system_information::SystemInformation;

fn main() {
    let image = art::get_image();
    let mut image_handler = ImageHandler::new(image);
    let sys = SystemInformation::new();
    let mut lock = stdout().lock();
    let mut system_info = String::new();

    let args: Vec<String> = env::args().collect();
    let hide_cpu = args.contains(&"--hide-cpu".to_string());
    let hide_memory = args.contains(&"--hide-memory".to_string());
    let hide_uptime = args.contains(&"--hide-uptime".to_string());
    let hide_os = args.contains(&"--hide-os".to_string());
    let hide_host = args.contains(&"--hide-host".to_string());
    let hide_kernel = args.contains(&"--hide-kernel".to_string());
    let hide_shell = args.contains(&"--hide-shell".to_string());

    let user_string = sys.get_user();
    let host_string = sys.get_host();

    let primary_color = image_handler.get_primary_theme_color().to_string();
    let secondary_color = image_handler.get_secondary_theme_color().to_string();

    let name_string = format!(
        "{}{}\x1b[0m@{}{}\x1b[0m",
        primary_color,
        user_string.as_str(),
        secondary_color,
        host_string.as_str()
    );

    let line = "─".repeat(user_string.len() + host_string.len() + 1);

    let header = format!("{}\n{}\n", name_string, line.as_str(),);

    system_info.push_str(&header);

    let mut info_map = IndexMap::new();

    if !hide_os {
        info_map.insert("OS".to_string(), sys.get_os());
    }
    if !hide_host {
        info_map.insert("Host".to_string(), sys.get_device_name());
    }
    if !hide_kernel {
        info_map.insert("Kernel".to_string(), sys.get_kernel());
    }
    if !hide_uptime {
        info_map.insert("Uptime".to_string(), sys.get_uptime());
    }
    if !hide_shell {
        info_map.insert("Shell".to_string(), sys.get_shell());
    }

    if !hide_cpu {
        let cpus = sys.get_cpu();

        if cpus.len() == 1 {
            info_map.insert("CPU".to_string(), cpus[0].to_string());
        } else {
            for (i, cpu) in cpus.iter().enumerate() {
                info_map.insert(format!("CPU {} ", i + 1), cpu.to_string());
            }
        }
    }

    if !hide_memory {
        info_map.insert("Memory".to_string(), sys.get_memory());
    }

    for (info_key, info_value) in info_map.iter() {
        if info_value.is_empty() {
            continue;
        }

        system_info.push_str(&format!(
            "{}{}\x1b[0m: {}\n",
            secondary_color, &info_key, info_value
        ));
    }

    system_info.push('\n');
    system_info.push_str(&sys.get_colors());

    let lines = image_handler.interpolate_image(system_info);

    for image_line in lines {
        writeln!(lock, "{}", image_line).unwrap();
    }
}
