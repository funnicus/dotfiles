use std::{path::Path, process::Command};

use crate::packages::Packages;

pub struct AppleSiliconInstaller {
    packages: Packages,
    commands: Vec<Vec<String>>,
}

impl AppleSiliconInstaller {
    pub fn new(packages: Packages) -> Self {
        Self {
            packages,
            commands: Vec::new(),
        }
    }

    pub fn get_commands(&self) -> &[Vec<String>] {
        &self.commands
    }

    pub fn install(&mut self) -> anyhow::Result<()> {
        if self.packages.macos.install_xcode_command_line_tools && !developer_dir_exists()? {
            self.commands
                .push(vec!["xcode-select".into(), "--install".into()]);
        }

        // Formula
        let mut formulae = self.packages.macos.brew.formula.clone();
        if !formulae.is_empty() {
            let mut args = vec!["brew".into(), "install".into()];
            args.append(&mut formulae);
            self.commands.push(args);
        } else {
            println!("No MacOS Brew formula to install!");
        }

        // Casks
        if should_install_casks() {
            let casks = &self.packages.macos.brew.casks;
            if !casks.is_empty() {
                let mut args = vec!["brew".into(), "install".into(), "--cask".into()];
                args.append(&mut casks.clone());
                self.commands.push(args);
            } else {
                println!("No MacOS Brew casks to install!");
            }
        }

        Ok(())
    }
}

fn developer_dir_exists() -> anyhow::Result<bool> {
    let output = Command::new("xcode-select").arg("-p").output()?;
    if !output.status.success() {
        return Ok(false);
    }

    let developer_dir = String::from_utf8_lossy(&output.stdout);
    Ok(Path::new(developer_dir.trim()).exists())
}

fn should_install_casks() -> bool {
    std::env::var("INSTALL_CASKS")
        .map(|value| matches!(value.as_str(), "1" | "true" | "TRUE" | "yes" | "YES"))
        .unwrap_or(true)
}
