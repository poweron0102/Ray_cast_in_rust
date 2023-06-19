use macroquad::prelude::*;
use crate::Game;
use crate::in_game::mine_map::MineMap;
use crate::in_game::pause_menu::PauseMenu;
use crate::in_game::player::Player;
use crate::in_game::ray_cast::RayCast;
use crate::map::TilePosition;
use rlua::prelude::*;


mod mine_map;
pub(crate) mod player;
mod ray_cast;
mod pause_menu;

impl Clone for InGame {
    fn clone(&self) -> Self {
        InGame {
            map: self.map.clone(),
            player: self.player,
            ray_cast: self.ray_cast.clone(),
            pause_menu: None,
            lua_interpreter: Lua::new(),
            lua_scripts: vec![],
        }
    }
}
//#[derive()]
pub struct InGame {
    pub map: MineMap,
    pub player: Player,
    ray_cast: RayCast,
    pause_menu: Option<PauseMenu>,
    pub lua_interpreter: Lua,
    pub lua_scripts: Vec<(String, TilePosition)>
}
impl InGame {
    pub fn new(map_name: &str) -> InGame {
        let mut lua_interpreter = Lua::new();

        InGame {
            map: MineMap::new(map_name),
            player: Player::new(),
            ray_cast: RayCast::new(),
            pause_menu: None,
            lua_interpreter,
            lua_scripts: vec![],
        }
    }

    pub fn events(&mut self, update_state: &mut Option<Box<dyn Game>>) {
        self.player.mouse();
        self.player.keyboard(&self.map);



        if self.player.show_menu {
            if self.pause_menu.is_some() {
                self.pause_menu.as_mut().unwrap().update(update_state, &mut self.player)
            }
            else { self.pause_menu = Some(PauseMenu::new()) }
        }


        self.lua_scripts.clear();
        if let Some(action) = &self.map.tile_in_position(self.player.locate).step_action {
            let full_path = "./lua/".to_string() + action;

            self.lua_scripts.push((full_path, TilePosition::cord_to_tile_position(self.player.locate, vec2(0.0, 0.0))))
        }

        self.lua_execute()
    }


    pub fn draw(&mut self) {
        self.ray_cast.draw(&self.player, &self.map);

        if self.player.is_map_open {
            self.map.mine_map_draw();
            self.player.mine_player_draw(&self.map);
        }

        if self.player.show_menu {
            self.pause_menu.as_ref().unwrap().draw()
        }

        if self.player.show_fps {
            draw_text(&format!("FPS: {}", get_fps()), 20.0, 20.0, 23.0, YELLOW);
        }
    }
}
