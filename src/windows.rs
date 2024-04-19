use std::cmp::Ordering;
use std::collections::HashMap;
use std::f64::consts::PI;
use std::ops::Sub;

use crate::yabai::YabaiWindowObject;

pub type WindowId = usize;

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Direction {
    North,
    West,
    South,
    East,
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

impl Positon {
    pub fn abs(&self) -> usize {
        (self.x.powi(2) + self.y.powi(2)).sqrt() as usize
    }

    pub fn direction(&self) -> f64 {
        let rad = self.y.atan2(self.x);
        if self.x >= 0.0 {
            let degree = rad * 180.0 / PI;
            if degree < 0.0 {
                360.0 + degree
            } else {
                degree
            }
        } else if self.y >= 0.0 {
            rad * 180.0 / PI
        } else {
            360.0 + rad * 180.0 / PI
        }
    }
}

#[derive(Debug)]
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
        for other_window in windows {
            if window.id == other_window.id {
                continue;
            }
            let directions = window.frame.direction(&other_window.frame);
            directions.into_iter().for_each(|direction| {
                if let Some((score, direction)) = direction {
                    match direction {
                        Direction::North => {
                            if let Ordering::Greater = best_north.cmp(&score) {
                                best_north = score;
                                win.north = Some(other_window.id)
                            }
                        }
                        Direction::West => match best_west.cmp(&score) {
                            Ordering::Greater => {
                                best_west = score;
                                win.west = Some(other_window.id)
                            }
                            Ordering::Equal => todo!(),
                            _ => {}
                        },
                        Direction::South => {
                            if best_south > score {
                                best_south = score;
                                win.south = Some(other_window.id)
                            }
                        }
                        Direction::East => match best_east.cmp(&score) {
                            Ordering::Greater => {
                                best_east = score;
                                win.east = Some(other_window.id)
                            }
                            Ordering::Equal => {
                                if let Some(current_best_window_id) = win.east {
                                    let current_best_window = windows
                                        .iter()
                                        .find(|x| x.id == current_best_window_id)
                                        .unwrap();
                                    if current_best_window.frame.y > other_window.frame.y {
                                        win.east = Some(other_window.id)
                                    }
                                } else {
                                    win.east = Some(other_window.id)
                                }
                            }
                            _ => {}
                        },
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_direction() {
        for i in 0..360 {
            let x = (i as f64 * PI / 180.0).cos();
            let y = (i as f64 * PI / 180.0).sin();
            let position = Positon { x, y };
            println!("{i}- ({},{}) {}", x, y, position.direction());
            assert!((position.direction() - i as f64).abs() <= 1.0);
        }
    }
}
