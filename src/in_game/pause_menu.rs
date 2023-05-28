use macroquad::prelude::*;
use Simples_menu::*;
use Simples_menu::PositionType::*;
use crate::Game;
use crate::in_game::player::Player;
use crate::in_menu::In_menu;

pub struct PauseMenu {
    pub pause_menu: Menu,

    pub continue_button: Element<Button>,
    pub config_button: Element<Button>,
    pub main_menu_button: Element<Button>,
}
impl PauseMenu {
    pub fn new() -> PauseMenu {
        let mut pause_menu = Menu::new("Pause menu".to_string(), vec2(20.0, 20.0));

        pause_menu.size = Some(vec2(250.0, screen_height()));
        pause_menu.color = Color{ r: 0.0, g: 0.7, b: 0.6, a: 0.4, };

        //let menu_rect = pause_menu.bounding_rect().unwrap();
        //pause_menu.position = Vec2 {
        //    x: (screen_width() - menu_rect.x) / 2.0,
        //    y: (screen_height() - menu_rect.y) / 2.0
        //};

        let continue_button = pause_menu.add_element(Button::new("Continue".to_string(), Center, Vec2 { x: 125.0, y: 10.0 }, None));
        let config_button = pause_menu.add_element(Button::new("Config".to_string(), Center, Vec2 { x: 125.0, y: 60.0 }, None));
        let main_menu_button = pause_menu.add_element(Button::new("Return to main menu".to_string(), Center, Vec2 { x: 125.0, y: 110.0 }, None));


        PauseMenu {
            pause_menu,
            continue_button,
            config_button,
            main_menu_button,
        }
    }

    pub fn update(&mut self, update_state: &mut Option<Box<dyn Game>>, player: &mut Player) {
        self.pause_menu.update();

        if self.continue_button.read().has_been_pressed {
            player.show_menu = false
        }

        if self.main_menu_button.read().has_been_pressed {
            *update_state = Some(Box::new(In_menu::new()))
        }

    }

    pub fn draw(&self) {
        self.pause_menu.draw()
    }
}