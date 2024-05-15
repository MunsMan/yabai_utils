use std::cmp::Ordering;
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

#[derive(Debug)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Sub<Position> for Position {
    type Output = Position;

    fn sub(self, positon: Position) -> Self::Output {
        Position {
            x: self.x - positon.x,
            y: self.y - positon.y,
        }
    }
}

impl Position {
    fn fuzz_cmp_x(&self, other: Position, fuzz: f64) -> Ordering {
        let delta = other.x - self.x;
        if delta < fuzz {
            Ordering::Equal
        } else {
            self.x.total_cmp(&other.x)
        }
    }
    fn fuzz_cmp_y(&self, other: Position, fuzz: f64) -> Ordering {
        let delta = other.y - self.y;
        if delta < fuzz {
            Ordering::Equal
        } else {
            self.y.total_cmp(&other.y)
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

pub fn new_window_order(windows: &[YabaiWindowObject]) -> HashMap<WindowId, WindowNeighbours> {
    let fuzz = 15.0;
    let mut windows_hash = HashMap::new();

    let mut left_right = Vec::new();
    let mut up_down = Vec::new();
    for window in windows {
        windows_hash.insert(window.id, window);
        left_right.push(window.id);
        up_down.push(window.id);
    }
    let mut results = HashMap::new();
    left_right.sort_by(|x, y| {
        windows_hash
            .get(x)
            .unwrap()
            .frame
            .center()
            .fuzz_cmp_x(windows_hash.get(y).unwrap().frame.center(), fuzz)
    });
    up_down.sort_by(|x, y| {
        windows_hash
            .get(x)
            .unwrap()
            .frame
            .center()
            .fuzz_cmp_y(windows_hash.get(y).unwrap().frame.center(), fuzz)
    });
    let up_down: Vec<Vec<usize>> = up_down[1..]
        .iter()
        .fold(vec![vec![up_down[0]]], |mut acc, e| {
            let last_id = acc.last().unwrap().last().unwrap();
            match windows_hash
                .get(last_id)
                .unwrap()
                .frame
                .center()
                .fuzz_cmp_y(windows_hash.get(e).unwrap().frame.center(), fuzz)
            {
                Ordering::Equal => {
                    let last = acc.last_mut().unwrap();
                    last.push(*e);
                }
                _ => {
                    acc.push(vec![*e]);
                }
            }
            acc
        })
        .into_iter()
        .map(|mut x| {
            x.sort_by(|x, y| {
                windows_hash
                    .get(x)
                    .unwrap()
                    .frame
                    .center()
                    .fuzz_cmp_x(windows_hash.get(y).unwrap().frame.center(), fuzz)
            });
            x
        })
        .collect();
    let left_right: Vec<Vec<usize>> = left_right[1..]
        .iter()
        .fold(vec![vec![left_right[0]]], |mut acc, e| {
            let last_id = acc.last().unwrap().last().unwrap();
            match windows_hash
                .get(last_id)
                .unwrap()
                .frame
                .center()
                .fuzz_cmp_x(windows_hash.get(e).unwrap().frame.center(), fuzz)
            {
                Ordering::Equal => {
                    let last = acc.last_mut().unwrap();
                    last.push(*e);
                }
                _ => {
                    acc.push(vec![*e]);
                }
            }
            acc
        })
        .into_iter()
        .map(|mut x| {
            x.sort_by(|x, y| {
                windows_hash
                    .get(x)
                    .unwrap()
                    .frame
                    .center()
                    .fuzz_cmp_x(windows_hash.get(y).unwrap().frame.center(), fuzz)
            });
            x
        })
        .collect();
    for window in windows {
        results.insert(window.id, WindowNeighbours::default());
    }
    let mut prev = None;
    let mut next;
    for (i, windows) in up_down.iter().enumerate() {
        next = up_down.get(i + 1).map(|x| *x.first().unwrap());
        for window_id in windows {
            results.entry(*window_id).and_modify(|x| {
                x.up = prev;
                x.down = next;
            });
        }
        prev = Some(*windows.first().unwrap())
    }
    let mut prev = None;
    for (i, windows) in left_right.iter().enumerate() {
        next = left_right.get(i + 1).map(|x| *x.first().unwrap());
        for window_id in windows {
            results.entry(*window_id).and_modify(|x| {
                x.left = prev;
                x.right = next;
            });
        }
        prev = Some(*windows.first().unwrap())
    }
    results
}

#[allow(dead_code)]
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
    let current_window = focused_window(&windows).unwrap();
    let store = new_window_order(&windows);
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
