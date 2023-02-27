mod in_game;
mod in_menu;

use macroquad::prelude::*;
use std::f32::consts::PI;

use crate::in_game::In_game;

const PI2:f32 = PI/2.0;
const _3PI2:f32 = 3.0 * PI / 2.0;
#[macroquad::main("Ray cast")]
async fn main() {
    let mut game = In_game::new(0);

    loop {
        game.events();

        game.draw();

        next_frame().await
    }
}