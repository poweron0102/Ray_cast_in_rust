#[warn(dead_code)]
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
const WHITE: [f32; 4] = [1.0; 4];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

const DEFAULT_SPEED:f64 = 0.1;

use piston_window::*;
use piston_window::ellipse::circle;
use piston_window::types::Color;
//use piston::input::{Button, ButtonState, Key};
//use piston::{ButtonEvent, RenderEvent};
//use piston_window::ellipse::circle;

struct Point{
    X:f64,
    Y:f64
}
struct Player{
    locate:Point,
    size: f64,
    speed:f64,
    color: Color
}

impl Player {
    fn new() ->  Player {
        Player{
            locate: Point {X:50.0, Y:50.0},
            size: 10.0,
            speed: DEFAULT_SPEED,
            color: BLUE,
        }
    }
    fn mover(&mut self){
        println!("movendo...")
    }

    fn down(&mut self) -> Rectangle {

    }
}

fn main() {
    let mut screen: PistonWindow =
        WindowSettings::new("Hello Piston!", [640, 480])
            .exit_on_esc(true).build().unwrap();



    while let Some(event) = screen.next() {
        let mut player: Player = Player::new();
        println!("X:{}  Y:{}", player.locate.X, player.locate.Y);
        player.mover();
        println!("{:?}", event);
        screen.draw_2d(&event, |context, graphics, _device| {
            clear(WHITE, graphics);
            rectangle(RED, // Cor
                      [900.0, 500.0, 100.0, 100.0], // Posiço / tamanho
                      context.transform,
                      graphics);

        });


    }
}

/*
Dezenhar retangulo: -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
        screen.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            rectangle([1.0, 0.0, 0.0, 1.0], // Cor
                      [900.0, 500.0, 100.0, 100.0], // Posiço / tamanho
                      context.transform,
                      graphics);
        });
Dezenhar retangulo: -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

        println!("A sua tela  de {}x{}",screen.size().width, screen.size().height);
*/