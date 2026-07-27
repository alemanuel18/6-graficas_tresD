//! Estado de la cámara/jugador.

use std::f32::consts::PI;

use raylib::prelude::Vector2;

/// Cámara de primera persona en el plano del mapa.
#[derive(Debug, Clone, Copy)]
pub struct Player {
    pub position: Vector2,
    /// Dirección de visión en radianes. Cero apunta a la derecha.
    pub angle: f32,
    /// Apertura horizontal de la cámara en radianes.
    pub fov: f32,
}

impl Player {
    pub fn new(position: Vector2) -> Self {
        Self {
            position,
            angle: 0.0,
            fov: PI / 3.0,
        }
    }

    pub fn forward(&self) -> Vector2 {
        Vector2::new(self.angle.cos(), self.angle.sin())
    }
}
