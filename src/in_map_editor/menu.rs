use macroquad::prelude::*;
use macroquad::ui::widgets::Label;
use Simples_menu::{Button, CheckBox, Element, Menu, SmartButton, TextLabel};
use Simples_menu::PositionType::Center;
use crate::Game;
use crate::in_map_editor::InMapEditor;
use crate::in_menu::In_menu;

pub struct MapEditorMenu {
    menu: Menu,

    is_wall_label: Element<TextLabel>,
    is_wall_check_box: Element<CheckBox>,

    render_label: Element<TextLabel>,
    render_check_box: Element<CheckBox>,

    paint_red_button: Element<Button>,
    set_tiles_button: Element<Button>,

    save_button: Element<Button>,
    back_to_menu: Element<Button>,
    // todo add sliders for color, text box for action and save.
}
//const T5:Tile = Tile{ is_wall: false, render: false, color: GREEN, visible_color: GREEN, action: Actions::NextMap };
impl MapEditorMenu {
    pub fn new() -> MapEditorMenu {
        println!("bahsbdksjdbkasnj");
        let mut menu = Menu::new("Ray cast".to_string(), vec2(20.0, 20.0));
        menu.size = Some(vec2(250.0, screen_height()));
        menu.color = Color {
            r: 0.0,
            g: 0.7,
            b: 0.6,
            a: 0.4,
        };
        let paint_red_button = menu.add_element(Button::new("Paint RED".to_string(), Center, vec2(125.0, 110.0), None, ));

        let is_wall_label = menu.add_element(TextLabel::new("Is wall?".to_string(), Center, vec2(62.5, 25.0)));
        let is_wall_check_box = menu.add_element(CheckBox::new(Center, vec2(62.5, 50.0), vec2(20.0, 20.0)));

        let render_label = menu.add_element(TextLabel::new("Is render?".to_string(), Center, vec2(187.5, 25.0)));
        let render_check_box = menu.add_element(CheckBox::new(Center, vec2(187.5, 50.0), vec2(20.0, 20.0)));

        let set_tiles_button = menu.add_element(Button::new("Set presets".to_string(), Center, vec2(125.0, 160.0), None, ));

        let save_button= menu.add_element(Button::new("Save".to_string(), Center, vec2(62.5, screen_height() - 90.0), None, ));
        let back_to_menu= menu.add_element(Button::new("Back to menu".to_string(), Center, vec2(187.5, screen_height() - 90.0), None, ));

        MapEditorMenu {
            menu,
            is_wall_label,
            is_wall_check_box,
            render_label,
            render_check_box,
            paint_red_button,
            set_tiles_button,
            save_button,
            back_to_menu,
        }
    }


    pub fn update(in_map_editor: &mut InMapEditor, update_state: &mut Option<Box<dyn Game>>) {
        //let SELF = &mut in_map_editor.in_mapa_editor_menu;

        in_map_editor.in_mapa_editor_menu.menu.update();

        if in_map_editor.in_mapa_editor_menu.paint_red_button.read().has_been_pressed {
            for selected_tile in in_map_editor.selected_tiles.clone() {
                let tile = in_map_editor.map.tile_in_position_mut(&selected_tile).unwrap();
                tile.color = RED;
            };
        }
        if in_map_editor.in_mapa_editor_menu.set_tiles_button.read().has_been_pressed {
            let is_wall = in_map_editor.in_mapa_editor_menu.is_wall_check_box.read().is_checked;
            let is_render = in_map_editor.in_mapa_editor_menu.render_check_box.read().is_checked;
            for selected_tile in in_map_editor.selected_tiles.clone() {
                let tile = in_map_editor.map.tile_in_position_mut(&selected_tile).unwrap();
                tile.is_wall = is_wall;
                tile.render = is_render;
                println!("{:?}", tile)
            };
        }

        if in_map_editor.in_mapa_editor_menu.back_to_menu.read().is_pressed {
            *update_state = Some(Box::new(In_menu::new()))
        }
    }

        pub fn draw(&self) {
            self.menu.draw()
        }
}
