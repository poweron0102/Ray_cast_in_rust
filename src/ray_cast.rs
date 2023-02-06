use std::f32::consts::PI;
use std::iter::Map;
use macroquad::color::{BLUE, DARKBROWN, DARKPURPLE, PURPLE};
use macroquad::math::Vec2;
use macroquad::prelude::{draw_rectangle, screen_width, vec2};
use macroquad::prelude::collections::storage::get;
use macroquad::shapes::{draw_circle, draw_line};
use macroquad::window::screen_height;
use crate::{_3PI2, PI2};
use crate::mine_map::{Tile_size, MapStruct, Tile};
use crate::player::{normalize_angle, Player};

const FOV:f32 = PI/3.0;
const FOV2:f32 = FOV / 2.0;

const RENDER_DIST:usize = 20;

pub struct RayCast {
    rays: Vec<Ray>
}
struct Ray {
    position: Vec2,
    size: f32
}

impl RayCast {
    pub fn new <'a> (player: &'a Player, map: &'a MapStruct) -> RayCast {
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
            let ray_posiH:Vec2;
            {
                let aTan:f32 = -1.0 / (f32::tan(angle_ray) + 0.000001);
                let mut rayX: f32;
                let mut rayY: f32;
                let mut offsetX: f32 = 0.0;
                let mut offsetY: f32 = 0.0;
                let mut rendist:usize = 0;

                if angle_ray > PI { // loking up
                    rayY = (((playerY / Tile_size) as i32) as f32) * Tile_size - 0.0001;
                    rayX = playerX + ((playerY - rayY) * aTan);
                    offsetY = -Tile_size;
                    offsetX = -offsetY * aTan;
                } else if angle_ray < PI { // loking down
                    rayY = (((playerY / Tile_size) as i32) as f32) * Tile_size + Tile_size;
                    rayX = playerX + ((playerY - rayY) * aTan);
                    offsetY = Tile_size;
                    offsetX = -offsetY * aTan;
                } else {
                    rayY = playerY;
                    rayX = playerX;
                    rendist = RENDER_DIST
                }

                while rendist < RENDER_DIST {
                    if map.tile_in_position(Vec2{ x: rayX, y: rayY }).render {
                        break
                    }
                    rayX += offsetX;
                    rayY += offsetY;
                    rendist += 1;
                }
                ray_posiH = vec2(rayX, rayY)
            }
            // Calc horizontal -=-=-=-=-=-=-=--=-=-=-=-
            // Calc vertical -=-=-=-=-=-=-=--=-=-=-=-
            let ray_posiV:Vec2;
            {
                let aTan:f32 = -f32::tan(angle_ray);
                let mut rayX: f32;
                let mut rayY: f32;
                let mut offsetX: f32 = 0.0;
                let mut offsetY: f32 = 0.0;
                let mut rendist:usize = 0;

                if angle_ray < PI2 || angle_ray > _3PI2 { // loking rigth
                    rayX = (((playerX / Tile_size) as i32) as f32) * Tile_size + Tile_size;
                    rayY = playerY + ((playerX - rayX) * aTan);
                    offsetX = Tile_size;
                    offsetY = -offsetX * aTan;
                } else if angle_ray > PI2 && angle_ray < _3PI2 { //loking left
                    rayX = (((playerX / Tile_size) as i32) as f32) * Tile_size - 0.0001;
                    rayY = playerY + ((playerX - rayX) * aTan);
                    offsetX = -Tile_size;
                    offsetY = -offsetX * aTan;
                } else {
                    rayY = playerY;
                    rayX = playerX;
                    rendist = RENDER_DIST
                }

                while rendist < RENDER_DIST {
                    if map.tile_in_position(Vec2{ x: rayX, y: rayY }).render {
                        break
                    }
                    rayX += offsetX;
                    rayY += offsetY;
                    rendist += 1;
                }
                ray_posiV = vec2(rayX, rayY);
            }
            // Calc vertical -=-=-=-=-=-=-=--=-=-=-=-

            let distH = f32::abs(player.locate.distance(ray_posiH));
            let distV = f32::abs(player.locate.distance(ray_posiV));

            let point:Vec2;
            let size:f32;
            if distH < distV {
                point = ray_posiH;
                size = distH;
            }else {
                point = ray_posiV;
                size = distV;
            }

            self.rays.push(Ray{ position: point, size: f32::cos(player.angle - angle_ray) * size });
            angle_ray += angle_per_cont;
            angle_ray = normalize_angle(angle_ray);
            //draw_circle(ray_posiH.x,ray_posiH.y, 9.0,DARKBROWN);
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
            let line_higth = (Tile_size * screen_distance / (ray.size + 0.00001));

            draw_rectangle(ray_num as f32 * const_screan,
                           ((screen_width() / 2.0) - line_higth) / 2.0,
                           const_screan,
                           line_higth,
                           map.tile_in_position(ray.position).color);

        }
    }
}