mod in_game;
mod in_menu;
mod in_load;
mod in_map_editor;
mod lua;
mod map;

use macroquad::prelude::*;
use std::f32::consts::PI;
use crate::in_game::InGame;
use crate::in_map_editor::InMapEditor;
use crate::in_menu::In_menu;


const PI2:f32 = PI/2.0;
const _3PI2:f32 = 3.0 * PI / 2.0;

pub trait Game {
    fn events(&mut self, update_state: &mut Option<Box<dyn Game>>);
    fn draw(&mut self);
}
impl Game for InGame {
    fn events(&mut self, update_state: &mut Option<Box<dyn Game>>) {
        self.events(update_state)
    }

    fn draw(&mut self) {
        self.draw()
    }

}
impl Game for In_menu {
    fn events(&mut self, update_state: &mut Option<Box<dyn Game>>) {
        self.events(update_state);
    }

    fn draw(&mut self) {
        self.draw();
    }
}
impl Game for InMapEditor {
    fn events(&mut self, update_state: &mut Option<Box<dyn Game>>) {
        InMapEditor::events(self, update_state);
    }

    fn draw(&mut self) {
        InMapEditor::draw(self);
    }
}


#[macroquad::main("Ray cast")]
async fn main() {
    let mut game: Box<dyn Game> = Box::new(In_menu::new());
    let mut update_state: Option<Box<dyn Game>> = None;

    loop {
        game.events(&mut update_state);


        if let Some(new_state) = update_state {
            game = new_state;
            update_state = None;
        }


        game.draw();
        next_frame().await
    }
}