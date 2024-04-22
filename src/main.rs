use ::clap::Parser;

use crate::clap::{Cli, Commands, WindowDirectionArgs};
use crate::windows::{current_window, order_windows, resize_window, Direction};
use crate::yabai::{focus_window, query_windows};

use self::clap::{SpaceCommand, WindowCommand, WindowResizeDirectionArgs};
use self::spaces::{destroy_all_empty, focus_space};

mod clap;
mod spaces;
mod windows;
mod yabai;

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Window(x) => match &x.command {
            WindowCommand::Focus(WindowDirectionArgs { direction }) => {
                let windows = query_windows();
                let current_window = current_window(&windows).unwrap();
                let store = order_windows(&windows);
                let window = store.get(&current_window.id).unwrap();
                if let Some(neighbour_id) = window.neigbour(direction) {
                    focus_window(neighbour_id)
                }
            }
            WindowCommand::Resize(WindowResizeDirectionArgs {
                left,
                right,
                up,
                down,
            }) => {
                if let Some(left) = left {
                    resize_window(Direction::Left, *left)
                }
                if let Some(right) = right {
                    resize_window(Direction::Right, *right)
                }
                if let Some(up) = up {
                    resize_window(Direction::Up, *up)
                }
                if let Some(down) = down {
                    resize_window(Direction::Down, *down)
                }
            }
        },
        Commands::Space(arg) => match &arg.command {
            SpaceCommand::Focus(arg) => focus_space(&arg.direction_or_index),
            SpaceCommand::DestroyAllEmpty => destroy_all_empty(),
        },
    }
}
