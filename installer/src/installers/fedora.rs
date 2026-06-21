use console::style;

use crate::{helpers::is_non_interactive, packages::Packages, platform};

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
            self.commands.push(vec![
                "dnf".into(),
                "install".into(),
                "--assumeyes".into(),
                "dnf-plugins-core".into(),
            ]);

            for repository in copr {
                self.commands.push(vec![
                    "dnf".into(),
                    "copr".into(),
                    "enable".into(),
                    "--assumeyes".into(),
                    repository.clone(),
                ]);
            }
        }

        let dnf = &self.packages.fedora.dnf;
        if dnf.is_empty() {
            println!("{}", style("No DNF packages to install").bold().yellow());
            return Ok(());
        }

        let mut args = vec!["dnf".into(), "install".into()];
        if is_non_interactive() {
            args.push("--assumeyes".into());
        }
        args.append(&mut dnf.clone());
        self.commands.push(args);

        Ok(())
    }
}
