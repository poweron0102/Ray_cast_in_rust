fn ray_size(&mut self) {
    let player_x = self.game.player.x;
    let player_y = self.game.player.y;
    let player_ang = self.game.player.ang;
    let mut offset_x = 0.0;
    let mut offset_y = 0.0;
    self.rays = vec![];
    let rays_per_angle = Rays_per_angle as f32;
    let angle_ray = angle_to_fist(player_ang - (FOV / 2.0).to_radians());

    for i in 0..(FOV * rays_per_angle) as i32 {
        // Calculo horizontal -=-=-=-=-=-=-=--=-=-=-=-
        let mut rendist = 0.0;
        let a_tan = -1.0 / (angle_ray.tan() + 0.00001);
        let (ray_x, ray_y, offset_x_temp, offset_y_temp) = if angle_ray > std::f32::consts::PI {
            // looking up
            let ray_y = (player_y / Tile_size as f32).floor() * Tile_size as f32 - 0.0001;
            let ray_x = player_x + ((player_y - ray_y) * a_tan);
            let offset_y = -Tile_size as f32;
            let offset_x = -offset_y * a_tan;
            (ray_x, ray_y, offset_x, offset_y)
        } else if angle_ray < std::f32::consts::PI {
            // looking down
            let ray_y = (player_y / Tile_size as f32).floor() * Tile_size as f32 + Tile_size as f32;
            let ray_x = player_x + ((player_y - ray_y) * a_tan);
            let offset_y = Tile_size as f32;
            let offset_x = -offset_y * a_tan;
            (ray_x, ray_y, offset_x, offset_y)
        } else {
            // if angle_ray == 0.0 or angle_ray == std::f32::consts::PI:
            let ray_y = player_y;
            let ray_x = player_x;
            let rendist = Render_dist as f32;
            (ray_x, ray_y, 0.0, 0.0, rendist)
        };

        while rendist < Render_dist as f32 {
            if (Tile_size as f32) < ray_x && ray_x < (RES[0] + Tile_size) as f32 &&
                -(Tile_size as f32) < ray_y && ray_y < (RES[1] + Tile_size) as f32
            {
                if self.game.map.is_render(ray_x, ray_y) {
                    break;
                }
            }
            ray_x += offset_x_temp;
            ray_y += offset_y_temp;
            rendist += 1.0;
        }

        let ray_posi
