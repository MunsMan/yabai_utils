use std::str::FromStr;

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
    Space(SpaceArgs),
    // Display,
}

#[derive(Args)]
pub struct SpaceArgs {
    #[command(subcommand)]
    pub command: SpaceCommand,
}

#[derive(Subcommand)]
pub enum SpaceCommand {
    Focus(SpaceDirectionArgs),
    DestroyAllEmpty,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
pub struct SpaceDirectionArgs {
    pub direction_or_index: DirectionOrIndex,
}

#[derive(Debug, Clone)]
pub enum DirectionOrIndex {
    Left,
    Right,
    Index(u8),
}

impl FromStr for DirectionOrIndex {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(index) = s.parse::<u8>() {
            Ok(DirectionOrIndex::Index(index))
        } else if s.to_lowercase() == "left" {
            Ok(DirectionOrIndex::Left)
        } else if s.to_lowercase() == "right" {
            Ok(DirectionOrIndex::Right)
        } else {
            Err(format!("invalid value: {}", s))
        }
    }
}

#[derive(Args)]
pub struct WindowArgs {
    #[command(subcommand)]
    pub command: Option<WindowCommand>,
}

#[derive(Subcommand)]
pub enum WindowCommand {
    Focus(WindowDirectionArgs),
}

#[derive(clap::Args)]
pub struct WindowDirectionArgs {
    pub direction: Direction,
}
