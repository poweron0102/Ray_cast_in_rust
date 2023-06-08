use std::fs::File;
use std::path::Path;
use hlua::Lua;
use crate::in_game::InGame;

/*
pub struct LuaInterpreter {
    lua: Lua<'static>
}
impl LuaInterpreter {
    pub fn new() -> LuaInterpreter {
        LuaInterpreter{
            lua: Lua::new()
        }
    }


    pub fn update(in_game: &mut InGame) {

    }
}
*/

impl InGame {
    pub fn lua_execute(&mut self) {
        {
            let mut lua_player = self.lua_interpreter.empty_array("Player");
            lua_player.set("positionX", self.player.locate.x);
            lua_player.set("positionY", self.player.locate.y);

            lua_player.set("angle", self.player.angle);
            lua_player.set("speed", self.player.speed);
        }
        for (code, position) in &self.lua_scripts {
            {
                let mut lua_tile = self.lua_interpreter.empty_array("Tile");
                lua_tile.set("positionX", position.x as u32);
                lua_tile.set("positionY", position.y as u32);
            }
            self.lua_interpreter.execute_from_reader::<(), _>(File::open(&Path::new(&code)).unwrap())
                .expect(&*format!("The file on {code} get an error when executed by tile in {position:?}."));
        }
        self.player.locate.x = self.lua_interpreter.get("Player.positionY").unwrap();
        self.player.locate.y = self.lua_interpreter.get("Player.positionY").unwrap();

        self.player.angle = self.lua_interpreter.get("Player.angle").unwrap();
        self.player.speed = self.lua_interpreter.get("Player.speed").unwrap();
    }
}