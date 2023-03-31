mod in_game;
mod in_menu;
mod in_load;

use macroquad::prelude::*;
use std::f32::consts::PI;
use crate::Game::InGame;

use crate::in_game::In_game;
use crate::in_load::In_load;
use crate::in_menu::In_menu;

const PI2:f32 = PI/2.0;
const _3PI2:f32 = 3.0 * PI / 2.0;

enum Game {
    InGame(In_game),
    InMenu(In_menu),
    InLoad(In_load),
}


#[macroquad::main("Ray cast")]
async fn main() {
    let mut game = InGame(In_game::new(0));

    loop {
        match &mut game {
            Game::InGame(game) => { game.events(); game.draw() }
            Game::InMenu(game) => { }//game.events(); game.draw() }
            Game::InLoad(game)  => { }//game.events(); game.draw() }
        };


        next_frame().await
    }
}