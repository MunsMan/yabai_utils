use std::str::FromStr;

use clap::{Args, Parser, Subcommand};

use crate::windows::Direction;
use crate::yabai::YabaiSignalEvent;

#[derive(Parser)]
#[command(version, about, long_about=None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Handle Windows
    Window(WindowArgs),
    /// Handle Spaces
    Space(SpaceArgs),
    /// Signals,
    Signal(SignalArgs),
}
#[derive(Args)]
pub struct SignalArgs {
    #[command(subcommand)]
    pub command: SignalCommand,
}

#[derive(Subcommand)]
pub enum SignalCommand {
    Load,
    Unload,
    SignalEvent(SignalEventArg),
}

#[derive(Args)]
pub struct SignalEventArg {
    pub event: YabaiSignalEvent,
}

#[derive(Args)]
pub struct SpaceArgs {
    #[command(subcommand)]
    pub command: SpaceCommand,
}

#[derive(Subcommand)]
pub enum SpaceCommand {
    /// Command to manage the Spaces focus
    /// Allows for cycling through spaces with directions
    /// or direct access by providing an index.
    /// When a space with the provided index doesn't exist, yabai-utils creates it for you
    Focus(SpaceDirectionArgs),
    /// Destroying empty spaces
    DestroyAllEmpty,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
pub struct SpaceDirectionArgs {
    /// Provide a direction <left, right> or an index <number>
    pub direction_or_index: DirectionOrIndex,
}

#[derive(Debug, Clone)]
pub enum DirectionOrIndex {
    /// Selecting the space to the Left
    Left,
    /// Selecting the space to the Right
    Right,
    /// Going to space with <index>
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
    pub command: WindowCommand,
}

#[derive(Subcommand)]
pub enum WindowCommand {
    Focus(WindowDirectionArgs),
    Resize(WindowResizeDirectionArgs),
    Move(SpaceDirectionArgs),
    AutoFocus,
}

#[derive(clap::Args)]
pub struct WindowDirectionArgs {
    pub direction: Direction,
}

#[derive(clap::Args)]
#[group(required = true)]
pub struct WindowResizeDirectionArgs {
    #[arg(short, long)]
    pub left: Option<i32>,
    #[arg(short, long)]
    pub right: Option<i32>,
    #[arg(short, long)]
    pub up: Option<i32>,
    #[arg(short, long)]
    pub down: Option<i32>,
}
