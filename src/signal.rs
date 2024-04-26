use crate::spaces::destroy_all_empty;
use crate::windows::auto_focus;
use crate::yabai::{yabai_add_event, yabai_remove_event, YabaiSignalEvent};

fn signals() -> [YabaiSignalEvent; 2] {
    [
        YabaiSignalEvent::WindowMinimized,
        YabaiSignalEvent::WindowDestroyed,
    ]
}

pub fn signal_event_handler(event: &YabaiSignalEvent) {
    match event {
        YabaiSignalEvent::WindowMinimized => auto_focus(),
        YabaiSignalEvent::WindowDestroyed => {
            auto_focus();
            destroy_all_empty();
        }
        YabaiSignalEvent::WindowMoved => destroy_all_empty(),
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
