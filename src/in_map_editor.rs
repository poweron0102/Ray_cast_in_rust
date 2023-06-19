use std::collections::HashSet;


use macroquad::prelude::*;

use crate::Game;

use crate::in_map_editor::menu::MapEditorMenu;
use crate::map::{TilePosition, WordMap};

mod menu;

//use std::hash::{Hash, Hasher};

pub const TILE_SIZE: f32 = 64.0;

pub struct InMapEditor {
    pub map: WordMap,

    pub selected_tiles: HashSet<TilePosition>,
    pub in_mapa_editor_menu: MapEditorMenu,
}
impl InMapEditor {
    pub fn new() -> InMapEditor {
        InMapEditor {
            selected_tiles: HashSet::new(),
            map: WordMap::new_empty(),

            in_mapa_editor_menu: MapEditorMenu::new(),
        }
    }



    pub fn events(&mut self, update_state: &mut Option<Box<dyn Game>>) {
        // Reset
        for linha in self.map.map.iter_mut() {
            for tile in linha {
                tile.visible_color = tile.color
            }
        };
        // Reset
        // Pintar os selecionados
        for selected_tile_position in self.selected_tiles.clone() {
            let selected_tile = self.map.tile_in_position_mut(&selected_tile_position).unwrap();
            selected_tile.visible_color = BLUE;
        }
        // Pintar os selecionados

        let mouse = mouse_position();
        let mouse = Vec2::new(mouse.0, mouse.1);


        if let Some(hover_tile) = self.map.tile_in_position_vec2_mut(mouse, Vec2 { x: 288.0, y: 0.0 }) {
            let tile_hover_position = TilePosition::cord_to_tile_position(mouse, Vec2 { x: 288.0, y: 0.0 });
            hover_tile.visible_color = Color { r: 0.0, g: 0.0, b: 1.0, a: 0.5 };

            if is_mouse_button_pressed(MouseButton::Left) {
                if self.selected_tiles.contains(&tile_hover_position) {
                    self.selected_tiles.remove(&tile_hover_position);
                } else {
                    self.selected_tiles.insert(tile_hover_position);
                }
            }
        };

        MapEditorMenu::update(self, update_state)
    }


    pub fn draw(&self) {
        for (row_id, row) in self.map.map.iter().enumerate() {
            for (column_id, tile) in row.iter().enumerate() {
                draw_rectangle((column_id as f32) * TILE_SIZE + 8.0 + 288.0,
                               (row_id as f32) * TILE_SIZE + 8.0,
                               TILE_SIZE - 8.0,
                               TILE_SIZE - 8.0,
                               tile.visible_color,
                )
            }
        }


        for selected_tile_position in self.selected_tiles.iter() {
            //let selected_tile = self.tile_in_position(&selected_tile_position).unwrap();
            // Text draw
            let text = format!("X:{} Y:{}", selected_tile_position.x, selected_tile_position.y);
            let text_rect = measure_text(&text, None, 12, 1.0);
            draw_text(&text,
                      (((selected_tile_position.x as f32) + 0.5) * TILE_SIZE) - (text_rect.width/2.0) + 288.0,
                      (((selected_tile_position.y as f32) + 0.5) * TILE_SIZE) + (text_rect.height/2.0),
                      12.0,
                      BLACK
            )
        }

        self.in_mapa_editor_menu.draw();
    }
}