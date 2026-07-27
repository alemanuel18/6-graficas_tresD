//! Renderizado de columnas 3D y mapa cenital de depuración.

use raylib::prelude::*;

use crate::{
    framebuffer::Framebuffer,
    map::{Map, TILE_SIZE},
    player::Player,
    raycaster::{HitSide, RayHit, cast_ray},
};

const CEILING: Color = Color::new(27, 36, 54, 255);
const FLOOR: Color = Color::new(43, 37, 32, 255);
const MINIMAP_SCALE: f32 = 0.38;

/// Genera una columna por cada píxel horizontal: cielo, muro y suelo.
/// La distancia se corrige con coseno para eliminar el ojo de pez.
pub fn render_world(framebuffer: &mut Framebuffer, map: &Map, player: &Player) {
    let width = framebuffer.width();
    let height = framebuffer.height();
    let horizon = height / 2;
    framebuffer.rectangle(0, 0, width, horizon, CEILING);
    framebuffer.rectangle(0, horizon, width, height - horizon, FLOOR);

    let projection_plane = width as f32 / (2.0 * (player.fov / 2.0).tan());
    for column in 0..width {
        let normalized_x = (column as f32 + 0.5) / width as f32;
        let ray_angle = player.angle - player.fov / 2.0 + normalized_x * player.fov;
        let hit = cast_ray(map, player.position, ray_angle);
        let perpendicular_distance = (hit.distance * (ray_angle - player.angle).cos()).max(0.001);
        let wall_height = (TILE_SIZE / perpendicular_distance * projection_plane) as i32;
        let top = horizon - wall_height / 2;
        let bottom = horizon + wall_height / 2;
        framebuffer.vertical_line(column, top, bottom, wall_color(hit));
    }
}

fn wall_color(hit: RayHit) -> Color {
    let base = match hit.wall {
        '+' => Color::new(204, 79, 70, 255),
        '-' => Color::new(185, 128, 61, 255),
        '|' => Color::new(69, 129, 173, 255),
        _ => Color::new(158, 77, 159, 255),
    };
    match hit.side {
        HitSide::Vertical => base,
        // Oscurecer una orientación crea la lectura de profundidad de los
        // raycasters clásicos sin requerir luces ni texturas.
        HitSide::Horizontal => Color::new(base.r / 2, base.g / 2, base.b / 2, 255),
    }
}

/// Dibuja el nivel, jugador y algunos rayos. En modo 3D cabe como minimapa;
/// en modo mapa ocupa la pantalla y permite inspeccionar DDA y colisiones.
pub fn render_minimap(framebuffer: &mut Framebuffer, map: &Map, player: &Player, overlay: bool) {
    let available_width = framebuffer.width() as f32 - 24.0;
    let available_height = framebuffer.height() as f32 - 48.0;
    let scale = if overlay {
        MINIMAP_SCALE
    } else {
        (available_width / (map.width() as f32 * TILE_SIZE))
            .min(available_height / (map.height() as f32 * TILE_SIZE))
            .min(1.5)
    };
    let cell_size = (TILE_SIZE * scale).max(2.0) as i32;
    let origin = if overlay {
        Vector2::new(12.0, 12.0)
    } else {
        Vector2::new(12.0, 24.0)
    };
    let map_width = map.width() as i32 * cell_size;
    let map_height = map.height() as i32 * cell_size;

    framebuffer.rectangle(
        origin.x as i32 - 3,
        origin.y as i32 - 3,
        map_width + 6,
        map_height + 6,
        Color::new(220, 220, 220, 255),
    );
    for y in 0..map.height() as i32 {
        for x in 0..map.width() as i32 {
            let cell = map.cell(x, y).unwrap_or('#');
            let color = if map.is_wall_cell(x, y) {
                Color::new(62, 74, 91, 255)
            } else {
                Color::new(215, 205, 180, 255)
            };
            framebuffer.rectangle(
                origin.x as i32 + x * cell_size,
                origin.y as i32 + y * cell_size,
                cell_size - 1,
                cell_size - 1,
                color,
            );
            if cell == 'g' {
                framebuffer.rectangle(
                    origin.x as i32 + x * cell_size + cell_size / 4,
                    origin.y as i32 + y * cell_size + cell_size / 4,
                    cell_size / 2,
                    cell_size / 2,
                    Color::GOLD,
                );
            }
        }
    }

    // Sólo unos rayos en el mapa: mostrar los 960 escondería el laberinto.
    for ray in 0..17 {
        let fraction = ray as f32 / 16.0;
        let angle = player.angle - player.fov / 2.0 + fraction * player.fov;
        let hit = cast_ray(map, player.position, angle);
        line(
            framebuffer,
            world_to_screen(player.position, origin, scale),
            world_to_screen(hit.position, origin, scale),
            Color::new(238, 188, 80, 180),
        );
    }

    let player_position = world_to_screen(player.position, origin, scale);
    let radius = (6.0 * scale).max(3.0) as i32;
    framebuffer.rectangle(
        player_position.x as i32 - radius,
        player_position.y as i32 - radius,
        radius * 2 + 1,
        radius * 2 + 1,
        Color::RED,
    );
    let forward_end = Vector2::new(
        player.position.x + player.forward().x * TILE_SIZE,
        player.position.y + player.forward().y * TILE_SIZE,
    );
    line(
        framebuffer,
        player_position,
        world_to_screen(forward_end, origin, scale),
        Color::MAROON,
    );
}

fn world_to_screen(point: Vector2, origin: Vector2, scale: f32) -> Vector2 {
    Vector2::new(origin.x + point.x * scale, origin.y + point.y * scale)
}

/// Bresenham evita depender del renderer de Raylib durante el render software.
fn line(framebuffer: &mut Framebuffer, from: Vector2, to: Vector2, color: Color) {
    let (mut x0, mut y0) = (from.x.round() as i32, from.y.round() as i32);
    let (x1, y1) = (to.x.round() as i32, to.y.round() as i32);
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut error = dx + dy;
    loop {
        framebuffer.pixel(x0, y0, color);
        if x0 == x1 && y0 == y1 {
            break;
        }
        let twice_error = 2 * error;
        if twice_error >= dy {
            error += dy;
            x0 += sx;
        }
        if twice_error <= dx {
            error += dx;
            y0 += sy;
        }
    }
}
