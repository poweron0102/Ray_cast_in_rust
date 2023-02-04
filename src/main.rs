mod ray_cast;

//use std::fs::read;
//use std::thread::current;
use macroquad::prelude::*;
//use mouse_rs::{types::keys::Keys, Mouse};


use std::f32::consts::PI;
use crate::ray_cast::RayCast;

const PI2:f32 = PI/2.0;
const _3PI2:f32 = 3.0 * PI / 2.0;

//=-=-=-=-=-=-=-=-=-=-=-Bloco do mapa=-=-=-=-=-=-=-=-=-=-=-=-=-=
const Tile_size:f32 = 64.0;

enum Actions {
    None,
    Lose,
    NextMap
}
enum TileValor {
    Name,
    IsWall,
    Render,
    Color,
    Action
}

struct Tile {
    is_wall: bool,
    render: bool,
    color: Color,
    action: Actions
}

const T0:Tile = Tile{ is_wall: false, render: false, color: BLACK, action: Actions::None };
const T1:Tile = Tile{ is_wall: true,  render: true,  color: GRAY, action: Actions::None };
const T2:Tile = Tile{ is_wall: true,  render: true,  color: RED, action: Actions::None };
const T3:Tile = Tile{ is_wall: true,  render: true,  color: DARKBLUE, action: Actions::None };
const T4:Tile = Tile{ is_wall: true,  render: true,  color: DARKPURPLE, action: Actions::None };
const T5:Tile = Tile{ is_wall: false, render: false, color: GREEN, action: Actions::NextMap };



pub struct MapStruct {
    current_map_id:i32,
    map:[[Tile; 20]; 11]
}

impl MapStruct {

    fn new(id:i32) -> MapStruct {
        MapStruct {
            current_map_id: id,
            map: MapStruct::get_map(id),
        }
    }

    fn tile_in_position(&self, position: Vec2) -> &Tile {
        let x_in_map = (position.x / Tile_size) as usize;
        let y_in_map = (position.y / Tile_size) as usize;
        if self.map.len() <= y_in_map {
            return &T0
        }
        else {
            if self.map[y_in_map].len() <= x_in_map {
                return &T0
            }
            else {
                return  &self.map[y_in_map][x_in_map]
            };
        };
    }

