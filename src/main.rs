use ::clap::Parser;

use crate::clap::{Cli, Commands, WindowDirectionArgs};
use crate::windows::{resize_window, Direction};

use self::clap::{SignalCommand, SpaceCommand, WindowCommand, WindowResizeDirectionArgs};
use self::log::log;
use self::signal::{load_signal, signal_event_handler, unload_signal};
use self::spaces::{destroy_all_empty, focus_space};
use self::windows::{auto_focus, focus_window_by_direction, move_window_to_space};

mod clap;
mod log;
mod signal;
mod spaces;
mod windows;
mod yabai;

fn main() -> Result<(), ()> {
    let cli = Cli::try_parse();
    let cli = match cli {
        Ok(x) => x,
        Err(e) => {
            log(e.to_string());
            let _ = e.print();
            return Err(());
        }
    };
    match &cli.command {
        Commands::Window(x) => match &x.command {
            WindowCommand::Focus(WindowDirectionArgs {
                direction,
                ignore_sticky,
            }) => focus_window_by_direction(direction, *ignore_sticky),
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
        Commands::Signal(arg) => {
            log(format!("Signal {:?}", arg.command));
            match &arg.command {
                SignalCommand::Load => load_signal(),
                SignalCommand::Unload => unload_signal(),
                SignalCommand::Event(signal) => signal_event_handler(&signal.event),
            }
        }
    }
    Ok(())
}
