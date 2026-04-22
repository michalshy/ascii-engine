use serde::Deserialize;
use crate::game::Room;

#[derive(Deserialize)]
    pub struct Coords {                                       
        pub x: u32,
        pub y: u32,                                           
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
        self.rooms.iter().find(|r| r.x == self.pos.x && r.y == self.pos.y)      
    }
}

