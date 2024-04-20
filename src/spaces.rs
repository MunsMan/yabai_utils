use crate::clap::DirectionOrIndex;
use crate::yabai::{query_spaces, yabai_create_space, yabai_focus_space};

pub type SpaceIndex = u8;

pub fn focus_space(direction_or_index: &DirectionOrIndex) {
    let spaces_infos = query_spaces();
    let current_space = spaces_infos.iter().find(|x| x.has_focus).unwrap();
    let num_spaces = spaces_infos.len() as u8;

    match direction_or_index {
        DirectionOrIndex::Left => {
            if current_space.index > 2 {
                yabai_focus_space(current_space.index - 1)
            } else {
                yabai_focus_space(num_spaces)
            }
        }
        DirectionOrIndex::Right => {
            if current_space.index == num_spaces {
                yabai_focus_space(1)
            } else {
                yabai_focus_space(current_space.index + 1)
            }
        }
        DirectionOrIndex::Index(index) => {
            if index <= &num_spaces {
                yabai_focus_space(*index)
            } else {
                for _ in num_spaces..*index {
                    yabai_create_space()
                }
                yabai_focus_space(*index)
            }
        }
    }
}
