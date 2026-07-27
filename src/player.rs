// player.rs

use raylib::prelude::*;
use std::f32::consts::PI;

pub struct Player {
    pub pos: Vector2,
    pub a: f32, // angle of view
    pub fov: f32,
}

// draw what the player sees
let num_rays = 5;
for i in 0..num_rays {
    let current_ray = i as f32 / num_rays as f32; // current ray divided by total rays
    let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
    cast_ray(framebuffer, &maze, &player, a, block_size);
}