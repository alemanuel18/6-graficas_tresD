mod caster;
mod controls;
mod draw_cell;
mod framebuffer;
mod load_maze;
mod player;

use crate::caster::cast_ray;
use crate::controls::process_events;
use crate::draw_cell::{render_maze, draw_cell};
use crate::framebuffer::Framebuffer;
use crate::load_maze::{load_maze, Maze};
use crate::player::Player;

use raylib::prelude::*;

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 600;

fn main() {
    let (mut window, thread) = RaylibBuilder::new()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("2D Maze")
        .build();

    let mut framebuffer = Framebuffer::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    let maze = load_maze("src/map/map.txt");

    let player = Player {
        pos: Vector2::new(100.0, 100.0),
        a: 0.0, // looking right
        fov: 60.0 * PI / 180.0, // 60 degrees
    };

    while !window.window_should_close() {
        // 1. clear the framebuffer
        framebuffer.clear();

        // 2. move the player on user input
        process_events(&mut player, &window);

        let mut mode = "2D";

        if window.is_key_down(KeyboardKey::KEY_M) {
            mode = if mode == "2D" { "3D" } else { "2D" };
        }

        // Clear the framebuffer
        framebuffer.clear();
        // 3. draw stuff
        if mode == "2D" {
            render_maze(&mut framebuffer, &maze, block_size, &player);
        } else {
            render_world(&mut framebuffer, &player);
        }
    }
}
