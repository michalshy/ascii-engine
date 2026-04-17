mod map;

use map::Map;

pub struct Game {
    map: Map,
}

impl Game {
    pub fn new(path: &str) -> Game {
        let loaded_map = Map::load(path).expect("Map didn't load correctly");
        
        Game { map: loaded_map }
    }

    pub fn run(&self) {
        self.map.render();
    }
}