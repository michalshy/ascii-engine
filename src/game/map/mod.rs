use serde::Deserialize;
use crate::game::room::{Coords, Room};

enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST
}

fn StrToDir(dir: String) -> Option<Direction> {
    match dir.to_lowercase().as_str() {
        "north" => Some(Direction::NORTH),
        "east" => Some(Direction::EAST),
        "south" => Some(Direction::SOUTH),
        "west" => Some(Direction::WEST),
        "n" => Some(Direction::NORTH),
        "e" => Some(Direction::EAST),
        "s" => Some(Direction::SOUTH),
        "w" => Some(Direction::WEST),
        _ => return None
    }
}

#[derive(Deserialize)]
pub struct Map {
    pub pos: Coords,
    pub rooms: Vec<Room>,
}

impl Map {
    pub fn load(path: &str) -> anyhow::Result<Map> {
        let content = std::fs::read_to_string(path)?;
        let map: Map = serde_json::from_str(&content)?;
        Ok(map)
    }

    pub fn get_current_room(&self) -> Option<&Room> {  
        self.rooms.iter().find(|r| r.pos == self.pos)      
    }

    pub fn change_room(&mut self, dir: String) {
        let direction = StrToDir(dir);
        if direction.is_none() {
            return
        }

        match direction.unwrap() {
            Direction::NORTH => self.go(self.pos.x, self.pos.y - 1),
            Direction::EAST => self.go(self.pos.x - 1, self.pos.y),
            Direction::SOUTH => self.go(self.pos.x, self.pos.y + 1),
            Direction::WEST => self.go(self.pos.x + 1, self.pos.y),
        }
    }

    fn go(&mut self, x: i32, y: i32) {
        let potential_pos = Coords { x, y };
        let potential_room = self.rooms.iter().find(|r| r.pos == potential_pos);
        if potential_room.is_some() {
            self.pos = potential_pos;
        }
    }
}

