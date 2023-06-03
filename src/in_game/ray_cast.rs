use std::f32::consts::PI;
use std::iter::Map;
use macroquad::color::{BLUE, DARKBROWN, DARKPURPLE, PURPLE};
use macroquad::math::Vec2;
use macroquad::prelude::{draw_rectangle, screen_width, vec2};
use macroquad::prelude::collections::storage::get;
use macroquad::shapes::{draw_circle, draw_line};
use macroquad::window::screen_height;
use crate::{_3PI2, PI2};
use crate::in_game::mine_map::{Tile_size, MapStruct, Tile};
use crate::in_game::player::{normalize_angle, Player};

const FOV:f32 = PI/3.0;
const FOV2:f32 = FOV / 2.0;

const RENDER_DIST:usize = 20;

#[derive(Debug, Clone)]
pub struct RayCast {
    rays: Vec<Ray>
}
#[derive(Debug, Copy, Clone)]
struct Ray {
    position: Vec2,
    size: f32
}

impl RayCast {
    pub fn new () -> RayCast {
        RayCast {
            rays: vec![]
        }
    }

    fn get_rays_size(&mut self, player: &Player, map: &MapStruct) {
        self.rays = vec![];
        let angle_start = normalize_angle(player.angle - FOV2);
        //let angle_end = normalize_angle(player.angle + FOV2);
        let angle_per_cont = normalize_angle(FOV / screen_width());
        let playerX = player.locate.x;
        let playerY = player.locate.y;


        let mut angle_ray = angle_start;
        for cont in 0..(screen_width() as usize) {
            // Calc horizontal -=-=-=-=-=-=-=--=-=-=-=-
            let ray_posi_h:Vec2;
            {
                let aTan:f32 = -1.0 / (f32::tan(angle_ray) + 0.000001);
                let mut ray_x: f32;
                let mut ray_y: f32;
                let mut offset_x: f32 = 0.0;
                let mut offset_y: f32 = 0.0;
                let mut rendist:usize = 0;

                if angle_ray > PI { // loking up
                    ray_y = (((playerY / Tile_size) as i32) as f32) * Tile_size - 0.0001;
                    ray_x = playerX + ((playerY - ray_y) * aTan);
                    offset_y = -Tile_size;
                    offset_x = -offset_y * aTan;
                } else if angle_ray < PI { // loking down
                    ray_y = (((playerY / Tile_size) as i32) as f32) * Tile_size + Tile_size;
                    ray_x = playerX + ((playerY - ray_y) * aTan);
                    offset_y = Tile_size;
                    offset_x = -offset_y * aTan;
                } else {
                    ray_y = playerY;
                    ray_x = playerX;
                    rendist = RENDER_DIST
                }

                while rendist < RENDER_DIST {
                    if map.tile_in_position(Vec2{ x: ray_x, y: ray_y }).render {
                        break
                    }
                    ray_x += offset_x;
                    ray_y += offset_y;
                    rendist += 1;
                }
                ray_posi_h = vec2(ray_x, ray_y)
            }
            // Calc horizontal -=-=-=-=-=-=-=--=-=-=-=-
            // Calc vertical -=-=-=-=-=-=-=--=-=-=-=-
            let ray_posiV:Vec2;
            {
                let aTan:f32 = -f32::tan(angle_ray);
                let mut ray_x: f32;
                let mut ray_y: f32;
                let mut offset_x: f32 = 0.0;
                let mut offset_y: f32 = 0.0;
                let mut rendist:usize = 0;

                if angle_ray < PI2 || angle_ray > _3PI2 { // loking rigth
                    ray_x = (((playerX / Tile_size) as i32) as f32) * Tile_size + Tile_size;
                    ray_y = playerY + ((playerX - ray_x) * aTan);
                    offset_x = Tile_size;
                    offset_y = -offset_x * aTan;
                } else if angle_ray > PI2 && angle_ray < _3PI2 { //loking left
                    ray_x = (((playerX / Tile_size) as i32) as f32) * Tile_size - 0.0001;
                    ray_y = playerY + ((playerX - ray_x) * aTan);
                    offset_x = -Tile_size;
                    offset_y = -offset_x * aTan;
                } else {
                    ray_y = playerY;
                    ray_x = playerX;
                    rendist = RENDER_DIST
                }

                while rendist < RENDER_DIST {
                    if map.tile_in_position(Vec2{ x: ray_x, y: ray_y }).render {
                        break
                    }
                    ray_x += offset_x;
                    ray_y += offset_y;
                    rendist += 1;
                }
                ray_posiV = vec2(ray_x, ray_y);
            }
            // Calc vertical -=-=-=-=-=-=-=--=-=-=-=-

            let dist_h = f32::abs(player.locate.distance(ray_posi_h));
            let dist_v = f32::abs(player.locate.distance(ray_posiV));

            let point:Vec2;
            let size:f32;
            if dist_h < dist_v {
                point = ray_posi_h;
                size = dist_h;
            }else {
                point = ray_posiV;
                size = dist_v;
            }

            self.rays.push(Ray{ position: point, size: f32::cos(player.angle - angle_ray) * size });
            angle_ray += angle_per_cont;
            angle_ray = normalize_angle(angle_ray);
            //draw_circle(ray_posi_h.x,ray_posi_h.y, 9.0,DARKBROWN);
            //draw_circle(ray_posiV.x,ray_posiV.y, 9.0,DARKPURPLE);
        }
    }

    pub fn draw_rays(&mut self, player: &Player, map: &MapStruct) {
        self.get_rays_size(player, map);

        for ray in &self.rays {
            draw_line(player.locate.x,
                      player.locate.y,
                      ray.position.x,
                      ray.position.y,
                      1.0,
                      BLUE)
        }
    }

    pub fn draw(&mut self, player: &Player, map: &MapStruct) {
        self.get_rays_size(player, map);

        let const_screan = 1.0;
        let screen_distance = (screen_width() / 2.0) / f32::tan(FOV2);
        for (ray_num, ray) in self.rays.iter().enumerate() {
            let line_higth = Tile_size * screen_distance / (ray.size + 0.00001);

            draw_rectangle(ray_num as f32 * const_screan,
                           ((screen_width() / 2.0) - line_higth) / 2.0,
                           const_screan,
                           line_higth,
                           map.tile_in_position(ray.position).visible_color);

        }
    }
}