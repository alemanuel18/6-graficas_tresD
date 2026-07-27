//! Entrada y colisión del jugador.

use raylib::prelude::*;

use crate::{map::Map, player::Player};

const MOVE_SPEED: f32 = 105.0;
const TURN_SPEED: f32 = 2.4;
const PLAYER_RADIUS: f32 = 7.0;

/// Actualiza orientación y posición. Cada componente se prueba por separado
/// para que el jugador se deslice junto a una pared en vez de quedar bloqueado.
pub fn update_player(player: &mut Player, map: &Map, window: &RaylibHandle, delta: f32) {
    let turn = (window.is_key_down(KeyboardKey::KEY_RIGHT) as i32
        - window.is_key_down(KeyboardKey::KEY_LEFT) as i32) as f32;
    player.angle = (player.angle + turn * TURN_SPEED * delta).rem_euclid(std::f32::consts::TAU);

    let forward = player.forward();
    let right = Vector2::new(-forward.y, forward.x);
    let forward_input = (window.is_key_down(KeyboardKey::KEY_W) as i32
        + window.is_key_down(KeyboardKey::KEY_UP) as i32
        - window.is_key_down(KeyboardKey::KEY_S) as i32
        - window.is_key_down(KeyboardKey::KEY_DOWN) as i32) as f32;
    let strafe_input = (window.is_key_down(KeyboardKey::KEY_D) as i32
        - window.is_key_down(KeyboardKey::KEY_A) as i32) as f32;

    let displacement = Vector2::new(
        (forward.x * forward_input + right.x * strafe_input) * MOVE_SPEED * delta,
        (forward.y * forward_input + right.y * strafe_input) * MOVE_SPEED * delta,
    );
    let next_x = Vector2::new(player.position.x + displacement.x, player.position.y);
    if !map.collides_circle(next_x, PLAYER_RADIUS) {
        player.position.x = next_x.x;
    }
    let next_y = Vector2::new(player.position.x, player.position.y + displacement.y);
    if !map.collides_circle(next_y, PLAYER_RADIUS) {
        player.position.y = next_y.y;
    }
}
