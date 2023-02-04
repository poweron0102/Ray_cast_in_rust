const DEFAULT_SPEED:f64 = 0.1;

struct Player{
    //locate:Point,
    size: f64,
    speed:f64,
    //color: Color
}

impl Player {
    fn new() -> Player {
        Player {
            //locate: Point { X: 50.0, Y: 50.0 },
            size: 10.0,
            speed: DEFAULT_SPEED,
            //color: BLUE,
        }
    }
    fn mover(&mut self) {
        println!("movendo...")
    }
}

struct Point{
    X:f64,
    Y:f64
}