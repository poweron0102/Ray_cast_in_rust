use std::{fs};
use macroquad::prelude::*;
use serde::*;
use std::fs::File;
use std::io::{Read};
use std::path::Path;


pub const TILE_SIZE: f32 = 64.0;

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq, Serialize, Deserialize)]
pub struct TilePosition {
    pub x: usize,
    pub y: usize
}
impl TilePosition {
    pub fn cord_to_tile_position(cord: Vec2, offset: Vec2) -> TilePosition {
        let real_cord = cord - offset;

        TilePosition{
            x: (real_cord.x / TILE_SIZE) as usize,
            y: (real_cord.y / TILE_SIZE) as usize,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tile {
    pub is_wall: bool,
    pub render: bool,
    pub color: Color,
    pub visible_color: Color,
    pub step_action: Option<String>,
    pub look_action: Option<String>,
    pub render_action: Option<String>
}

const T0:Tile = Tile{ is_wall: false, render: false, color: BLANK, visible_color: BLANK, step_action: None, look_action: None, render_action: None };
const T1:Tile = Tile{ is_wall: true,  render: true,  color: GRAY, visible_color: GRAY, step_action: None, look_action: None, render_action: None };
const T2:Tile = Tile{ is_wall: true,  render: true,  color: RED, visible_color: RED, step_action: None, look_action: None, render_action: None };
const T3:Tile = Tile{ is_wall: true,  render: true,  color: DARKBLUE, visible_color: DARKBLUE, step_action: None, look_action: None, render_action: None };
const T4:Tile = Tile{ is_wall: true,  render: true,  color: DARKPURPLE, visible_color: DARKPURPLE, step_action: None, look_action: None, render_action: None };
const T5:Tile = Tile{ is_wall: false, render: false, color: GREEN, visible_color: GREEN, step_action: None, look_action: None, render_action: None };

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WordMap {
    //pub map: [[Tile; 64]; 64],
    pub map: Vec<Vec<Tile>>
}
impl WordMap {
    pub fn new_empty() -> WordMap {
        WordMap{
            map: vec![vec![T0; 32]; 32]
            /*
            map: vec![  // Mapa 0
                        vec![T0, T0, T1, T0, T0, T0, T0, T2, T0, T0, T0, T0, T1, T0, T0, T0, T0, T0, T0, T0],
                        vec![T0, T0, T1, T0, T0, T0, T0, T2, T0, T0, T0, T0, T1, T0, T0, T0, T0, T0, T0, T0],
                        vec![T0, T0, T0, T0, T0, T0, T0, T2, T0, T0, T0, T0, T1, T0, T0, T0, T0, T0, T0, T0],
                        vec![T0, T0, T1, T0, T0, T0, T0, T0, T0, T0, T0, T0, T1, T0, T0, T0, T0, T0, T0, T0],
                        vec![T0, T0, T1, T0, T0, T2, T3, T3, T3, T2, T0, T0, T0, T0, T0, T0, T0, T0, T0, T0],
                        vec![T0, T0, T1, T0, T0, T2, T0, T0, T0, T2, T0, T1, T1, T1, T1, T1, T0, T0, T0, T0],
                        vec![T0, T0, T0, T0, T0, T1, T0, T4, T0, T1, T0, T0, T0, T1, T0, T0, T0, T0, T0, T0],
                        vec![T0, T0, T1, T0, T0, T1, T0, T0, T0, T1, T0, T0, T0, T1, T0, T0, T0, T0, T0, T0],
                        vec![T0, T0, T1, T0, T0, T1, T1, T0, T1, T1, T0, T0, T0, T1, T0, T0, T0, T0, T0, T0],
                        vec![T0, T0, T1, T0, T0, T1, T0, T0, T0, T1, T0, T0, T0, T2, T0, T0, T0, T0, T0, T0],
                        vec![T0, T0, T1, T0, T0, T0, T0, T0, T0, T1, T0, T0, T0, T0, T0, T0, T0, T0, T0, T0],
            ]
             */
        }
    }


    pub fn save_map(&self, file_name: &str) {
        let path_str = "./save/maps/".to_string() + file_name;
        let file_dir = Path::new(&path_str);

        let serded_string = serde_json::to_string(self).unwrap();
        fs::write(file_dir, serded_string)
            .expect(&format!("The diretory: \"{path_str}\" :"));
    }


    pub fn new_from_map_save(file_name: &str) -> WordMap {
        let path_str = "./save/maps/".to_string() + file_name;
        let file_dir = Path::new(&path_str);

        let mut file= "".to_string();
        File::open(file_dir).unwrap().read_to_string(&mut file)
            .expect(&format!("The diretory: \"{path_str}\" :"));

        serde_json::from_str(&file).unwrap()
    }

    pub fn tile_in_position_vec2_mut(&mut self, position: Vec2, offset: Vec2) -> Option<&mut Tile> {
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
    pub fn tile_in_position_vec2(&self, position: Vec2, offset: Vec2) -> Option<&Tile> {
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
                let tile = &self.map[y_in_map][x_in_map];
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



