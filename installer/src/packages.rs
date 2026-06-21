use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Arch {
    pub pacman: Vec<String>,
    pub aur: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Fedora {
    pub dnf: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Brew {
    pub formula: Vec<String>,
    pub casks: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MacOS {
    pub brew: Brew,
    pub install_xcode_command_line_tools: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ToolPackage {
    pub check: String,
    pub package: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ToolPackages {
    pub packages: Vec<ToolPackage>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CargoPackages {
    pub packages: Vec<ToolPackage>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UvPackages {
    pub install_uv: bool,
    pub packages: Vec<ToolPackage>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Fish {
    pub plugins: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Node {
    pub install_default_lts: bool,
    pub install_bun: bool,
    pub install_pnpm: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Manual {
    pub reminders: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Packages {
    pub arch: Arch,
    pub fedora: Fedora,
    pub macos: MacOS,
    pub cargo: CargoPackages,
    pub go: ToolPackages,
    pub npm: ToolPackages,
    pub uv: UvPackages,
    pub fish: Fish,
    pub node: Node,
    pub manual: Manual,
}

impl Packages {
    pub fn load() -> anyhow::Result<Self> {
        let file = std::fs::read_to_string("packages.toml")?;
        let packages: Packages = toml::from_str(&file)?;

        Ok(packages)
    }
}

#[cfg(test)]
mod tests {
    use crate::packages::Packages;

    #[test]
    fn packages_toml_matches_schema() {
        toml::from_str::<Packages>(include_str!("../packages.toml")).unwrap();
    }
}
