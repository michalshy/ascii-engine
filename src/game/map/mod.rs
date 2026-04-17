use serde::Deserialize;

#[derive(Deserialize)]
pub struct Map {
    rooms: Vec<Room>
}

#[derive(Deserialize)]
pub struct Room {
    id: String,
    name: String,
    x: u32,
    y: u32,
    event: Event,
    connections: Vec<String>
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Event {
    Npc {
        name: String,
        dialogue: String,
    },
    Enemy {
        name: String,
        health: u32,
        damage: u32,
        description: String,
    },
    Item {
        name: String,
        description: String,
    },
    Nothing,
}

impl Map {
    pub fn new() -> Map {
        Map{ rooms: Vec::new() }
    }

    pub fn load(path: &str) -> anyhow::Result<Map> {
        let content = std::fs::read_to_string(path)?;
        let map: Map = serde_json::from_str(&content)?;
        Ok(map)
    }

    pub fn render(&self) {

    }
}