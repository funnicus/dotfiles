use std::process::Command;

use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use which::which;

use crate::{
    helpers::{confirm_with_spinner, is_non_interactive},
    installers::{arch::ArchInstaller, mac::AppleSiliconInstaller},
    packages::Packages,
    platform::{self, LinuxDistro, Platform},
};

pub struct Installer {
    arch: ArchInstaller,
    silicon: AppleSiliconInstaller,
    pub packages: Packages,
    pub commands: Vec<Vec<String>>,
    pub spinner: ProgressBar,
}

impl Installer {
    pub fn new() -> Result<Self, anyhow::Error> {
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(ProgressStyle::with_template("{spinner} {msg}")?);
        spinner.enable_steady_tick(std::time::Duration::from_millis(80));
        spinner.set_message("Starting installation...");

        let packages = Packages::load()?;
        let arch_packages = packages.clone();
        let silicon_packages = packages.clone();

        Ok(Self {
            packages,
            commands: Vec::new(),
            arch: ArchInstaller::new(arch_packages),
            silicon: AppleSiliconInstaller::new(silicon_packages),
            spinner,
        })
    }

    pub fn bootstrap(&mut self) -> anyhow::Result<()> {
        let proceed = confirm_with_spinner(&self.spinner, "Install package managers?", true)?;

        if !proceed {
            println!("Cancelled.");
            return Ok(());
        }

        self.install_package_managers()?;
        Ok(())
    }

    pub fn install(&mut self) -> anyhow::Result<()> {
        let proceed = confirm_with_spinner(&self.spinner, "Install packages?", true)?;

        if !proceed {
            println!("{}", style("Cancelled.").bold().red());
            return Ok(());
        }

        if cfg!(target_os = "macos") && std::env::consts::ARCH == "x86_64" {
            anyhow::bail!("Unsupported environment: macOS Intel")
        }

        match platform::current() {
            Platform::MacOS => {
                self.silicon.install()?;
            }
            Platform::Linux(LinuxDistro::Arch | LinuxDistro::CachyOS) => {
                self.arch.install()?;
            }
            other => anyhow::bail!("Unsupported OS: {other:?}"),
        }

        self.run_commands()?;

        self.install_fish_plugins()?;
        self.install_node_tools()?;
        self.install_tool_self()?;

        self.print_manual_reminders();

        Ok(())
    }

