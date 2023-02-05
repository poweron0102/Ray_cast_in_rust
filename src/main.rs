mod ray_cast;
mod mine_map;
mod player;

//use std::fs::read;
//use std::thread::current;
use macroquad::prelude::*;
//use mouse_rs::{types::keys::Keys, Mouse};


use std::f32::consts::PI;
use crate::mine_map::MapStruct;
use crate::player::Player;
//use crate::mine_map::MapStruct;
use crate::ray_cast::RayCast;

const PI2:f32 = PI/2.0;
const _3PI2:f32 = 3.0 * PI / 2.0;


/*fn mouse_get_delta(last_position:&mut Vec2) -> (Vec2, Vec2) {
    let current_position = mouse_position_local();
    let delta = *last_position - current_position;

    (delta, current_position)
}*/

#[macroquad::main("Ray cast")]
async fn main() {
    //set_cursor_grab(true);
    let mut fps_text = String::new();


    let mut map = MapStruct::new(0);
    let mut player = Player::new();
    let mut ray_cast = RayCast::new(&player, &map);
    loop {
        player.mouse();
        player.keyboard(&map);

        //map.draw();
        //player.draw();
        //ray_cast.draw_rays(&player, &map);
        ray_cast.draw(&player, &map);

        fps_text = format!("FPS: {}", get_fps());
        draw_text(&fps_text, 20.0, 20.0, 23.0, YELLOW);
        next_frame().await
    }
}