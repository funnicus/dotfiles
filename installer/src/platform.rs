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
    Fedora,
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

pub fn is_fedora() -> bool {
    matches!(current(), Platform::Linux(LinuxDistro::Fedora))
}

fn linux_platform() -> Platform {
    let Ok(os_release) = std::fs::read_to_string("/etc/os-release") else {
        return Platform::Unsupported;
    };

    linux_platform_from_os_release(&os_release)
}

fn linux_platform_from_os_release(os_release: &str) -> Platform {
    let id = os_release_value(os_release, "ID");
    let id_like = os_release_value(os_release, "ID_LIKE");

    let distro = match (id.as_deref(), id_like.as_deref()) {
        (Some("cachyos"), _) => LinuxDistro::CachyOS,
        (Some("arch"), _) => LinuxDistro::Arch,
        (Some("fedora"), _) => LinuxDistro::Fedora,
        (_, Some(value)) if value.split_whitespace().any(|part| part == "arch") => {
            LinuxDistro::Arch
        }
        (_, Some(value)) if value.split_whitespace().any(|part| part == "fedora") => {
            LinuxDistro::Fedora
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

#[cfg(test)]
mod tests {
    use super::{LinuxDistro, Platform, linux_platform_from_os_release};

    #[test]
    fn detects_fedora() {
        assert_eq!(
            linux_platform_from_os_release("ID=fedora\nID_LIKE=\"fedora\"\n"),
            Platform::Linux(LinuxDistro::Fedora)
        );
    }
}
