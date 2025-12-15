use std::fs::File;
use std::io::Read;
use toml::Value;
use sysinfo::System;

pub enum PackageManager {
    Apt, Dnf, Pacman, Zypper, Brew, Flatpak, Snap, Scoop, 
}

pub fn detect_managers() -> Vec<PackageManager> {
    let mut managers = Vec::new();

    // Parse /etc/os-release for distro
    if let Ok(mut file) = File::open("/etc/os-release") {
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let os_release: Value = contents.parse::<Value>().unwrap();
        let id = os_release["ID"].as_str().unwrap_or("unknown");

        match id {
            "ubuntu" | "debian" => managers.push(PackageManager::Apt),
            "fedora" | "rhel" => managers.push(PackageManager::Dnf),
            "arch" => managers.push(PackageManager::Pacman),
            "opensuse" => managers.push(PackageManager::Zypper),
            _ => {},
        }
    }

    // Check for user-installed managers
    let sys = System::new_all();
    if sys.processes_by_name("brew").next().is_some() || which::which("brew").is_ok() {
        managers.push(PackageManager::Brew);
    }
    if which::which("flatpak").is_ok() { managers.push(PackageManager::Flatpak); }
    if which::which("snap").is_ok() { managers.push(PackageManager::Snap); }

    managers
}
