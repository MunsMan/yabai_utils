use std::process::{Command, Output};

use clap::ValueEnum;
use serde::{Deserialize, Serialize};

use crate::spaces::SpaceIndex;
use crate::windows::{Direction, WindowId};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
struct YabaiSignal {
    index: u32,
    label: String,
    app: String,
    event: YabaiSignalEvent,
    action: String,
}

#[derive(Serialize, Deserialize, Debug)]
enum SpaceType {
    #[serde(rename = "managed")]
    Managed,
    #[serde(rename = "bsp")]
    Bsp,
    #[serde(rename = "float")]
    Float,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct YabaiSpaceObject {
    id: u32,
    uuid: String,
    pub index: SpaceIndex,
    label: String,
    #[serde(rename = "type")]
    space_type: SpaceType,
    display: u32,
    pub windows: Vec<WindowId>,
    first_window: WindowId,
    last_window: WindowId,
    pub has_focus: bool,
    is_visible: bool,
    is_native_fullscreen: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct YabaiWindowObject {
    pub id: usize,
    pid: usize,
    pub app: String,
    pub title: String,
    pub frame: YabaiWindowFrame,
    root_window: bool,
    display: usize,
    space: usize,
    level: usize,
    sub_level: isize,
    layer: String,
    sub_layer: String,
    opacity: f32,
    split_type: String,
    split_child: String,
    stack_index: usize,
    can_move: bool,
    can_resize: bool,
    pub has_focus: bool,
    has_shadow: bool,
    has_parent_zoom: bool,
    has_fullscreen_zoom: bool,
    has_ax_reference: bool,
    is_native_fullscreen: bool,
    pub is_visible: bool,
    pub is_minimized: bool,
    is_hidden: bool,
    is_floating: bool,
    is_sticky: bool,
    is_grabbed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct YabaiWindowFrame {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

impl YabaiWindowFrame {
    pub fn size(&self) -> f64 {
        self.w * self.h
    }
}

impl std::cmp::PartialEq for YabaiWindowFrame {
    fn eq(&self, other: &Self) -> bool {
        self.size() == other.size()
    }
}

impl std::cmp::PartialOrd for YabaiWindowFrame {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let size = self.size();
        let other_size = other.size();
        size.partial_cmp(&other_size)
    }
}

pub fn query_windows() -> Vec<YabaiWindowObject> {
    let mut windows = query_yabai::<YabaiWindowObject>("query --windows --space").unwrap();
    windows.retain(|x| x.is_visible && !x.is_hidden);
    windows
}

pub fn query_spaces() -> Vec<YabaiSpaceObject> {
    query_yabai("query --spaces").unwrap()
}

#[derive(Debug)]
pub enum YabaiQueryError {
    CommandExecutionError,
    ResponseParsingError,
}

fn query_yabai<T>(message: &str) -> Result<Vec<T>, YabaiQueryError>
where
    T: for<'a> Deserialize<'a>,
{
    let mut args = Vec::new();
    args.push("-m");
    args.append(&mut Iterator::collect(message.split_whitespace()));
    let result = match Command::new("yabai").args(args).output() {
        Ok(stream) => stream.stdout,
        Err(_) => return Err(YabaiQueryError::CommandExecutionError),
    };

    match serde_json::from_slice(&result) {
        Ok(result) => Ok(result),
        Err(e) => {
            dbg!(e);
            Err(YabaiQueryError::ResponseParsingError)
        }
    }
}

fn send_yabai(message: &str) -> Result<Output, std::io::Error> {
    let mut args = Vec::new();
    args.push("-m");
    args.append(&mut Iterator::collect(message.split_whitespace()));
    dbg!(&args);
    let output = Command::new("yabai").args(args).output()?;
    Ok(output)
}

pub fn focus_window(window_id: WindowId) {
    let _ = send_yabai(format!("window --focus {}", &window_id).as_str());
}

pub fn yabai_focus_space(space_index: SpaceIndex) {
    let _ = send_yabai(format!("space --focus {}", &space_index).as_str());
}

pub fn yabai_delete_space(space_index: SpaceIndex) {
    let _ = send_yabai(format!("space {} --destroy", &space_index).as_str());
}

pub fn yabai_create_space() {
    let _ = send_yabai("space  --create");
}

pub fn yabai_resize_window(direction: Direction, offset: i32) {
    let (corner, offset) = match direction {
        Direction::Up => ("top_right", format!("0:{}", offset)),
        Direction::Left => ("top_left", format!("{}:0", offset)),
        Direction::Down => ("bottom_right", format!("0:{}", offset)),
        Direction::Right => ("bottom_right", format!("{}:0", offset)),
    };
    let _ = send_yabai(format!("window --resize {}:{}", corner, offset).as_str());
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, PartialEq, ValueEnum, Clone)]
#[serde(rename_all = "snake_case")]
pub enum YabaiSignalEvent {
    ApplicationLaunched,
    ApplicationTerminated,
    ApplicationFrontSwitched,
    ApplicationActivated,
    ApplicationDeactivated,
    ApplicationVisible,
    ApplicationHidden,
    WindowCreated,
    WindowDestroyed,
    WindowFocused,
    WindowMoved,
    WindowResized,
    WindowMinimized,
    WindowDeminimized,
    WindowTitleChanged,
    SpaceCreated,
    SpaceDestroyed,
    SpaceChanged,
    DisplayAdded,
    DisplayRemoved,
    DisplayMoved,
    DisplayResized,
    DisplayChanged,
    MissionControlEnter,
    MissionControlExit,
    DockDidChangePref,
    DockDidRestart,
    MenuBarHiddenChanged,
    SystemWoke,
}

impl std::fmt::Display for YabaiSignalEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            YabaiSignalEvent::ApplicationLaunched => write!(f, "application_launched"),
            YabaiSignalEvent::ApplicationTerminated => write!(f, "application_terminated"),
            YabaiSignalEvent::ApplicationFrontSwitched => write!(f, "application_front_switched"),
            YabaiSignalEvent::ApplicationActivated => write!(f, "application_activated"),
            YabaiSignalEvent::ApplicationDeactivated => write!(f, "application_deactivated"),
            YabaiSignalEvent::ApplicationVisible => write!(f, "application_visible"),
            YabaiSignalEvent::ApplicationHidden => write!(f, "application_hidden"),
            YabaiSignalEvent::WindowCreated => write!(f, "window_created"),
            YabaiSignalEvent::WindowDestroyed => write!(f, "window_destroyed"),
            YabaiSignalEvent::WindowFocused => write!(f, "window_focused"),
            YabaiSignalEvent::WindowMoved => write!(f, "window_moved"),
            YabaiSignalEvent::WindowResized => write!(f, "window_resized"),
            YabaiSignalEvent::WindowMinimized => write!(f, "window_minimized"),
            YabaiSignalEvent::WindowDeminimized => write!(f, "window_deminimized"),
            YabaiSignalEvent::WindowTitleChanged => write!(f, "window_title_changed"),
            YabaiSignalEvent::SpaceCreated => write!(f, "space_created"),
            YabaiSignalEvent::SpaceDestroyed => write!(f, "space_destroyed"),
            YabaiSignalEvent::SpaceChanged => write!(f, "space_changed"),
            YabaiSignalEvent::DisplayAdded => write!(f, "display_added"),
            YabaiSignalEvent::DisplayRemoved => write!(f, "display_removed"),
            YabaiSignalEvent::DisplayMoved => write!(f, "display_moved"),
            YabaiSignalEvent::DisplayResized => write!(f, "display_resized"),
            YabaiSignalEvent::DisplayChanged => write!(f, "display_changed"),
            YabaiSignalEvent::MissionControlEnter => write!(f, "mission_control_enter"),
            YabaiSignalEvent::MissionControlExit => write!(f, "mission_control_exit"),
            YabaiSignalEvent::DockDidChangePref => write!(f, "dock_did_change_pref"),
            YabaiSignalEvent::DockDidRestart => write!(f, "dock_did_restart"),
            YabaiSignalEvent::MenuBarHiddenChanged => write!(f, "menu_bar_hidden_changed"),
            YabaiSignalEvent::SystemWoke => write!(f, "system_woke"),
        }
    }
}

const YABAI_UTILS_LABEL: &str = "yabai-utils";

pub fn yabai_add_event(event: YabaiSignalEvent) {
    let _output = Command::new("yabai")
        .args(vec![
            "-m",
            "signal",
            "--add",
            &format!("event={}", event),
            &format!(
                "action=yabai_utils signal-event {}",
                event.to_string().replace('_', "-")
            ),
            &format!("label={}", YABAI_UTILS_LABEL),
        ])
        .output();
}

fn query_signal_events() -> Vec<YabaiSignal> {
    let output = send_yabai("signal --list").unwrap();
    serde_json::from_slice(&output.stdout).unwrap()
}

pub fn yabai_remove_event(event: &YabaiSignalEvent) {
    let query = query_signal_events();
    for signal in query {
        if signal.label == YABAI_UTILS_LABEL && signal.event == *event {
            let _ = send_yabai(&format!("signal --remove {}", signal.index));
        }
    }
}
