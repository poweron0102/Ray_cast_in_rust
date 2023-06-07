use std::collections::HashSet;
use std::{fs, io};

use macroquad::color::Color;
use macroquad::math::vec2;
use macroquad::window::screen_height;
use Simples_menu::*;
use Simples_menu::PositionType::Center;

use crate::Game;
use crate::in_game::In_game;
use crate::in_map_editor::InMapEditor;
use crate::map::WordMap;

fn get_file_names_in_folder(folder_path: &str) -> io::Result<Vec<String>> {
    let entries = fs::read_dir(folder_path)?;
    let file_names: Vec<String> = entries
        .filter_map(|entry| {
            if let Ok(entry) = entry {
                if let Ok(file_name) = entry.file_name().into_string() {
                    return Some(file_name);
                }
            }
            None
        })
        .collect();
    Ok(file_names)
}

pub struct In_menu {
    main_menu: Menu,
    //Buttons
    continue_b:   Element<Button>,
    map_options:  Vec<Element<Button>>,
    new_game_b:   Element<Button>,
    configs_b:    Element<Button>,
    map_editor_b: Element<Button>,
}
impl In_menu {
    pub fn new() -> In_menu {
        let mut menu = Menu::new("Ray cast".to_string(), vec2(20.0, 20.0));
        menu.size = Some(vec2(250.0, screen_height()));
        menu.color = Color{
            r: 0.0,
            g: 0.7,
            b: 0.6,
            a: 0.4,
        };
        let continue_b =menu.add_element(
            Button::new("Continue".to_string(), Center, vec2(125.0, 10.0), None)
        );
        let new_game_b = menu.add_element(
            Button::new("New game".to_string(), Center, vec2(125.0, 60.0), None)
        );
        let configs_b = menu.add_element(
            Button::new("Config".to_string(), Center,vec2(125.0, 110.0), None)
        );
        let map_editor_b = menu.add_element(
            Button::new("Map editor".to_string(), Center,vec2(125.0, 160.0), None)
        );

        In_menu{
            main_menu: menu,

            continue_b,
            map_options: vec![],
            new_game_b,
            configs_b,
            map_editor_b,
        }
    }

    pub fn events(&mut self, update_state: &mut Option<Box<dyn Game>>) {
        self.main_menu.update();
        if let Some(size) = &mut self.main_menu.size { size.y = screen_height() }

        if self.new_game_b.read().has_been_pressed {
            let maps_options = get_file_names_in_folder("./save/maps").unwrap();
            let mut higth = 10.0;
            for map_option in maps_options {
                let button = self.main_menu.add_element(
                    Button::new(map_option, Center, vec2(325.0, higth), None)
                );
                self.map_options.push(button);
                higth += 50.0;
            }
        }
        if self.map_editor_b.read().has_been_pressed {
            *update_state = Some(Box::new(InMapEditor::new()))
        }


        if self.continue_b.read().has_been_pressed {
            println!("Todo")
        }

        for map_option in self.map_options.iter() {
            if map_option.read().has_been_pressed {
                *update_state = Some(Box::new(In_game::new(&*map_option.read().title.name)))
            }
        }
    }

    pub fn draw(&mut self) {
        self.main_menu.draw();
    }
}