use clap::{Parser, Subcommand};

#[derive(Subcommand)]
pub enum DotCommand {
    Install,
    Bootstrap,
}

#[derive(Parser)]
#[command(name = "dotsetup")]
#[command(about = "Bootstrap my dotfiles environment")]
pub struct Cli {
    #[command(subcommand)]
    pub command: DotCommand,
}
