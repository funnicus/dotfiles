use console::style;
use indicatif::ProgressBar;
use std::process::Command;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use which::which;

use crate::{
    helpers::{is_non_interactive, select_with_spinner},
    packages::Packages,
    platform,
};

#[derive(Debug, Clone, Copy, Display, EnumIter)]
pub enum AurHelper {
    Yay,
    Paru,
    None,
}

pub struct ArchInstaller {
    packages: Packages,
    commands: Vec<Vec<String>>,
}

impl ArchInstaller {
    pub fn new(packages: Packages) -> Self {
        Self {
            packages,
            commands: Vec::new(),
        }
    }

    pub fn get_commands(&self) -> &[Vec<String>] {
        &self.commands
    }

    fn detect_aur_helper(&self) -> Option<AurHelper> {
        if which::which("paru").is_ok() {
            Some(AurHelper::Paru)
        } else if which::which("yay").is_ok() {
            Some(AurHelper::Yay)
        } else {
            None
        }
    }

    pub fn install_aur_helper(&mut self, spinner: &ProgressBar) -> anyhow::Result<()> {
        if !platform::is_arch_like() {
            return Ok(());
        }

        if let None = self.detect_aur_helper() {
            let helper = select_with_spinner(
                spinner,
                "Select AUR helper to install:",
                AurHelper::iter().collect(),
            )?;

            let repo = match helper {
                AurHelper::Paru => "https://aur.archlinux.org/paru.git",
                AurHelper::Yay => "https://aur.archlinux.org/yay.git",
                AurHelper::None => return Ok(()),
            };

            let dir = match helper {
                AurHelper::Paru => "/tmp/paru",
                AurHelper::Yay => "/tmp/yay",
                AurHelper::None => return Ok(()),
            };

            let mut pacman_args = vec!["pacman", "-S", "--needed", "base-devel", "git"];
            if is_non_interactive() {
                pacman_args.push("--noconfirm");
            }

            let status = spinner.suspend(|| Command::new("sudo").args(pacman_args).status())?;

            if !status.success() {
                anyhow::bail!("Failed to install AUR build dependencies");
            }

            if std::path::Path::new(dir).exists() {
                std::fs::remove_dir_all(dir)?;
            }

            let status =
                spinner.suspend(|| Command::new("git").args(["clone", repo, dir]).status())?;

            if !status.success() {
                anyhow::bail!("Failed to clone AUR helper repository");
            }

            let mut makepkg_args = vec!["-si"];
            if is_non_interactive() {
                makepkg_args.push("--noconfirm");
            }

            let status = spinner.suspend(|| {
                Command::new("makepkg")
                    .args(makepkg_args)
                    .current_dir(dir)
                    .status()
            })?;

            if !status.success() {
                anyhow::bail!("Failed to build/install AUR helper");
            }
        }

        Ok(())
    }

    pub fn install(&mut self) -> anyhow::Result<()> {
        if !platform::is_arch_like() {
            return Ok(());
        }

        let pacman = &self.packages.arch.pacman;
        if !pacman.is_empty() {
            let mut args = vec!["pacman".into(), "-S".into(), "--needed".into()];

            if is_non_interactive() {
                args.push("--noconfirm".into());
            }

            args.append(&mut pacman.clone());
            self.commands.push(args);
        } else {
            println!("{}", style("No Pacman packages to install").bold().yellow());
        }

        // Aur installs
        let aur = &self.packages.arch.aur;
        if !aur.is_empty() {
            match self.detect_aur_helper() {
                Some(AurHelper::Yay) => {
                    let yay = which("yay");
                    if yay.is_ok() {
                        let mut args = vec![
                            "yay".into(),
                            "-S".into(),
                            "--noconfirm".into(),
                            "--needed".into(),
                        ];
                        args.append(&mut aur.clone());
                        self.commands.push(args);
                    }
                }
                Some(AurHelper::Paru) => {
                    let paru = which("paru");
                    if paru.is_ok() {
                        let mut args = vec![
                            "paru".into(),
                            "-S".into(),
                            "--noconfirm".into(),
                            "--needed".into(),
                        ];
                        args.append(&mut aur.clone());
                        self.commands.push(args);
                    } else {
                        anyhow::bail!("No AUR package manager found");
                    }
                }
                Some(AurHelper::None) => {
                    println!("{}", style("No AUR helper installed").bold().red());
                }
                None => {
                    println!("{}", style("No AUR helper installed").bold().red());
                }
            }
        } else {
            println!("{}", style("No AUR packages to install").bold().yellow());
        }

        Ok(())
    }
}
