use std::cell::RefCell;
use std::fs;
use std::rc::Rc;
use rlua::prelude::*;
use macroquad::prelude::*;
use rlua::{Error, Table, Value};
use crate::in_game::InGame;
use crate::in_game::player::Player;
use crate::map::{Tile, TilePosition};

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
    fn from_lua(lua: Value<'lua>, _: rlua::Context<'lua>) -> rlua::Result<Self> {
        match lua {
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


impl ToLua<'_>  for TilePosition {
    fn to_lua(self, lua: LuaContext<'_>) -> LuaResult<Value<'_>> {
        let table = lua.create_table()?;

        table.set("locateX", self.x)?;
        table.set("locateY", self.y)?;

        Ok(Value::Table(table))
    }
}
impl<'lua> FromLua<'lua> for TilePosition {
    fn from_lua(lua_value: Value<'lua>, _: LuaContext<'lua>) -> LuaResult<Self> {
        match lua_value {
            Value::Table(table) => {
                Ok(TilePosition {
                    x: table.get("locateX")?,
                    y: table.get("locateY")?,
                })
            },

            _ => Err(Error::FromLuaConversionError {
                from: "Value",
                to: "TilePosition",
                message: Some("Expected a table".to_string()),
            })
        }
    }
}


impl ToLua<'_>  for Tile {
    fn to_lua(self, lua: LuaContext<'_>) -> LuaResult<Value<'_>> {
        let color_table = lua.create_table()?;
        color_table.set("r", self.color.r)?;
        color_table.set("g", self.color.g)?;
        color_table.set("b", self.color.b)?;
        color_table.set("a", self.color.a)?;


        let table = lua.create_table()?;

        table.set("is_wall", self.is_wall)?;
        table.set("render", self.render)?;
        //table.set("render", self.render)?;
        table.set("color", color_table)?;
        table.set("step_action", self.step_action)?;
        table.set("look_action", self.look_action)?;
        table.set("render_action", self.render_action)?;

        Ok(Value::Table(table))
    }
}
impl<'lua> FromLua<'lua> for Tile {
    fn from_lua(lua_value: Value<'lua>, _: LuaContext<'lua>) -> LuaResult<Self> {
        match lua_value {
            Value::Table(table) => {
                let color_table: Table = table.get("color")?;
                let color = Color {
                    r: color_table.get("r")?,
                    g: color_table.get("g")?,
                    b: color_table.get("b")?,
                    a: color_table.get("a")?,
                };

                Ok(Tile {
                    is_wall: table.get("is_wall")?,
                    render: table.get("render")?,
                    color,
                    visible_color: color,
                    step_action: table.get("step_action")?,
                    look_action: table.get("look_action")?,
                    render_action: table.get("render_action")?,
                })
            },

            _ => Err(Error::FromLuaConversionError {
                from: "Value",
                to: "TilePosition",
                message: Some("Expected a table".to_string()),
            })
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
            let map = Rc::new(RefCell::new(&mut self.map.map));

            self.lua_interpreter.context(|lua_ctx|{
                let globals = lua_ctx.globals();

                globals.set("Player", self.player)?;
                globals.set("TilePosition", *position)?;

                lua_ctx.scope(|scope| {
                    //let get_tile_map = map.clone();
                    let get_tile = scope.create_function(|_, (x, y): (f32, f32)| {
                        let tile = map.borrow().tile_in_position_vec2(
                            Vec2::new(x, y), Vec2::new(0.0, 0.0)
                        ).expect(
                            &format!("There is no Tile in \"{x}, {y}\" \nError occurred while executed script in {position:?}")
                        ).clone();

                        Ok(tile)
                    }
                    )?;
                    globals.set("get_tile",get_tile)?;

                    let set_tile = scope.create_function_mut(|_, (x, y, tile): (f32, f32, Tile)| {
                        *map.borrow_mut().tile_in_position_vec2_mut(
                            Vec2::new(x, y), Vec2::new(0.0, 0.0)
                        ).expect(
                            &format!("There is no Tile in \"{x}, {y}\" \nError occurred while executed script in {position:?}")
                        ) = tile;


                        Ok(())
                    })?;
                    globals.set("set_tile",set_tile)?;



                    // Main script
                    lua_ctx
                        .load(&script)
                        .set_name(code)?
                        .exec()?;
                    // Main script
                    Ok(())
                })?;


                self.player = globals.get("Player")?;

                Ok::<(), LuaError>(())
            }).unwrap_or_else(|error|
                panic!("The file on {code} get an error when executed by tile in {position:?}\n error: {error}")
            );


        }
    }
}

//panic!("The file on {code} get an error when executed by tile in {position:?}.")