    fn get_map(id:i32) -> [[Tile; 20]; 11]{
        match id {
            0 => [  // Mapa 0
                [T0, T0, T1, T0, T0, T0, T0, T2, T0, T0, T0, T0, T1, T0, T0, T0, T0, T0, T0, T0],
                [T0, T0, T1, T0, T0, T0, T0, T2, T0, T0, T0, T0, T1, T0, T0, T0, T0, T0, T0, T0],
                [T0, T0, T0, T0, T0, T0, T0, T2, T0, T0, T0, T0, T1, T0, T0, T0, T0, T0, T0, T0],
                [T0, T0, T1, T0, T0, T0, T0, T0, T0, T0, T0, T0, T1, T0, T0, T0, T0, T0, T0, T0],
                [T0, T0, T1, T0, T0, T2, T3, T3, T3, T2, T0, T0, T0, T0, T0, T0, T0, T0, T0, T0],
                [T0, T0, T1, T0, T0, T2, T0, T0, T0, T2, T0, T1, T1, T1, T1, T1, T0, T0, T0, T0],
                [T0, T0, T0, T0, T0, T1, T0, T4, T0, T1, T0, T0, T0, T1, T0, T0, T0, T0, T0, T0],
                [T0, T0, T1, T0, T0, T1, T0, T0, T0, T1, T0, T0, T0, T1, T0, T0, T0, T0, T0, T0],
                [T0, T0, T1, T0, T0, T1, T1, T0, T1, T1, T0, T0, T0, T1, T0, T0, T0, T0, T0, T0],
                [T0, T0, T1, T0, T0, T1, T0, T0, T0, T1, T0, T0, T0, T2, T0, T0, T0, T0, T0, T0],
                [T0, T0, T1, T0, T0, T0, T0, T0, T0, T1, T0, T0, T0, T0, T0, T0, T0, T0, T0, T0],
            ],
            1 => [  // Mapa 1
                [T0, T0, T1, T0, T0, T0, T0, T2, T0, T0, T0, T0, T1, T0, T0, T0, T0, T0, T1, T1],
                [T0, T0, T1, T0, T0, T0, T0, T0, T0, T0, T0, T0, T1, T0, T0, T0, T0, T0, T0, T0],
                [T0, T0, T0, T0, T0, T0, T0, T2, T0, T0, T0, T0, T1, T0, T0, T0, T0, T0, T0, T0],
                [T0, T0, T1, T0, T0, T0, T0, T0, T0, T0, T0, T0, T1, T0, T0, T0, T0, T0, T0, T0],
                [T0, T0, T0, T0, T0, T0, T0, T0, T0, T0, T0, T0, T0, T0, T0, T0, T0, T0, T0, T0],
                [T0, T0, T0, T0, T0, T2, T0, T0, T0, T2, T0, T1, T1, T1, T1, T1, T0, T0, T0, T0],
                [T0, T0, T0, T0, T0, T1, T0, T0, T0, T1, T1, T1, T0, T1, T0, T0, T0, T0, T0, T0],
                [T0, T0, T0, T0, T0, T1, T0, T0, T0, T1, T0, T1, T0, T1, T0, T0, T0, T0, T0, T0],
                [T0, T0, T1, T0, T0, T1, T1, T0, T1, T1, T0, T0, T0, T1, T0, T0, T0, T0, T0, T4],
                [T0, T0, T1, T0, T0, T1, T0, T0, T0, T1, T0, T0, T0, T1, T0, T0, T0, T0, T0, T0],
                [T0, T0, T1, T0, T0, T0, T0, T0, T0, T1, T0, T0, T0, T0, T0, T0, T0, T0, T0, T5],
            ],
            _ => [  // Mapa 2
                [T0, T0, T1, T0, T0, T0, T0, T2, T0, T3, T3, T3, T1, T2, T1, T0, T0, T0, T1, T1],
                [T0, T0, T1, T0, T0, T0, T0, T0, T0, T3, T0, T0, T1, T0, T0, T0, T0, T0, T0, T0],
                [T0, T0, T0, T0, T0, T0, T0, T2, T0, T3, T0, T0, T0, T0, T0, T0, T0, T0, T0, T0],
                [T0, T0, T1, T0, T0, T0, T3, T3, T3, T2, T0, T0, T0, T0, T0, T0, T0, T0, T0, T0],
                [T0, T0, T0, T0, T0, T0, T0, T0, T0, T0, T0, T0, T0, T0, T0, T0, T0, T0, T0, T0],
                [T0, T0, T0, T0, T0, T3, T0, T0, T0, T2, T0, T1, T1, T1, T1, T1, T0, T0, T0, T0],
                [T0, T0, T0, T0, T0, T1, T0, T0, T0, T0, T1, T1, T0, T1, T0, T0, T0, T0, T0, T0],
                [T0, T0, T0, T0, T0, T1, T0, T0, T0, T0, T0, T0, T0, T1, T5, T0, T0, T0, T0, T0],
                [T0, T0, T1, T0, T0, T1, T1, T0, T1, T1, T0, T0, T0, T1, T0, T0, T0, T0, T0, T4],
                [T0, T0, T1, T0, T0, T1, T0, T0, T0, T1, T0, T0, T0, T1, T0, T0, T0, T0, T0, T0],
                [T0, T0, T1, T0, T0, T0, T0, T0, T0, T1, T0, T0, T0, T0, T0, T0, T0, T0, T0, T5],
            ]
        }
    }

    fn draw(&self){
        for (row_id, row ) in self.map.iter().enumerate() {
            for (column_id, tile) in row.iter().enumerate() {
                draw_rectangle((column_id as f32) * Tile_size,
                               (row_id as f32) * Tile_size,
                               Tile_size,
                               Tile_size,
                               tile.color
                )
            }
        }
    }
}
//=-=-=-=-=-=-=-=-=-=-=-Bloco do mapa=-=-=-=-=-=-=-=-=-=-=-=-=-=
//=-=-=-=-=-=-=-=-=-=-=-Bloco do player=-=-=-=-=-=-=-=-=-=-=-=-=
const MOUSE_SEN:f32 = 4.0;

