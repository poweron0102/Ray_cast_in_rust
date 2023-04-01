mod in_game;
mod in_menu;
mod in_load;

use macroquad::prelude::*;
use std::f32::consts::PI;
use std::ops::Deref;

use crate::in_game::In_game;
use crate::in_load::In_load;
use crate::in_menu::In_menu;

const PI2:f32 = PI/2.0;
const _3PI2:f32 = 3.0 * PI / 2.0;

pub trait Game {
    fn events(&mut self, update_state: &mut Option<Box<dyn Game>>);
    fn draw(&mut self);
    fn change_state(self) -> Box<dyn Game>;
    fn change_state_status(&self) -> bool;
}
impl Game for In_game {
    fn events(&mut self, update_state: &mut Option<Box<dyn Game>>) {
        self.events()
    }

    fn draw(&mut self) {
        self.draw()
    }

    fn change_state(self) -> Box<dyn Game > {
        self.change_state.unwrap()
    }

    fn change_state_status(&self) -> bool {
        if self.change_state.is_some() { return true }
        false
    }
}
impl Game for In_menu {
    fn events(&mut self, update_state: &mut Option<Box<dyn Game>>) {
        self.events();
    }

    fn draw(&mut self) {
        self.draw();
    }

    fn change_state(self) -> Box<dyn Game > {
        self.change_state.unwrap()
    }

    fn change_state_status(&self) -> bool {
        if self.change_state.is_some() { return true }
        false
    }
}


#[macroquad::main("Ray cast")]
async fn main() {
    let mut game: Box<dyn Game> = Box::new(In_menu::new());
    let mut update_state: Option<Box<dyn Game>> = None;

    loop {
        game.events(&mut update_state);

        if game.change_state_status() {
            game = game.change_state();
        }

        //draw_circle(280.0, 60.0, 30.0, RED);

        game.draw();
        next_frame().await
    }
}