mod in_game;
mod in_menu;
mod in_load;

use macroquad::prelude::*;
use std::f32::consts::PI;

use crate::in_game::In_game;
use crate::in_load::In_load;
use crate::in_menu::In_menu;

const PI2:f32 = PI/2.0;
const _3PI2:f32 = 3.0 * PI / 2.0;

trait Game {
    fn events(&mut self);
    fn draw(&mut self);
}
impl Game for In_game {
    fn events(&mut self) {
        self.events()
    }

    fn draw(&mut self) {
        self.draw()
    }
}


#[macroquad::main("Ray cast")]
async fn main() {
    let mut game:Box<dyn Game> = Box::new(In_game::new(0));

    loop {
        game.events();

        game.draw();
        next_frame().await
    }
}