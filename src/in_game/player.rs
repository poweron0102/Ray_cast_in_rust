use crate::in_game::mine_map::*;
use crate::*;


const MOUSE_SEN:f32 = 4.0;

fn sig(num:f32) -> f32 {
    if num < 0.0 {
        return -1.0
    }else {
        return 1.0
    }
}

pub fn normalize_angle(angle: f32) -> f32 {
    let mut result = angle;
    while result < 0.0 {
        result += 2.0 * PI;
    }
    while result >= 2.0 * PI {
        result -= 2.0 * PI;
    }
    result
}

#[derive(Debug, Copy, Clone)]
pub struct Player {
    pub locate: Vec2,
    pub speed: f32,
    pub angle: f32,
    pub size: f32,
    pub is_map_open: bool,
    pub show_fps: bool,
    pub show_menu: bool
}
impl Player {
    pub fn new() -> Player {
        Player {
            locate: vec2(50.0, 50.0),
            speed: 300.0,
            angle: 0.0,
            size:  15.0,
            is_map_open: false,
            show_fps: false,
            show_menu: false,
        }
    }

    fn forward_pont(&self, size:f32) -> Vec2 {
        Vec2{
            x: self.locate.x + (size * f32::cos(self.angle)),
            y: self.locate.y + (size * f32::sin(self.angle)),
        }
    }


    pub fn keyboard(&mut self, map:&MapStruct) {
        //-=-=-=-=-=-=-=-=- Keyboard Movimento-=-=-=-=-=-=-=-=-=-=-=-
        let relative_speed = self.speed * get_frame_time();
        let mut delta = self.locate;

        if is_key_down(KeyCode::W) {
            delta = self.locate;
            delta.x += relative_speed * f32::cos(self.angle);
            if !map.tile_in_position(delta).is_wall { self.locate = delta };

            delta = self.locate;
            delta.y += relative_speed * f32::sin(self.angle);
            if !map.tile_in_position(delta).is_wall { self.locate = delta };
        }
        if is_key_down(KeyCode::A) {
            delta = self.locate;
            delta.x += -relative_speed * f32::sin(self.angle + PI);
            if !map.tile_in_position(delta).is_wall { self.locate = delta };

            delta = self.locate;
            delta.y += relative_speed * f32::cos(self.angle + PI);
            if !map.tile_in_position(delta).is_wall { self.locate = delta };
        }
        if is_key_down(KeyCode::S) {
            delta = self.locate;
            delta.x += relative_speed * f32::sin(self.angle + (PI2 * 3.0));
            if !map.tile_in_position(delta).is_wall { self.locate = delta };

            delta = self.locate;
            delta.y += -relative_speed * f32::cos(self.angle + (PI2 * 3.0));
            if !map.tile_in_position(delta).is_wall { self.locate = delta };
        }
        if is_key_down(KeyCode::D) {
            delta = self.locate;
            delta.x += relative_speed * f32::cos(self.angle + PI2);
            if !map.tile_in_position(delta).is_wall { self.locate = delta };

            delta = self.locate;
            delta.y += relative_speed * f32::sin(self.angle + PI2);
            if !map.tile_in_position(delta).is_wall { self.locate = delta };
        }
        //println!("A posição do jogador é: {:?}", delta);
        //-=-=-=-=-=-=-=-=- Keyboard Movimento-=-=-=-=-=-=-=-=-=-=-=-
        if is_key_released(KeyCode::M) {
            self.is_map_open = !self.is_map_open;
        }
        if is_key_released(KeyCode::P) {
            self.show_fps = !self.show_fps;
        }
        if is_key_released(KeyCode::E) {
            self.show_menu = !self.show_menu;
        }
    }

    pub fn mouse(&mut self) {
        self.angle -= mouse_delta_position().x * MOUSE_SEN;
        self.angle = normalize_angle(self.angle);
        //keep_mouse_centered();
    }

    pub fn draw(&self) {
        draw_circle(self.locate.x,
                    self.locate.y,
                    16.0,
                    GREEN
        );

        let forward = self.forward_pont(16.0);
        draw_line(self.locate.x,
                  self.locate.y,
                  forward.x,
                  forward.y,
                  4.0,
                  PINK
        );
    }

    pub fn mine_player_draw(&self, map: &MapStruct) {
        let mine_locale = map.to_mine_map(self.locate);
        draw_circle(mine_locale.x,
                    mine_locale.y,
                    4.0,
                    GREEN
        );

        let forward = map.to_mine_map(self.forward_pont(16.0));
        draw_line(mine_locale.x,
                  mine_locale.y,
                  forward.x,
                  forward.y,
                  2.0,
                  PINK
        );
    }
}