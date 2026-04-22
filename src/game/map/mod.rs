use serde::Deserialize;
use crate::game::Room;

#[derive(Deserialize)]
pub struct Map {
    pub rooms: Vec<Room>
}

impl Map {
    pub fn load(path: &str) -> anyhow::Result<Map> {
        let content = std::fs::read_to_string(path)?;
        let map: Map = serde_json::from_str(&content)?;
        Ok(map)
    }
}

