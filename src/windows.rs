use std::collections::HashMap;
use std::ops::Sub;

use crate::yabai::YabaiWindowObject;

pub type WindowId = usize;

#[derive(clap::ValueEnum, Clone)]
pub enum Direction {
    North,
    West,
    South,
    East,
}

pub struct Positon {
    pub x: isize,
    pub y: isize,
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

impl Positon {
    pub fn abs(&self) -> usize {
        (self.x.pow(2) + self.y.pow(2)) as usize
    }
}

pub struct WindowNeighbours {
    pub north: Option<WindowId>,
    pub west: Option<WindowId>,
    pub south: Option<WindowId>,
    pub east: Option<WindowId>,
}

impl WindowNeighbours {
    pub fn new() -> Self {
        WindowNeighbours {
            north: None,
            west: None,
            south: None,
            east: None,
        }
    }

    pub fn neigbour(&self, direction: &Direction) -> Option<WindowId> {
        match direction {
            Direction::North => self.north,
            Direction::West => self.west,
            Direction::South => self.south,
            Direction::East => self.east,
        }
    }
}

pub fn order_windows(windows: &[YabaiWindowObject]) -> HashMap<WindowId, WindowNeighbours> {
    let mut store = HashMap::new();
    for window in windows {
        let mut win = WindowNeighbours::new();
        let mut best_north = usize::MAX;
        let mut best_east = usize::MAX;
        let mut best_south = usize::MAX;
        let mut best_west = usize::MAX;
        for other_windows in windows {
            if window.id == other_windows.id {
                continue;
            }
            let directions = window.frame.direction(&other_windows.frame);
            directions.into_iter().for_each(|direction| {
                if let Some((score, direction)) = direction {
                    match direction {
                        Direction::North => {
                            if best_north > score {
                                best_north = score;
                                win.north = Some(other_windows.id)
                            }
                        }
                        Direction::West => {
                            if best_west > score {
                                best_west = score;
                                win.west = Some(other_windows.id)
                            }
                        }
                        Direction::South => {
                            if best_south > score {
                                best_south = score;
                                win.south = Some(other_windows.id)
                            }
                        }
                        Direction::East => {
                            if best_east > score {
                                best_east = score;
                                win.east = Some(other_windows.id)
                            }
                        }
                    }
                }
            });
        }
        store.insert(window.id, win);
    }
    store
}

pub fn current_window(windows: &[YabaiWindowObject]) -> Option<&YabaiWindowObject> {
    windows.iter().find(|x| x.has_focus)
}
