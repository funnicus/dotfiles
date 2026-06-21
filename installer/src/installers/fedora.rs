use console::style;

use crate::{
    helpers::{is_non_interactive, root_command},
    packages::Packages,
    platform,
};

pub struct FedoraInstaller {
    packages: Packages,
    commands: Vec<Vec<String>>,
}

impl FedoraInstaller {
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
        if !platform::is_fedora() {
            return Ok(());
        }

        let copr = &self.packages.fedora.copr;
        if !copr.is_empty() {
            self.commands.push(root_command(
                "dnf",
                vec![
                    "install".into(),
                    "--assumeyes".into(),
                    "dnf-plugins-core".into(),
                ],
            ));

            for repository in copr {
                self.commands.push(root_command(
                    "dnf",
                    vec![
                        "copr".into(),
                        "enable".into(),
                        "--assumeyes".into(),
                        repository.clone(),
                    ],
                ));
            }
        }

        let dnf = &self.packages.fedora.dnf;
        if dnf.is_empty() {
            println!("{}", style("No DNF packages to install").bold().yellow());
            return Ok(());
        }

        let mut args = vec!["install".into()];
        if is_non_interactive() {
            args.push("--assumeyes".into());
        }
        args.append(&mut dnf.clone());
        self.commands.push(root_command("dnf", args));

        Ok(())
    }
}
