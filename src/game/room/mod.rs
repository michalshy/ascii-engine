use serde::Deserialize;

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

#[derive(Deserialize)]
pub struct Room {
    pub id: String,
    pub x: u32,
    pub y: u32,
    pub connections: Vec<String>,
    name: String,
    event: Event,
}