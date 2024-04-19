use std::process::Command;

use serde::{Deserialize, Serialize};

use crate::windows::WindowId;

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
    let result = Command::new("yabai")
        .args(["-m", "query", "--windows", "--space"])
        .output()
        .expect("failed to execute process")
        .stdout;
    let mut windows: Vec<YabaiWindowObject> = serde_json::from_slice(&result).unwrap();
    windows.retain(|x| x.is_visible && !x.is_hidden);
    windows
}

pub fn focus_window(window_id: WindowId) {
    let _ = Command::new("yabai")
        .args(["-m", "window", "--focus", &window_id.to_string()])
        .spawn();
}
