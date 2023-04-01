mod player;

use ggez;
use ggez::{Context, GameError, GameResult};
use ggez::graphics;
//use ggez::nalgebra as na;
use ggez::event;

struct Game {
    //player: Player
}

impl Game {
    pub fn new() -> Self {
        Game {

        }
    }
}

impl event::EventHandler for Game{
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {


        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        graphics::

        Ok(())
    }
}

fn main() -> GameResult {
    let context_builder = ggez::ContextBuilder::new("Ray_Cast", "Tio_Power");
    let (ctx, event_loop) = context_builder.build()?;
    ctx.gfx.set_window_title("Ray cast");

    let mut game = Game::new();
    event::run(ctx, event_loop, game);



    print!("Acabou!");
    Ok(())
}