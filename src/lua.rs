use std::fs;
use std::fs::File;
use std::path::Path;
use rlua::prelude::*;
use macroquad::prelude::*;
use rlua::{MetaMethod, UserData, UserDataMethods};
use crate::in_game::InGame;
use crate::in_game::player::Player;

impl UserData for Player {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        // Implement custom methods if needed

        // Implement __index meta-method to allow accessing Player fields in Lua
        methods.add_meta_method(
            MetaMethod::Index,
            |lua, player: &Player, field: String| {
                match field.as_str() {
                    "locateX" => player.locate.x.to_lua(lua),
                    "locateY" => player.locate.y.to_lua(lua),
                    "speed" => player.speed.to_lua(lua),
                    "angle" => player.angle.to_lua(lua),
                    "size" => player.size.to_lua(lua),
                    "is_map_open" => player.is_map_open.to_lua(lua),
                    "show_fps" => player.show_fps.to_lua(lua),
                    "show_menu" => player.show_menu.to_lua(lua),
                    _ => Ok(rlua::Value::Nil), // Return nil for unknown fields
                }
            },
        );
    }
}

pub fn in_game_lua() -> Lua {
    let mut lua = Lua::new();

    lua
}




impl InGame {
    pub fn lua_execute(&mut self) {
        for (code, position) in &self.lua_scripts {
            let script = fs::read_to_string(code).unwrap_or_else(|_|
                panic!("Can't read the file in {code}")
            );

            self.lua_interpreter.context(|lua_ctx|{
                let globals = lua_ctx.globals();

                globals.set("Player", self.player)?;

                // Main script
                lua_ctx
                    .load(&script)
                    .set_name(code)?
                    .exec()?;
                // Main script

                lua_ctx
                    .load("print(Olá por lua!)")
                    .set_name(code)?
                    .exec()?;

                self.player = globals.get("Player")?;

                Ok::<(), LuaError>(())
            }).unwrap_or_else(|_|
                panic!("The file on {code} get an error when executed by tile in {position:?}.")
            );


        }
    }
}

//panic!("The file on {code} get an error when executed by tile in {position:?}.")


