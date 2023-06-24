use std::fs;
use rlua::prelude::*;
use macroquad::prelude::*;
use rlua::{Error, Value};
use crate::in_game::InGame;
use crate::in_game::player::Player;

impl ToLua<'_> for Player {
    fn to_lua(self, lua: rlua::Context<'_>) -> rlua::Result<Value> {
        let table = lua.create_table()?;
        table.set("locateX", self.locate.x)?;
        table.set("locateY", self.locate.y)?;
        table.set("speed", self.speed)?;
        table.set("angle", self.angle)?;
        table.set("size", self.size)?;
        table.set("is_map_open", self.is_map_open)?;
        table.set("show_fps", self.show_fps)?;
        table.set("show_menu", self.show_menu)?;
        Ok(Value::Table(table))
    }
}

impl<'lua> FromLua<'lua> for Player {
    fn from_lua(lua_value: Value<'lua>, _: rlua::Context<'lua>) -> rlua::Result<Self> {
        match lua_value {
            Value::Table(table) => {
                let locate = Vec2{ x: table.get("locateX")?, y: table.get("locateY")? };
                Ok(Player {
                    locate,
                    speed: table.get("speed")?,
                    angle: table.get("angle")?,
                    size: table.get("size")?,
                    is_map_open: table.get("is_map_open")?,
                    show_fps: table.get("show_fps")?,
                    show_menu: table.get("show_menu")?,
                })
            }
            _ => Err(Error::FromLuaConversionError {
                from: "Value",
                to: "Player",
                message: Some("Expected a table".to_string()),
            }),
        }
    }
}



// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

impl InGame {
    pub fn lua_execute(&mut self) {
        for (code, position) in &self.lua_scripts {
            let script = fs::read_to_string(code).unwrap_or_else(|error|
                panic!("Can't read the file in {code} error: {error}")
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

                self.player = globals.get("Player")?;

                Ok::<(), LuaError>(())
            }).unwrap_or_else(|error|
                panic!("The file on {code} get an error when executed by tile in {position:?}\n error: {error}")
            );


        }
    }
}

//panic!("The file on {code} get an error when executed by tile in {position:?}.")


