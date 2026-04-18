use arcanna::game;

fn main() {
    game::Game::new("stub_map.json").run().expect("Run failed");
}
