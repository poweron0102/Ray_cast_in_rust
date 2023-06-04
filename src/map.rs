use std::ops::{Deref, DerefMut};
use macroquad::prelude::*;
use serde::*;

pub const Tile_size: f32 = 64.0;

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq, Serialize, Deserialize)]
pub struct TilePosition {
    pub x: usize,
    pub y: usize
}
impl TilePosition {
    pub fn cord_to_tile_position(cord: Vec2, offset: Vec2) -> TilePosition {
        let real_cord = cord - offset;

        TilePosition{
            x: (real_cord.x / Tile_size) as usize,
            y: (real_cord.y / Tile_size) as usize,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tile {
    pub is_wall: bool,
    pub render: bool,
    pub color: Color,
    pub visible_color: Color,
    pub action: Option<&'static str>
}

const T0:Tile = Tile{ is_wall: false, render: false, color: WHITE, visible_color: WHITE, action: None };
const T1:Tile = Tile{ is_wall: true,  render: true,  color: GRAY, visible_color: GRAY, action: None };
const T2:Tile = Tile{ is_wall: true,  render: true,  color: RED, visible_color: RED, action: None };
const T3:Tile = Tile{ is_wall: true,  render: true,  color: DARKBLUE, visible_color: DARKBLUE, action: None };
const T4:Tile = Tile{ is_wall: true,  render: true,  color: DARKPURPLE, visible_color: DARKPURPLE, action: None };
const T5:Tile = Tile{ is_wall: false, render: false, color: GREEN, visible_color: GREEN, action: None };

pub struct WordMap {
    //pub map: [[Tile; 64]; 64],
    pub map: Vec<Vec<Tile>>
}
impl WordMap {
    pub fn new() -> WordMap {
        WordMap{
            //map: [[T0; 64]; 64]
            map: vec![vec![T0; 32]; 32]
        }
    }

    pub fn tile_in_position_vec2(&mut self, position: Vec2, offset: Vec2) -> Option<&mut Tile> {
        if position.x < offset.x || position.y < offset.y {
            return None
        }

        let x_in_map:usize;
        let y_in_map:usize;
        {
            let temp = TilePosition::cord_to_tile_position(position, offset);
            x_in_map = temp.x;
            y_in_map = temp.y;
        }

        return if self.map.len() <= y_in_map {
            None
        } else {
            if self.map[y_in_map].len() <= x_in_map {
                None
            } else {
                let tile = &mut self.map[y_in_map][x_in_map];
                Some(tile)
            }
        };
    }
    pub fn tile_in_position_mut(&mut self, position: &TilePosition) -> Option<&mut Tile> {
        return if self.map.len() <= position.y {
            None
        } else {
            if self.map[position.y].len() <= position.x {
                None
            } else {
                let tile = &mut self.map[position.y][position.x];
                Some(tile)
            }
        }
    }
    fn tile_in_position(&self, position: &TilePosition) -> Option<&Tile> {
        return if self.map.len() <= position.y {
            None
        } else {
            if self.map[position.y].len() <= position.x {
                None
            } else {
                let tile = &self.map[position.y][position.x];
                Some(tile)
            }
        }
    }
}



