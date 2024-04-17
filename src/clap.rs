use clap::{Args, Parser, Subcommand};

use crate::windows::Direction;

#[derive(Parser)]
#[command(version, about, long_about=None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Window(WindowArgs),
    // Space,
    // Display,
}

#[derive(Args)]
pub struct WindowArgs {
    #[command(subcommand)]
    pub command: Option<WindowCommands>,
}

#[derive(Subcommand)]
pub enum WindowCommands {
    Focus(DirectionArgs),
}

#[derive(clap::Args)]
pub struct DirectionArgs {
    pub direction: Direction,
}
