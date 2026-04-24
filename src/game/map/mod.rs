use serde::Deserialize;
use crate::game::room::{Coords, Room};

enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST
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


}

