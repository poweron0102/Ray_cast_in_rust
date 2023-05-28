use macroquad::color::Color;
use macroquad::math::vec2;
use macroquad::window::screen_height;
use Simples_menu::*;
use Simples_menu::PositionType::Center;
use crate::Game;
use crate::in_game::In_game;


pub struct In_menu {
    main_menu: Menu,
    //Buttons
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
        let new_game_b = menu.add_element(Button::new("New game".to_string(), Center, vec2(125.0, 10.0), None));
        let configs_b = menu.add_element(Button::new("Config".to_string(), Center,vec2(125.0, 60.0), None));
        let map_editor_b = menu.add_element(Button::new("Map editor".to_string(), Center,vec2(125.0, 110.0), None));

        In_menu{
            main_menu: menu,

            new_game_b,
            configs_b,
            map_editor_b,
        }
    }

    pub fn events(&mut self, update_state: &mut Option<Box<dyn Game>>) {
        self.main_menu.update();
        if let Some(size) = &mut self.main_menu.size { size.y = screen_height() }

        if self.new_game_b.read().has_been_pressed {
            *update_state = Some(Box::new(In_game::new(0)))
        }
    }

    pub fn draw(&mut self) {
        self.main_menu.draw();
    }
}