fn sig(num:f32) -> f32 {
    if num < 0.0 {
        return -1.0
    }else {
        return 1.0
    }
}

fn normalize_angle(angle: f32) -> f32 {
    let mut result = angle;
    while result < 0.0 {
        result += 2.0 * PI;
    }
    while result >= 2.0 * PI {
        result -= 2.0 * PI;
    }
    result
}

pub struct Player {
    locate: Vec2,
    speed: f32,
    angle:f32,
    size:f32,
    is_map_open:bool
}
impl Player {
    fn new() -> Player {
        Player {
            locate: vec2(50.0, 50.0),
            speed: 300.0,
            angle: 0.0,
            size: 15.0,
            is_map_open: false
        }
    }

    fn forward_pont(&self, size:f32) -> Vec2 {
        Vec2{
            x: self.locate.x + (size * f32::cos(self.angle)),
            y: self.locate.y + (size * f32::sin(self.angle)),
        }
    }

    fn draw(&self) {
        draw_circle(self.locate.x,
                    self.locate.y,
                    16.0,
                    GREEN
        );

        let forward = self.forward_pont(16.0);
        draw_line(self.locate.x,
                  self.locate.y,
                  forward.x,
                  forward.y,
                  4.0,
                  PINK
        );
    }

    fn keyboard(&mut self, map:&MapStruct) {
        //-=-=-=-=-=-=-=-=- Keyboard -=-=-=-=-=-=-=-=-=-=-=-
        let relative_speed = self.speed * get_frame_time();
        let mut delta = self.locate;

        if is_key_down(KeyCode::W) {
            delta = self.locate;
            delta.x += relative_speed * f32::cos(self.angle);
            if !map.tile_in_position(delta).is_wall { self.locate = delta };

            delta = self.locate;
            delta.y += relative_speed * f32::sin(self.angle);
            if !map.tile_in_position(delta).is_wall { self.locate = delta };
        }
        if is_key_down(KeyCode::A) {
            delta = self.locate;
            delta.x += -relative_speed * f32::sin(self.angle + PI);
            if !map.tile_in_position(delta).is_wall { self.locate = delta };

            delta = self.locate;
            delta.y += relative_speed * f32::cos(self.angle + PI);
            if !map.tile_in_position(delta).is_wall { self.locate = delta };
        }
        if is_key_down(KeyCode::S) {
            delta = self.locate;
            delta.x += relative_speed * f32::sin(self.angle + (PI2 * 3.0));
            if !map.tile_in_position(delta).is_wall { self.locate = delta };

            delta = self.locate;
            delta.y += -relative_speed * f32::cos(self.angle + (PI2 * 3.0));
            if !map.tile_in_position(delta).is_wall { self.locate = delta };
        }
        if is_key_down(KeyCode::D) {
            delta = self.locate;
            delta.x += relative_speed * f32::cos(self.angle + PI2);
            if !map.tile_in_position(delta).is_wall { self.locate = delta };

            delta = self.locate;
            delta.y += relative_speed * f32::sin(self.angle + PI2);
            if !map.tile_in_position(delta).is_wall { self.locate = delta };
        }
        //println!("A posição do jogador é: {:?}", delta);
        //-=-=-=-=-=-=-=-=- Keyboard -=-=-=-=-=-=-=-=-=-=-=-
    }

    fn mouse(&mut self) {
        self.angle -= mouse_delta_position().x * MOUSE_SEN;
        self.angle = normalize_angle(self.angle);
        //keep_mouse_centered();
    }
}
//=-=-=-=-=-=-=-=-=-=-=-Bloco do player=-=-=-=-=-=-=-=-=-=-=-=-=
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

        map.draw();
        player.draw();
        ray_cast.draw_rays(&player, &map);
        //ray_cast.draw(&player, &map);

        fps_text = format!("FPS: {}", get_fps());
        draw_text(&fps_text, 20.0, 20.0, 23.0, YELLOW);
        next_frame().await
    }
}