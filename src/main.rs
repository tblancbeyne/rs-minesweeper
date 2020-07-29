use gol::game::Game;

fn main() {
    let mut game = Game::new(10, 10, 20, 10);

    game.run();
}
