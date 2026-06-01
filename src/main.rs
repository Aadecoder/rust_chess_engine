use crate::game::Game;

mod board;
mod game;
mod pieces;
mod squares;

fn main() {
    let game = Game::new();
    println!("{:#?}", game);
}
