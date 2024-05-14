use crate::clap::{SignalEvent, YabaiWindowId};
use crate::spaces::destroy_all_empty;
use crate::windows::auto_focus;
use crate::yabai::{yabai_add_event, yabai_focus_window, yabai_remove_event, YabaiSignalEvent};

fn signals() -> [YabaiSignalEvent; 4] {
    [
        YabaiSignalEvent::WindowMinimized,
        YabaiSignalEvent::WindowDestroyed,
        YabaiSignalEvent::WindowMoved,
        YabaiSignalEvent::WindowCreated,
    ]
}

pub fn signal_event_handler(event: &SignalEvent) {
    match event {
        SignalEvent::WindowMinimized(_) => auto_focus(),
        SignalEvent::WindowDeminimized(YabaiWindowId { window_id }) => {
            yabai_focus_window(*window_id);
        }
        SignalEvent::WindowDestroyed(_) => {
            auto_focus();
            destroy_all_empty();
        }
        SignalEvent::WindowMoved(_) => destroy_all_empty(),
        SignalEvent::WindowCreated(YabaiWindowId { window_id }) => {
            yabai_focus_window(*window_id);
        }
        _ => {}
    }
}

pub fn load_signal() {
    for event in signals() {
        println!("{}", event);
        yabai_add_event(event);
    }
}

pub fn unload_signal() {
    for event in signals() {
        yabai_remove_event(&event);
    }
}
