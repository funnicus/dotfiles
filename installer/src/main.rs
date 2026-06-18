use clap::Parser;
use console::style;

use crate::{
    cli::{Cli, DotCommand},
    installers::installer::Installer,
};

mod cli;
mod helpers;
mod installers;
mod packages;
mod platform;

fn main() -> anyhow::Result<()> {
    println!("{}", style("Dotfiles Installer 🧰").bold().cyan());

    let cli = Cli::parse();
    let mut install = Installer::new()?;

    let result = match cli.command {
        DotCommand::Install => install.install(),
        DotCommand::Bootstrap => install.bootstrap(),
    };

    match result {
        Ok(_) => install.spinner.finish_with_message("Done"),
        Err(_) => install.spinner.finish_and_clear(),
    }

    result
}
