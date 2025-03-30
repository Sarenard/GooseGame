mod game;

use game::{
    game::Game as Game
};

fn main() {
    let mut game = Game::new();

    game.init();

    game.run();

    game.last();
}
