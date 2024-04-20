use ::clap::Parser;

use crate::clap::{Cli, Commands, WindowDirectionArgs};
use crate::windows::{current_window, order_windows};
use crate::yabai::{focus_window, query_windows};

use self::clap::{SpaceCommand, WindowCommand};
use self::spaces::{destroy_all_empty, focus_space};

mod clap;
mod spaces;
mod windows;
mod yabai;

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Window(x)) => match &x.command {
            Some(WindowCommand::Focus(WindowDirectionArgs { direction })) => {
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
        Some(Commands::Space(arg)) => match &arg.command {
            SpaceCommand::Focus(arg) => focus_space(&arg.direction_or_index),
            SpaceCommand::DestroyAllEmpty => destroy_all_empty(),
        },
        None => todo!(),
    }
}
