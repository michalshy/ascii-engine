use serde::Deserialize;

#[derive(Deserialize, PartialEq, PartialOrd)]
pub struct Coords {                                       
    pub x: u32,
    pub y: u32,                                           
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

#[derive(Deserialize)]
pub struct Room {
    pub id: String,
    pub name: String,
    pub pos: Coords,
    pub description: String,
    pub connections: Vec<String>,
    event: Event,
}