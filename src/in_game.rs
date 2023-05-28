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
use crate::in_game::pause_menu::PauseMenu;
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
    pause_menu: Option<PauseMenu>,
}
impl In_game {
    pub fn new(map_id: i32) -> In_game {
        In_game {
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
            if self.pause_menu.is_some() {
                self.pause_menu.as_mut().unwrap().update(update_state, &mut self.player)
            }
            else { self.pause_menu = Some(PauseMenu::new()) }
        }
    }


    pub fn draw(&mut self) {
        //self.map.draw();
        //self.player.draw();
        //self.ray_cast.draw_rays(&player, &map);

        self.ray_cast.draw(&self.player, &self.map);

        if self.player.is_map_open {
            self.map.mine_map_draw();
            self.player.mine_player_draw(&self.map);
        }

        if self.player.show_menu {
            self.pause_menu.as_ref().unwrap().draw()
        }

        if self.player.show_fps {
            draw_text(&format!("FPS: {}", get_fps()), 20.0, 20.0, 23.0, YELLOW);
        }
    }
}
