use crate::game::room::{self, Room};

pub struct Story {
    pub dialogue_line: String,
    pub history: Vec<String>,
    room_introduced: bool,
    room_changed: bool,
    letter_idx: u32,
}

impl Story {
    pub fn new() -> Story {
        return Self { dialogue_line: String::new(), history: Vec::new(), room_introduced: false, room_changed: true, letter_idx: 0 };
    }

    pub fn update(&mut self, current_room: &Room) {
        if self.room_changed {
            self.room_changed = false;
            self.dialogue_line.clear();
        }
        if !self.room_introduced {
            let to_append = current_room.description.chars().nth(self.dialogue_line.len());
            if to_append.is_some() {
                self.dialogue_line.push(to_append.unwrap());
            } 
            else {
                self.room_introduced = true;
            }
        }
    }

    pub fn get_current_dialogue(&self) -> &String {
        &self.dialogue_line
    }
}