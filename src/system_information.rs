use colored::Colorize;
use sysinfo::{System, Users};

pub struct SystemInformation {
    sys: System,
}

impl SystemInformation {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        Self { sys }
    }

    pub fn get_os(&self) -> String {
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

    pub fn get_user(&self) -> String {
        let users = Users::new_with_refreshed_list();
        let user = users.get_user_by_id(
            self.sys
                .process(sysinfo::get_current_pid().expect("Unable to get PID of current process"))
                .expect("Unable to get current process")
                .user_id()
                .expect("Unable to get owner of current process"),
        );

        if let Some(user) = user {
            return user.name().to_string();
        }

        "".to_string()
    }

    pub fn get_host(&self) -> String {
        if let Some(host_name) = System::host_name() {
            return host_name;
        }

        "".to_string()
    }

    pub fn get_device_name(&self) -> String {
        if let Some(device_name) = System::host_name() {
            return device_name;
        }

        "".to_string()
    }

    pub fn get_kernel(&self) -> String {
        if let Some(os_kernel) = System::kernel_version() {
            return os_kernel;
        }

        "".to_string()
    }

    pub fn get_uptime(&self) -> String {
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

    pub fn get_cpu(&self) -> Vec<&str> {
        let mut unique_cpu_names = vec![];

        for cpu in self.sys.cpus() {
            if !unique_cpu_names.contains(&cpu.brand()) {
                unique_cpu_names.push(cpu.brand());
            }
        }

        unique_cpu_names
    }

    pub fn get_memory(&self) -> String {
        let used = self.sys.used_memory() / 1048576;
        let total = self.sys.total_memory() / 1048576;
        format!("{}MiB / {}MiB", used, total)
    }

    pub fn get_shell(&self) -> String {
        let parent_process = self
            .sys
            .process(
                self.sys
                    .process(
                        sysinfo::get_current_pid().expect("Unable to get PID of current process"),
                    )
                    .expect("Unable to get current process")
                    .parent()
                    .expect("Unable to get parent process"),
            )
            .expect("Unable to get parent process");

        parent_process.name().to_string()
    }

    pub fn get_colors(&self) -> String {
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
}
