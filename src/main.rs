use ::clap::Parser;

use crate::clap::{Cli, Commands, WindowDirectionArgs};
use crate::windows::{focused_window, order_windows, resize_window, Direction};
use crate::yabai::{focus_window, query_windows};

use self::clap::{SignalCommand, SpaceCommand, WindowCommand, WindowResizeDirectionArgs};
use self::signal::{load_signal, signal_event_handler, unload_signal};
use self::spaces::{destroy_all_empty, focus_space};
use self::windows::{auto_focus, move_window_to_space};

mod clap;
mod signal;
mod spaces;
mod windows;
mod yabai;

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Window(x) => match &x.command {
            WindowCommand::Focus(WindowDirectionArgs { direction }) => {
                let windows = query_windows();
                let current_window = focused_window(&windows).unwrap();
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
            WindowCommand::Move(arg) => move_window_to_space(&arg.direction_or_index, true),
            WindowCommand::AutoFocus => auto_focus(),
        },
        Commands::Space(arg) => match &arg.command {
            SpaceCommand::Focus(arg) => focus_space(&arg.direction_or_index),
            SpaceCommand::DestroyAllEmpty => destroy_all_empty(),
        },
        Commands::Signal(arg) => match &arg.command {
            SignalCommand::Load => load_signal(),
            SignalCommand::Unload => unload_signal(),
            SignalCommand::SignalEvent(signal) => signal_event_handler(&signal.event),
        },
    }
}
