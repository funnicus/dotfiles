#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Platform {
    Linux(LinuxDistro),
    MacOS,
    Unsupported,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinuxDistro {
    Arch,
    CachyOS,
    Other { id: Option<String> },
}

pub fn current() -> Platform {
    match std::env::consts::OS {
        "macos" => Platform::MacOS,
        "linux" => linux_platform(),
        _ => Platform::Unsupported,
    }
}

pub fn is_arch_like() -> bool {
    matches!(
        current(),
        Platform::Linux(LinuxDistro::Arch | LinuxDistro::CachyOS)
    )
}

fn linux_platform() -> Platform {
    let Ok(os_release) = std::fs::read_to_string("/etc/os-release") else {
        return Platform::Unsupported;
    };

    let id = os_release_value(&os_release, "ID");
    let id_like = os_release_value(&os_release, "ID_LIKE");

    let distro = match (id.as_deref(), id_like.as_deref()) {
        (Some("cachyos"), _) => LinuxDistro::CachyOS,
        (Some("arch"), _) => LinuxDistro::Arch,
        (_, Some(value)) if value.split_whitespace().any(|part| part == "arch") => {
            LinuxDistro::Arch
        }
        _ => LinuxDistro::Other { id },
    };

    Platform::Linux(distro)
}

fn os_release_value(contents: &str, key: &str) -> Option<String> {
    let prefix = format!("{key}=");
    let value = contents
        .lines()
        .find_map(|line| line.strip_prefix(&prefix))?
        .trim();

    Some(value.trim_matches('"').to_string())
}
