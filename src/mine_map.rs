use macroquad::prelude::*;

pub const Tile_size:f32 = 64.0;

pub enum Actions {
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

pub struct Tile {
    pub is_wall: bool,
    pub render: bool,
    pub color: Color,
    pub action: Actions
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

    pub fn new(id:i32) -> MapStruct {
        MapStruct {
            current_map_id: id,
            map: MapStruct::get_map(id),
        }
    }

    pub fn tile_in_position(&self, position: Vec2) -> &Tile {
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

    pub fn draw(&self){
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