mod mine_map;
mod player;
mod ray_cast;

use macroquad::prelude::*;
//use egui;
//use egui_macroquad;

use std::f32::consts::PI;
use crate::in_game::mine_map::MapStruct;
use crate::in_game::player::Player;
use crate::in_game::ray_cast::RayCast;

pub struct In_game {
    map: MapStruct,
    player: Player,
    ray_cast: RayCast,
}
impl In_game {
    pub fn new(map_id: i32) -> In_game {
        In_game{
            map: MapStruct::new(map_id),
            player: Player::new(),
            ray_cast: RayCast::new(),
        }
    }

    pub fn events(&mut self) {
        self.player.mouse();
        self.player.keyboard(&self.map);
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

        if self.player.show_fps {
            draw_text(&*format!("FPS: {}", get_fps()), 20.0, 20.0, 23.0, YELLOW);
        }
    }
}