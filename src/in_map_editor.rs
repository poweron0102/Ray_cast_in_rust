mod menu;

use macroquad::prelude::*;
use std::fs;
use serde;
use std::collections::HashSet;
use crate::in_map_editor::menu::MapEditorMenu;
//use std::hash::{Hash, Hasher};

pub const Tile_size: f32 = 64.0;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Actions {
    None,
    Lose,
    NextMap
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Tile {
    pub is_wall: bool,
    pub render: bool,
        color: Color,
    pub visible_color: Color,
    pub action: Actions
}
#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
struct TilePosition {
    x: usize,
    y: usize
}
impl TilePosition {
    fn cord_to_tile_position(cord: Vec2, offset: Vec2) -> TilePosition {
        let real_cord = cord - offset;

        TilePosition{
            x: (real_cord.x / Tile_size) as usize,
            y: (real_cord.y / Tile_size) as usize,
        }
    }
}

const T0:Tile = Tile{ is_wall: false, render: false, color: WHITE, visible_color: WHITE, action: Actions::None };
const T1:Tile = Tile{ is_wall: true,  render: true,  color: GRAY, visible_color: GRAY, action: Actions::None };
const T2:Tile = Tile{ is_wall: true,  render: true,  color: RED, visible_color: RED, action: Actions::None };
const T3:Tile = Tile{ is_wall: true,  render: true,  color: DARKBLUE, visible_color: DARKBLUE, action: Actions::None };
const T4:Tile = Tile{ is_wall: true,  render: true,  color: DARKPURPLE, visible_color: DARKPURPLE, action: Actions::None };
const T5:Tile = Tile{ is_wall: false, render: false, color: GREEN, visible_color: GREEN, action: Actions::NextMap };



pub struct InMapEditor {
    map:[[Tile; 256]; 256],
    selected_tiles: HashSet<TilePosition>,

    in_mapa_editor_menu: MapEditorMenu
}

impl InMapEditor {
    pub fn new() -> InMapEditor {
        InMapEditor {
            map: [[T0;256];256],
            selected_tiles: HashSet::new(),

            in_mapa_editor_menu: MapEditorMenu::new(),
        }
    }


    fn tile_in_position_vec2(&mut self, position: Vec2, offset: Vec2) -> Option<&mut Tile> {
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
    fn tile_in_position_mut(&mut self, position: &TilePosition) -> Option<&mut Tile> {
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


    pub fn events(&mut self) {
        // Reset
        for linha in self.map.iter_mut() {
            for tile in linha {
                tile.visible_color = tile.color
            }
        };
        // Reset
        // Pintar os selecionados
        for selected_tile_position in self.selected_tiles.clone() {
            let selected_tile = self.tile_in_position_mut(&selected_tile_position).unwrap();
            selected_tile.visible_color = BLUE;
            //println!("{:?}", selected_tile_position);
        }
        // Pintar os selecionados

        let mouse = mouse_position();
        let mouse = Vec2::new(mouse.0, mouse.1);


        if let Some(hover_tile) = self.tile_in_position_vec2(mouse,Vec2{ x: 288.0, y: 0.0 }) {
            let tile_hover_position = TilePosition::cord_to_tile_position(mouse, Vec2{ x: 288.0, y: 0.0 });
            hover_tile.visible_color = Color{ r: 0.0, g: 0.0, b: 1.0, a: 0.5, };

            if is_mouse_button_pressed(MouseButton::Left) {
                if self.selected_tiles.contains(&tile_hover_position) {
                    self.selected_tiles.remove(&tile_hover_position);
                }
                else {
                    self.selected_tiles.insert(tile_hover_position.clone());
                }
            }
        };

        MapEditorMenu::update(self)
    }


    pub fn draw(&self) {
        for (row_id, row ) in self.map.iter().enumerate() {
            for (column_id, tile) in row.iter().enumerate() {
                draw_rectangle((column_id as f32) * Tile_size + 8.0 + 288.0,
                               (row_id as f32) * Tile_size + 8.0,
                               Tile_size - 8.0,
                               Tile_size - 8.0,
                               tile.visible_color
                )
            }
        }


        for selected_tile_position in self.selected_tiles.iter() {
            //let selected_tile = self.tile_in_position(&selected_tile_position).unwrap();
            // Text draw
            let text = format!("X:{} Y:{}", selected_tile_position.x, selected_tile_position.y);
            let text_rect = measure_text(&*text, None, 12, 1.0);
            draw_text(&*text,
                      (((selected_tile_position.x as f32) + 0.5) * Tile_size) - (text_rect.width/2.0) + 288.0,
                      (((selected_tile_position.y as f32) + 0.5) * Tile_size) + (text_rect.height/2.0),
                      12.0,
                      BLACK
            )
        }

        self.in_mapa_editor_menu.draw();
    }
}