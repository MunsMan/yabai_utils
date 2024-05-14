use std::collections::HashMap;
use std::ops::Sub;

use crate::clap::DirectionOrIndex;
use crate::yabai::{
    query_spaces, query_windows, yabai_create_space, yabai_focus_space, yabai_focus_window,
    yabai_move_window_space, yabai_resize_window, YabaiWindowObject,
};

pub type WindowId = usize;

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Direction {
    Up,
    Left,
    Down,
    Right,
}

pub struct Positon {
    pub x: f64,
    pub y: f64,
}

impl Sub<Positon> for Positon {
    type Output = Positon;

    fn sub(self, positon: Positon) -> Self::Output {
        Positon {
            x: self.x - positon.x,
            y: self.y - positon.y,
        }
    }
}

#[derive(Debug, Default)]
pub struct WindowNeighbours {
    pub up: Option<WindowId>,
    pub right: Option<WindowId>,
    pub down: Option<WindowId>,
    pub left: Option<WindowId>,
}

impl WindowNeighbours {
    pub fn neigbour(&self, direction: &Direction) -> Option<WindowId> {
        match direction {
            Direction::Up => self.up,
            Direction::Left => self.left,
            Direction::Down => self.down,
            Direction::Right => self.right,
        }
    }
}

pub fn order_windows(windows: &[YabaiWindowObject]) -> HashMap<WindowId, WindowNeighbours> {
    let mut result = HashMap::new();
    let mut windows_hash = HashMap::new();

    for window in windows {
        windows_hash.insert(window.id, window);
    }
    for window in windows {
        let win = &window.frame;
        let win_right = win.x + win.w;
        let win_bottom = win.y + win.h;

        let mut closest = WindowNeighbours::default();

        for other in windows.iter() {
            if other.id == window.id {
                continue;
            }

            let other_right = other.frame.x + other.frame.w;
            let other_bottom = other.frame.y + other.frame.h;

            let vertical_overlap = win.y < other_bottom && win_bottom > other.frame.y;
            let horizontal_overlap = win.x < other_right && win_right > other.frame.x;

            // Check right neighbor
            if other.frame.x > win.x
                && vertical_overlap
                && (closest.right.is_none()
                    || other.frame.x < windows_hash.get(&closest.right.unwrap()).unwrap().frame.x)
            {
                closest.right = Some(other.id);
            }

            // Check left neighbor
            if other_right < win_right
                && vertical_overlap
                && (closest.left.is_none()
                    || other_right
                        > windows_hash.get(&closest.left.unwrap()).unwrap().frame.x
                            + windows_hash.get(&closest.left.unwrap()).unwrap().frame.w)
            {
                closest.left = Some(other.id);
            }

            // Check up neighbor
            if other_bottom < win_bottom
                && horizontal_overlap
                && (closest.up.is_none()
                    || other_bottom
                        > windows_hash.get(&closest.up.unwrap()).unwrap().frame.y
                            + windows_hash.get(&closest.up.unwrap()).unwrap().frame.h)
            {
                closest.up = Some(other.id);
            }

            // Check down neighbor
            if other.frame.y > win.y
                && horizontal_overlap
                && (closest.down.is_none()
                    || other.frame.y < windows_hash.get(&closest.down.unwrap()).unwrap().frame.y)
            {
                closest.down = Some(other.id);
            }
        }
        result.insert(window.id, closest);
    }
    result
}

pub fn focused_window(windows: &[YabaiWindowObject]) -> Option<&YabaiWindowObject> {
    windows.iter().find(|x| x.has_focus)
}

pub fn resize_window(direction: Direction, offset: i32) {
    yabai_resize_window(direction, offset);
}

pub fn auto_focus() {
    let mut windows = query_windows();
    windows.retain(|x| x.is_visible && !x.is_hidden);
    if focused_window(&windows).is_none() {
        let next_window = windows.iter().reduce(|largest, window| {
            if largest.frame < window.frame {
                window
            } else {
                largest
            }
        });
        if let Some(next_window) = next_window {
            yabai_focus_window(next_window.id);
        }
    }
}

pub fn focus_window_by_direction(direction: &Direction, ignore_sticky: bool) {
    let mut windows = query_windows();
    windows.retain(|x| x.is_visible && !x.is_hidden && (!x.is_sticky || ignore_sticky));
    dbg!(&windows);
    let current_window = focused_window(&windows).unwrap();
    let store = order_windows(&windows);
    let window = store.get(&current_window.id).unwrap();
    if let Some(neighbour_id) = window.neigbour(direction) {
        yabai_focus_window(neighbour_id)
    }
}

pub fn move_window_to_space(direction_or_index: &DirectionOrIndex, follow_focus: bool) {
    let spaces_infos = query_spaces();
    let windows = query_windows();
    let focused_window = focused_window(&windows);
    if focused_window.is_none() {
        return;
    }
    let num_spaces = spaces_infos.len() as u8;
    let index = match direction_or_index {
        DirectionOrIndex::Left => {
            let current_space = spaces_infos.iter().find(|x| x.has_focus).unwrap();
            if current_space.index > 2 {
                current_space.index - 1
            } else {
                num_spaces
            }
        }
        DirectionOrIndex::Right => {
            let current_space = spaces_infos.iter().find(|x| x.has_focus).unwrap();
            if current_space.index == num_spaces {
                1
            } else {
                current_space.index + 1
            }
        }
        DirectionOrIndex::Index(index) => {
            if index <= &num_spaces {
                *index
            } else {
                for _ in num_spaces..*index {
                    yabai_create_space()
                }
                *index
            }
        }
    };
    yabai_move_window_space(index);
    if follow_focus {
        yabai_focus_space(index);
        yabai_focus_window(focused_window.unwrap().id);
    }
}
