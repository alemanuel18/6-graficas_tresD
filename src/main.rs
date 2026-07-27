//! Un raycaster de columnas inspirado en la técnica de Wolfenstein 3D.

mod controls;
mod framebuffer;
mod map;
mod player;
mod raycaster;
mod renderer;

use std::process;

use raylib::prelude::*;

use crate::{
    controls::update_player,
    framebuffer::Framebuffer,
    map::Map,
    player::Player,
    renderer::{render_minimap, render_world},
};

const SCREEN_WIDTH: i32 = 960;
const SCREEN_HEIGHT: i32 = 640;
const MAP_PATH: &str = "src/map/map.txt";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ViewMode {
    ThreeDimensional,
    Map,
}

fn main() {
    let map = Map::load(MAP_PATH).unwrap_or_else(|error| {
        eprintln!("No se pudo cargar {MAP_PATH}: {error}");
        process::exit(1);
    });
    let mut player = Player::new(map.player_spawn());

    let (mut window, raylib_thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Wolfenstein-style raycaster — M: mapa, ESC: salir")
        .build();
    window.set_target_fps(60);

    let mut framebuffer = Framebuffer::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    // La textura se crea una vez y después sólo se actualizan sus píxeles.
    let initial_image = Image::gen_image_color(SCREEN_WIDTH, SCREEN_HEIGHT, Color::BLACK);
    let mut screen_texture = window
        .load_texture_from_image(&raylib_thread, &initial_image)
        .expect("Raylib no pudo crear la textura de pantalla");
    let mut mode = ViewMode::ThreeDimensional;

    while !window.window_should_close() {
        let delta_seconds = window.get_frame_time().min(0.05);
        update_player(&mut player, &map, &window, delta_seconds);

        if window.is_key_pressed(KeyboardKey::KEY_M) {
            mode = match mode {
                ViewMode::ThreeDimensional => ViewMode::Map,
                ViewMode::Map => ViewMode::ThreeDimensional,
            };
        }

        framebuffer.clear();
        match mode {
            ViewMode::ThreeDimensional => {
                render_world(&mut framebuffer, &map, &player);
                render_minimap(&mut framebuffer, &map, &player, true);
            }
            ViewMode::Map => render_minimap(&mut framebuffer, &map, &player, false),
        }
        framebuffer.present(&mut window, &raylib_thread, &mut screen_texture);
    }
}
