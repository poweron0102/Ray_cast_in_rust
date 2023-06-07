use std::cell::Ref;
use macroquad::prelude::*;
use crate::map::{Tile, WordMap};

pub const Tile_size:f32 = 64.0;
const MINE_MAP_ZOON:f32 = 16.0;
const T0:Tile = Tile{ is_wall: false, render: false, color: BLANK, visible_color: BLANK, step_action: None, look_action: None, render_action: None };

#[derive(Debug, Clone)]
pub struct MineMap {
    map: WordMap
}
impl AsRef<Vec<Vec<Tile>>> for WordMap {
    fn as_ref(&self) -> &Vec<Vec<Tile>> {
        &self.map
    }
}
impl AsMut<Vec<Vec<Tile>>> for WordMap {
    fn as_mut(&mut self) -> &mut Vec<Vec<Tile>> {
        &mut self.map
    }
}

impl MineMap {

    pub fn new(map_name: &str) -> MineMap {
        MineMap {
            map: WordMap::new_from_map_save(map_name),
        }
    }


    /*fn get_map(id:i32) -> [[Tile; 20]; 11]{
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
    }*/

    pub fn tile_in_position(&self, position: Vec2) -> &Tile {
        &self.map.tile_in_position_vec2(position, Vec2::new(0.0, 0.0)).unwrap_or(&T0)
    }


    pub fn draw(&self) {
        for (row_id, row ) in self.map.as_ref().iter().enumerate() {
            for (column_id, tile) in row.iter().enumerate() {
                draw_rectangle((column_id as f32) * Tile_size,
                               (row_id as f32) * Tile_size,
                               Tile_size,
                               Tile_size,
                               tile.visible_color
                )
            }
        }
    }

    pub fn to_mine_map(&self, point: Vec2) -> Vec2 {
        let mine_map_size = vec2((self.map.as_ref()[0].len() as f32) * MINE_MAP_ZOON,
                                 (self.map.as_ref().len() as f32) * MINE_MAP_ZOON);
        let distance = vec2(20.0, 20.0);

        Vec2{
            x: (point.x * MINE_MAP_ZOON / Tile_size) + distance.x,
            y: (screen_height() - mine_map_size.y - distance.y) + (point.y * MINE_MAP_ZOON / Tile_size)
        }
    }

    pub fn mine_map_draw(&self) {
        let mine_map_size = vec2((self.map.as_ref()[0].len() as f32) * MINE_MAP_ZOON,
                             (self.map.as_ref().len() as f32) * MINE_MAP_ZOON);
        let distance = vec2(20.0, 20.0);
        let border_radius = 4.0;
        //println!("{:?}", mine_map_size);

        draw_rectangle(distance.x - border_radius,
                       screen_height() - mine_map_size.y - distance.y - border_radius,
                       mine_map_size.x + (border_radius * 2.0),
                       mine_map_size.y + (border_radius * 2.0),
                       WHITE
        );


        for (row_id, row ) in self.map.as_ref().iter().enumerate() {
            for (column_id, tile) in row.iter().enumerate() {
                draw_rectangle(((column_id as f32) * MINE_MAP_ZOON) + distance.x ,
                               (screen_height() - mine_map_size.y - distance.y) + ((row_id as f32) * MINE_MAP_ZOON),
                               MINE_MAP_ZOON,
                               MINE_MAP_ZOON,
                               tile.visible_color
                )
            }
        }


    }
}