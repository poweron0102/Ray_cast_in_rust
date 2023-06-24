use std::io;
use macroquad::prelude::*;
use macroquad::ui::widgets::Label;
use Simples_menu::{Button, CheckBox, Element, Menu, SmartButton, TextLabel};
use Simples_menu::PositionType::Center;
use crate::Game;
use crate::in_map_editor::InMapEditor;
use crate::in_menu::In_menu;
use crate::map::WordMap;

pub struct MapEditorMenu {
    menu: Menu,

    is_wall_label: Element<TextLabel>,
    is_wall_check_box: Element<CheckBox>,

    render_label: Element<TextLabel>,
    render_check_box: Element<CheckBox>,

    paint_red_button: Element<Button>,
    set_tiles_button: Element<Button>,
    set_step_action_button: Element<Button>,

    save_button: Element<Button>,
    load_button: Element<Button>,
    back_to_menu: Element<Button>,
    // todo add sliders for color, text box for action and save.
}
//const T5:Tile = Tile{ is_wall: false, render: false, color: GREEN, visible_color: GREEN, action: Actions::NextMap };
impl MapEditorMenu {
    pub fn new() -> MapEditorMenu {
        let mut menu = Menu::new("Ray cast".to_string(), vec2(20.0, 20.0));
        menu.size = Some(vec2(250.0, screen_height()));
        menu.color = Color {
            r: 0.0,
            g: 0.7,
            b: 0.6,
            a: 0.4,
        };
        let paint_red_button = menu.add_element(
            Button::new("Paint RED".to_string(), Center, vec2(125.0, 110.0), None )
        );

        let is_wall_label = menu.add_element(
            TextLabel::new("Is wall?".to_string(), Center, vec2(62.5, 25.0))
        );
        let is_wall_check_box = menu.add_element(
            CheckBox::new(Center, vec2(62.5, 50.0), vec2(20.0, 20.0))
        );

        let render_label = menu.add_element(
            TextLabel::new("Is render?".to_string(), Center, vec2(187.5, 25.0))
        );
        let render_check_box = menu.add_element(
            CheckBox::new(Center, vec2(187.5, 50.0), vec2(20.0, 20.0))
        );

        let set_tiles_button = menu.add_element(
            Button::new("Set presets".to_string(), Center, vec2(125.0, 160.0), None)
        );

        let set_step_action_button= menu.add_element(
            Button::new("Set step action".to_string(), Center, vec2(125.0, 210.0), None)
        );

        let save_button= menu.add_element(
            Button::new("Save".to_string(), Center, vec2(32.5, screen_height() - 100.0), None)
        );
        let load_button= menu.add_element(
            Button::new("Load".to_string(), Center, vec2(32.5, screen_height() - 80.0), None)
        );
        let back_to_menu= menu.add_element(
            Button::new("Back to menu".to_string(), Center, vec2(187.5, screen_height() - 90.0), None)
        );

        MapEditorMenu {
            menu,
            is_wall_label,
            is_wall_check_box,
            render_label,
            render_check_box,
            paint_red_button,
            set_tiles_button,
            set_step_action_button,
            save_button,
            load_button,
            back_to_menu,
        }
    }


    pub fn update(in_map_editor: &mut InMapEditor, update_state: &mut Option<Box<dyn Game>>) {

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
            };
        }

        if in_map_editor.in_mapa_editor_menu.save_button.read().has_been_pressed {
            println!("Qual o nome do mapa para salvar? ");
            let mut nome= String::new();
            io::stdin().read_line(&mut nome).expect("Erro ao ler o nome do mapa");
            nome = nome[..&nome.len() -1].to_string();
            let path = nome + ".json";
            in_map_editor.map.save_map(&*path);
        }

        if in_map_editor.in_mapa_editor_menu.load_button.read().has_been_pressed {
            println!("Qual o nome do mapa para carregar? ");
            let mut nome= String::new();
            io::stdin().read_line(&mut nome).expect("Erro ao ler o nome do mapa");
            nome = nome[..&nome.len() -1].to_string();
            let path = nome + ".json";

            in_map_editor.map = WordMap::new_from_map_save(&*path)
        }

        if in_map_editor.in_mapa_editor_menu.set_step_action_button.read().has_been_pressed {
            println!("Entre com o nome do script lua? ");
            let mut nome= String::new();
            io::stdin().read_line(&mut nome).expect("Erro ao ler o nome do mapa");
            nome = nome[..&nome.len() -1].to_string();
            let path = nome + ".lua";
            for selected_tile in in_map_editor.selected_tiles.clone() {
                let tile = in_map_editor.map.tile_in_position_mut(&selected_tile).unwrap();

                tile.step_action = Some(path.clone())
            }
        }

        if in_map_editor.in_mapa_editor_menu.back_to_menu.read().has_been_pressed {
            *update_state = Some(Box::new(In_menu::new()))
        }
    }

        pub fn draw(&self) {
            self.menu.draw()
        }
}