    fn install_package_managers(&mut self) -> anyhow::Result<()> {
        self.arch.install_aur_helper(&self.spinner)?;

        self.spinner.set_message("Installing homebrew...");

        if which("brew").is_err() {
            let mut command = Command::new("/bin/bash");
            command
                .arg("-c")
                .arg(r#"curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh | /bin/bash"#);
            if is_non_interactive() {
                command.env("NONINTERACTIVE", "1");
            }

            let status = self.spinner.suspend(|| command.status())?;

            if !status.success() {
                return Err(anyhow::anyhow!("Homebrew install failed"));
            }
        }

        self.spinner.set_message("Installing rust...");

        if which("rustup").is_err() {
            let rustup_install = if is_non_interactive() {
                r#"curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y"#
            } else {
                r#"curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"#
            };

            let status = self
                .spinner
                .suspend(|| Command::new("sh").arg("-c").arg(rustup_install).status())?;

            if !status.success() {
                return Err(anyhow::anyhow!("Rust install failed"));
            }
        }

        println!(
            r#"

        ✅ Package managers installed.

        You may need to restart your shell.

        Next, run `dotsetup install` to install desired programs.

        If fish is already installed, on Linux, run this command:

            eval (/home/linuxbrew/.linuxbrew/bin/brew shellenv)

        For fish on Apple Silicon macOS:

            eval (/opt/homebrew/bin/brew shellenv)

        For fish on Intel macOS:

            eval (/usr/local/bin/brew shellenv)

        `chezmoi apply` will add everything to path.
        "#
        );

        Ok(())
    }

    fn install_tool_self(&self) -> anyhow::Result<()> {
        let cargo_command = which("cargo")
            .map(|path| path.to_string_lossy().to_string())
            .or_else(|_| std::env::var("HOME").map(|home| format!("{home}/.cargo/bin/cargo")))?;

        for tool in &self.packages.cargo.packages {
            if which(&tool.check).is_ok() {
                continue;
            }

            if !Command::new(&cargo_command)
                .args(["install", tool.package.as_str()])
                .status()?
                .success()
            {
                return Err(anyhow::anyhow!(
                    "Failed to install Cargo package {}",
                    tool.package
                ));
            }
        }

        for tool in &self.packages.go.packages {
            if which(&tool.check).is_ok() {
                continue;
            }

            if !Command::new("go")
                .args(["install", tool.package.as_str()])
                .status()?
                .success()
            {
                return Err(anyhow::anyhow!(
                    "Failed to install Go package {}",
                    tool.package
                ));
            }
        }

        for tool in &self.packages.npm.packages {
            if which(&tool.check).is_ok() {
                continue;
            }

            let command = format!(
                "functions -q nvm; and nvm use default --silent; and npm install -g '{}'",
                tool.package
            );
            if !Command::new("fish")
                .args(["-c", command.as_str()])
                .status()?
                .success()
            {
                return Err(anyhow::anyhow!(
                    "Failed to install npm package {}",
                    tool.package
                ));
            }
        }

        if self.packages.uv.install_uv
            && which("uv").is_err()
            && !Command::new("sh")
                .args(["-c", "curl -LsSf https://astral.sh/uv/install.sh | sh"])
                .status()?
                .success()
        {
            return Err(anyhow::anyhow!("Failed to install uv"));
        }

        let uv_command = which("uv")
            .map(|path| path.to_string_lossy().to_string())
            .or_else(|_| std::env::var("HOME").map(|home| format!("{home}/.local/bin/uv")))?;

        for tool in &self.packages.uv.packages {
            if which(&tool.check).is_ok() {
                continue;
            }

            if !Command::new(&uv_command)
                .args(["tool", "install", tool.package.as_str()])
                .status()?
                .success()
            {
                return Err(anyhow::anyhow!(
                    "Failed to install uv package {}",
                    tool.package
                ));
            }
        }

        Ok(())
    }

    fn install_fish_plugins(&self) -> anyhow::Result<()> {
        if self.packages.fish.plugins.is_empty() {
            println!("No Fish plugins to install!");
            return Ok(());
        }

        if !Command::new("fish")
            .args([
                "-c",
                "functions -q fisher; or begin; curl -sL https://raw.githubusercontent.com/jorgebucaran/fisher/main/functions/fisher.fish | source; fisher install jorgebucaran/fisher; end",
            ])
            .status()?
            .success()
        {
            return Err(anyhow::anyhow!("Failed to install Fisher"));
        }

        for plugin in &self.packages.fish.plugins {
            let command = format!(
                "fisher list | string match -q -- '{plugin}'; or fisher install '{plugin}'"
            );
            if !Command::new("fish")
                .args(["-c", command.as_str()])
                .status()?
                .success()
            {
                return Err(anyhow::anyhow!("Failed to install Fish plugin {plugin}"));
            }
        }

        Ok(())
    }

    fn install_node_tools(&self) -> anyhow::Result<()> {
        if self.packages.node.install_default_lts
            && !Command::new("fish")
                .args([
                    "-c",
                    "functions -q nvm; and nvm install lts; and set --universal nvm_default_version lts",
                ])
                .status()?
                .success()
        {
            return Err(anyhow::anyhow!("Failed to install default Node.js LTS"));
        }

        if self.packages.node.install_bun
            && which("bun").is_err()
            && !Command::new("sh")
                .args(["-c", "curl -fsSL https://bun.sh/install | bash"])
                .status()?
                .success()
        {
            return Err(anyhow::anyhow!("Failed to install Bun"));
        }

        if self.packages.node.install_pnpm
            && which("pnpm").is_err()
            && !Command::new("sh")
                .args(["-c", "curl -fsSL https://get.pnpm.io/install.sh | sh -"])
                .status()?
                .success()
        {
            return Err(anyhow::anyhow!("Failed to install pnpm"));
        }

        Ok(())
    }

    fn print_manual_reminders(&self) {
        if self.packages.manual.reminders.is_empty() {
            return;
        }

        println!("\nManual setup reminders:");
        for reminder in &self.packages.manual.reminders {
            println!("  - {reminder}");
        }
        println!();
    }

    fn run_commands(&mut self) -> anyhow::Result<()> {
        let mut commands = self.arch.get_commands().to_vec();
        commands.extend(self.silicon.get_commands().to_vec());
        commands.extend(self.commands.clone());

        for command in commands {
            let mut cmd = Command::new(&command[0]);
            cmd.args(&command[1..]);
            if !self.spinner.suspend(|| cmd.status())?.success() {
                return Err(anyhow::anyhow!("Failed to run command: {:?}", command));
            }
        }

        Ok(())
    }
}
