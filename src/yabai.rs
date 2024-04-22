use std::process::Command;

use serde::{Deserialize, Serialize};

use crate::spaces::SpaceIndex;
use crate::windows::{Direction, WindowId};

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
    is_visible: bool,
    is_minimized: bool,
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

fn send_yabai(message: &str) {
    let mut args = Vec::new();
    args.push("-m");
    args.append(&mut Iterator::collect(message.split_whitespace()));
    let _ = Command::new("yabai").args(args).output();
}

pub fn focus_window(window_id: WindowId) {
    send_yabai(format!("window --focus {}", &window_id).as_str());
}

pub fn yabai_focus_space(space_index: SpaceIndex) {
    send_yabai(format!("space --focus {}", &space_index).as_str());
}

pub fn yabai_delete_space(space_index: SpaceIndex) {
    send_yabai(format!("space {} --destroy", &space_index).as_str());
}

pub fn yabai_create_space() {
    send_yabai("space  --create");
}

pub fn yabai_resize_window(direction: Direction, offset: i32) {
    let (corner, offset) = match direction {
        Direction::Up => ("top_right", format!("0:{}", offset)),
        Direction::Left => ("top_left", format!("{}:0", offset)),
        Direction::Down => ("bottom_right", format!("0:{}", offset)),
        Direction::Right => ("bottom_right", format!("{}:0", offset)),
    };
    send_yabai(format!("window --resize {}:{}", corner, offset).as_str());
}
