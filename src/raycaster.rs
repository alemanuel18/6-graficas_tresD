//! Intersección de rayos con el mapa mediante DDA (Digital Differential Analyzer).

use raylib::prelude::Vector2;

use crate::map::{Map, TILE_SIZE};

/// El eje de la cuadrícula que cruzó el rayo al impactar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HitSide {
    Vertical,
    Horizontal,
}

/// Resultado de un rayo. `distance` está medida desde el jugador en píxeles.
#[derive(Debug, Clone, Copy)]
pub struct RayHit {
    pub distance: f32,
    pub position: Vector2,
    pub wall: char,
    pub side: HitSide,
}

/// Recorre únicamente las celdas que atraviesa el rayo. A diferencia de
/// avanzar de píxel en píxel, DDA es preciso y su coste depende del mapa.
pub fn cast_ray(map: &Map, origin: Vector2, angle: f32) -> RayHit {
    let direction = Vector2::new(angle.cos(), angle.sin());
    let mut map_x = (origin.x / TILE_SIZE).floor() as i32;
    let mut map_y = (origin.y / TILE_SIZE).floor() as i32;

    let delta_x = if direction.x.abs() < f32::EPSILON {
        f32::INFINITY
    } else {
        (TILE_SIZE / direction.x).abs()
    };
    let delta_y = if direction.y.abs() < f32::EPSILON {
        f32::INFINITY
    } else {
        (TILE_SIZE / direction.y).abs()
    };

    let (step_x, mut side_x) = if direction.x < 0.0 {
        (
            -1,
            (origin.x - map_x as f32 * TILE_SIZE) / direction.x.abs(),
        )
    } else {
        (
            1,
            ((map_x + 1) as f32 * TILE_SIZE - origin.x) / direction.x.abs(),
        )
    };
    let (step_y, mut side_y) = if direction.y < 0.0 {
        (
            -1,
            (origin.y - map_y as f32 * TILE_SIZE) / direction.y.abs(),
        )
    } else {
        (
            1,
            ((map_y + 1) as f32 * TILE_SIZE - origin.y) / direction.y.abs(),
        )
    };

    // El borde exterior se trata como sólido, por eso este límite es sólo una
    // protección frente a mapas corruptos, no un comportamiento normal.
    for _ in 0..(map.width() + map.height()) * 4 {
        let (distance, side) = if side_x < side_y {
            let distance = side_x;
            side_x += delta_x;
            map_x += step_x;
            (distance, HitSide::Vertical)
        } else {
            let distance = side_y;
            side_y += delta_y;
            map_y += step_y;
            (distance, HitSide::Horizontal)
        };

        if map.is_wall_cell(map_x, map_y) {
            return RayHit {
                distance,
                position: Vector2::new(
                    origin.x + direction.x * distance,
                    origin.y + direction.y * distance,
                ),
                wall: map.cell(map_x, map_y).unwrap_or('#'),
                side,
            };
        }
    }

    unreachable!("un mapa posee siempre un borde sólido o se alcanza su exterior")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ray_reports_the_correct_distance() {
        let map = Map::from_text("+++\n+ +\n+++").unwrap();
        let hit = cast_ray(&map, Map::cell_center(1, 1), 0.0);
        assert!((hit.distance - TILE_SIZE / 2.0).abs() < 0.01);
        assert_eq!(hit.side, HitSide::Vertical);
    }
}
