//! Aplicación completa: menú, dos niveles, HUD y estados terminales.

mod controls;
mod framebuffer;
mod game;
mod map;
mod player;
mod raycaster;
mod renderer;

use crate::{
    framebuffer::Framebuffer,
    game::{Event, Level},
    renderer::{Bitmap, RenderAssets, render_minimap, render_world},
};
use raylib::prelude::*;
use std::process;

const SCREEN_WIDTH: i32 = 960;
const SCREEN_HEIGHT: i32 = 640;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Screen {
    Menu,
    Playing,
    Victory,
    Defeat,
}

fn main() {
    let (mut window, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Wolfenstein Rust | flechas/WASD, mouse, espacio")
        .build();
    window.set_target_fps(60);
    window.set_exit_key(Some(KeyboardKey::KEY_ESCAPE));
    window.enable_cursor();
    let mut framebuffer = Framebuffer::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    let image = Image::gen_image_color(SCREEN_WIDTH, SCREEN_HEIGHT, Color::BLACK);
    let mut screen_texture = window
        .load_texture_from_image(&thread, &image)
        .expect("No se pudo crear la textura de pantalla");
    let wall_texture =
        Bitmap::load("Assets/escenario/MS-DOS - Wolfenstein 3D - Miscellaneous - Walls.png");
    let enemy_sprite =
        Bitmap::load_sprite("Assets/Enemis/MS-DOS - Wolfenstein 3D - Enemies - SS.png");
    let boss_sprite = Bitmap::load_sprite(
        "Assets/Enemis/MS-DOS - Wolfenstein 3D - Bosses - General Fettgesicht.png",
    );
    let audio = raylib::audio::RaylibAudio::init_audio_device().ok();
    let pistol_sound = audio
        .as_ref()
        .and_then(|a| a.new_sound("Assets/Weapons/ATKPISTOLSND.WAV").ok());
    let shotgun_sound = audio
        .as_ref()
        .and_then(|a| a.new_sound("Assets/Weapons/ATKGATLINGSND.WAV").ok());
    let death_sound = audio
        .as_ref()
        .and_then(|a| a.new_sound("Assets/Bosses/DIESND.WAV").ok());
    let mut screen = Screen::Menu;
    let mut selected_level = 1usize;
    let mut level: Option<Level> = None;
    while !window.window_should_close() {
        let delta = window.get_frame_time().min(0.05);
        match screen {
            Screen::Menu => {
                if window.is_key_pressed(KeyboardKey::KEY_ONE) {
                    selected_level = 1;
                }
                if window.is_key_pressed(KeyboardKey::KEY_TWO) {
                    selected_level = 2;
                }
                if window.is_key_pressed(KeyboardKey::KEY_ENTER)
                    || window.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
                {
                    level = Some(Level::load(selected_level).unwrap_or_else(|error| {
                        eprintln!("No se pudo cargar el nivel: {error}");
                        process::exit(1);
                    }));
                    screen = Screen::Playing;
                    window.disable_cursor();
                    window.set_mouse_position(Vector2::new(
                        (SCREEN_WIDTH / 2) as f32,
                        (SCREEN_HEIGHT / 2) as f32,
                    ));
                }
            }
            Screen::Playing => {
                let event = level
                    .as_mut()
                    .map(|game| game.update(&window, delta))
                    .unwrap_or(Event::Defeat);
                if event == Event::Victory {
                    if let Some(sound) = &death_sound {
                        sound.play();
                    }
                    screen = Screen::Victory;
                    window.enable_cursor();
                }
                if event == Event::Defeat {
                    screen = Screen::Defeat;
                    window.enable_cursor();
                }
            }
            Screen::Victory | Screen::Defeat => {
                if window.is_key_pressed(KeyboardKey::KEY_ENTER) {
                    level = Some(Level::load(selected_level).expect("nivel inválido"));
                    screen = Screen::Playing;
                    window.disable_cursor();
                    window.set_mouse_position(Vector2::new(
                        (SCREEN_WIDTH / 2) as f32,
                        (SCREEN_HEIGHT / 2) as f32,
                    ));
                }
                if window.is_key_pressed(KeyboardKey::KEY_M)
                    || window.is_key_pressed(KeyboardKey::KEY_BACKSPACE)
                {
                    level = None;
                    screen = Screen::Menu;
                    window.enable_cursor();
                }
            }
        }
        framebuffer.clear();
        if let Some(game) = &level
            && screen == Screen::Playing
        {
            let boss = game
                .enemies
                .iter()
                .find(|enemy| enemy.boss && enemy.hp > 0)
                .map(|_| &boss_sprite)
                .and_then(Option::as_ref);
            let assets = RenderAssets {
                wall_texture: wall_texture.as_ref(),
                enemy_sprite: enemy_sprite.as_ref(),
                boss_sprite: boss,
                animation_time: game.animation_time,
            };
            render_world(
                &mut framebuffer,
                &game.map,
                &game.player,
                &assets,
                &game.enemies,
            );
            render_minimap(&mut framebuffer, &game.map, &game.player, true);
        }
        framebuffer.present(&mut window, &thread, &mut screen_texture);
        let mut draw = window.begin_drawing(&thread);
        match screen {
            Screen::Menu => draw_menu(&mut draw, selected_level),
            Screen::Playing => {
                if let Some(game) = &level {
                    draw_hud(&mut draw, game);
                    draw_crosshair(
                        &mut draw,
                        game.weapon,
                        game.shot_timer > 0.0,
                        game.has_aim_target(),
                    );
                    if game.muzzle_timer > 0.0 {
                        draw.draw_rectangle(
                            SCREEN_WIDTH / 2 - 3,
                            SCREEN_HEIGHT / 2 - 3,
                            6,
                            6,
                            Color::YELLOW,
                        );
                    }
                    if game.player_hurt_timer > 0.0 {
                        let alpha = if (game.animation_time * 24.0).floor() as i32 % 2 == 0 {
                            160
                        } else {
                            45
                        };
                        draw.draw_rectangle(
                            0,
                            0,
                            SCREEN_WIDTH,
                            SCREEN_HEIGHT,
                            Color::new(220, 0, 0, alpha),
                        );
                    }
                }
            }
            Screen::Victory => draw_end(&mut draw, true),
            Screen::Defeat => draw_end(&mut draw, false),
        }
        if screen == Screen::Playing
            && let Some(game) = &level
            && game.fired_this_frame
        {
            if game.weapon == game::Weapon::Pistol {
                if let Some(sound) = &pistol_sound {
                    sound.play();
                }
            } else if let Some(sound) = &shotgun_sound {
                sound.play();
            }
        }
    }
}

fn draw_menu(draw: &mut RaylibDrawHandle, selected: usize) {
    draw.clear_background(Color::new(10, 12, 18, 255));
    draw.draw_text("WOLFENSTEIN // RUST", 250, 120, 44, Color::GOLD);
    draw.draw_text("SELECCIONA UN NIVEL", 335, 220, 24, Color::LIGHTGRAY);
    draw.draw_text(
        if selected == 1 {
            "> 1. OPERACION NACHT"
        } else {
            "  1. OPERACION NACHT"
        },
        330,
        290,
        24,
        Color::WHITE,
    );
    draw.draw_text(
        if selected == 2 {
            "> 2. EL LABERINTO"
        } else {
            "  2. EL LABERINTO"
        },
        330,
        335,
        24,
        Color::WHITE,
    );
    draw.draw_text(
        "ENTER / CLICK: comenzar     ESC: salir",
        300,
        500,
        18,
        Color::GRAY,
    );
}
fn draw_hud(draw: &mut RaylibDrawHandle, game: &Level) {
    draw.draw_rectangle(
        0,
        SCREEN_HEIGHT - 58,
        SCREEN_WIDTH,
        58,
        Color::new(12, 12, 15, 230),
    );
    draw.draw_text("SALUD", 24, SCREEN_HEIGHT - 42, 18, Color::WHITE);
    let bar_x = 100;
    let bar_y = SCREEN_HEIGHT - 43;
    let bar_width = 220;
    let bar_height = 22;
    let health_ratio = (game.health.max(0) as f32 / 100.0).clamp(0.0, 1.0);
    let health_color = if health_ratio > 0.6 {
        Color::new(50, 210, 90, 255)
    } else if health_ratio > 0.3 {
        Color::new(240, 190, 45, 255)
    } else {
        Color::new(220, 45, 45, 255)
    };
    draw.draw_rectangle(
        bar_x,
        bar_y,
        bar_width,
        bar_height,
        Color::new(55, 55, 60, 255),
    );
    draw.draw_rectangle(
        bar_x + 3,
        bar_y + 3,
        ((bar_width - 6) as f32 * health_ratio) as i32,
        bar_height - 6,
        health_color,
    );
    draw.draw_text(
        &format!(
            "ARMA: {}    ENEMIGOS: {}",
            game.weapon.name(),
            game.alive_enemies()
        ),
        355,
        SCREEN_HEIGHT - 40,
        20,
        Color::WHITE,
    );
    draw.draw_text(
        "1/2 cambiar arma | click/ESPACIO disparar",
        570,
        SCREEN_HEIGHT - 38,
        16,
        Color::LIGHTGRAY,
    );
}
fn draw_end(draw: &mut RaylibDrawHandle, victory: bool) {
    draw.clear_background(if victory {
        Color::new(20, 60, 42, 255)
    } else {
        Color::new(65, 18, 20, 255)
    });
    draw.draw_text(
        if victory {
            "MISION CUMPLIDA"
        } else {
            "HAS SIDO DERROTADO"
        },
        285,
        210,
        42,
        Color::GOLD,
    );
    draw.draw_text("ENTER: repetir nivel", 350, 320, 24, Color::WHITE);
    draw.draw_text("M / BACKSPACE: volver al menu", 305, 365, 24, Color::WHITE);
}

fn draw_crosshair(
    draw: &mut RaylibDrawHandle,
    weapon: game::Weapon,
    reloading: bool,
    target: bool,
) {
    let center = Vector2::new((SCREEN_WIDTH / 2) as f32, (SCREEN_HEIGHT / 2) as f32);
    let color = if reloading {
        Color::RED
    } else if target {
        Color::new(255, 150, 80, 255)
    } else {
        Color::WHITE
    };
    let (gap, length) = match weapon {
        game::Weapon::Pistol => (6.0, 10.0),
        game::Weapon::Shotgun => (11.0, 18.0),
    };
    draw.draw_line_v(
        Vector2::new(center.x - gap - length, center.y),
        Vector2::new(center.x - gap, center.y),
        color,
    );
    draw.draw_line_v(
        Vector2::new(center.x + gap, center.y),
        Vector2::new(center.x + gap + length, center.y),
        color,
    );
    draw.draw_line_v(
        Vector2::new(center.x, center.y - gap - length),
        Vector2::new(center.x, center.y - gap),
        color,
    );
    draw.draw_line_v(
        Vector2::new(center.x, center.y + gap),
        Vector2::new(center.x, center.y + gap + length),
        color,
    );
    draw.draw_circle_v(center, 2.0, color);
}
