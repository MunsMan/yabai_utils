use ::clap::Parser;

use crate::clap::{Cli, Commands, DirectionArgs, WindowCommands};
use crate::windows::{current_window, order_windows};
use crate::yabai::{focus_window, query_windows};

mod clap;
mod windows;
mod yabai;

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Window(x)) => match &x.command {
            Some(WindowCommands::Focus(DirectionArgs { direction })) => {
                let windows = query_windows();
                let current_window = current_window(&windows).unwrap();
                let store = order_windows(&windows);
                let window = store.get(&current_window.id).unwrap();
                if let Some(neighbour_id) = window.neigbour(direction) {
                    focus_window(neighbour_id)
                }
            }
            None => todo!(),
        },
        None => todo!(),
    }
}
