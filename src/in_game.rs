mod mine_map;
mod player;
mod ray_cast;
mod pause_menu;

use macroquad::prelude::*;
//use egui;
//use egui_macroquad;

use std::f32::consts::PI;
use macroquad::input::KeyCode::Menu;
use Simples_menu::{Button, MenuElement};
use Simples_menu::PositionType::TopLeft;
use crate::Game;
use crate::in_game::mine_map::MapStruct;
use crate::in_game::player::Player;
use crate::in_game::ray_cast::RayCast;
use crate::in_menu::In_menu;

impl Clone for In_game {
    fn clone(&self) -> Self {
        In_game{
            map: self.map,
            player: self.player,
            ray_cast: self.ray_cast.clone(),
            pause_menu: None,
        }
    }
}

pub struct In_game {
    map: MapStruct,
    player: Player,
    ray_cast: RayCast,
    pause_menu: Option<Simples_menu::Menu>,
}
impl In_game {
    pub fn new(map_id: i32) -> In_game {
        In_game{
            map: MapStruct::new(map_id),
            player: Player::new(),
            ray_cast: RayCast::new(),
            pause_menu: None,
        }
    }

    pub fn events(&mut self, update_state: &mut Option<Box<dyn Game>>) {
        self.player.mouse();
        self.player.keyboard(&self.map);


        if self.player.show_menu {
            let mut pause_menu = Simples_menu::Menu::new("Pause menu".to_string(), vec2(0.0, 0.0));
            let menu_rect = pause_menu.bounding_rect().unwrap();

            pause_menu.position = Vec2{
                x: screen_width() - (menu_rect.x / 2.0),
                y: screen_height() - (menu_rect.y / 2.0)
            };
            let continue_b = pause_menu.add_element(Button::new("Continue".to_string(), TopLeft, Vec2{ x: 30.0, y: 0.0 }, None));
            let main_menu_b = pause_menu.add_element(Button::new("Return to main menu".to_string(), TopLeft,Vec2{ x: 20.0, y: 30.0 }, None));


            pause_menu.update();
            if main_menu_b.read().has_been_pressed {
                *update_state = Some(Box::new(In_menu::new()));
            }
            pause_menu.draw();
        }
    }

    pub fn draw(&mut self) {
        //self.map.draw();
        //self.player.draw();
        //self.ray_cast.draw_rays(&player, &map);
        if self.player.show_menu {

        }
        self.ray_cast.draw(&self.player, &self.map);

        if self.player.is_map_open {
            self.map.mine_map_draw();
            self.player.mine_player_draw(&self.map);
        }

        if self.player.show_fps {
            draw_text(&*format!("FPS: {}", get_fps()), 20.0, 20.0, 23.0, YELLOW);
        }
    }
}