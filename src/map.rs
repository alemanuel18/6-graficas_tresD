//! Carga y consulta del mapa de celdas.

use std::{fs, io, path::Path};

use raylib::prelude::Vector2;

/// Longitud en píxeles de una celda lógica del nivel.
pub const TILE_SIZE: f32 = 32.0;

/// El mapa ASCII. Espacio, `p`, `g`, `e` y `b` son transitables; el resto son paredes.
#[derive(Debug, Clone)]
pub struct Map {
    cells: Vec<Vec<char>>,
    width: usize,
}

impl Map {
    /// Lee un mapa rectangular desde texto y conserva los espacios finales.
    pub fn load(path: impl AsRef<Path>) -> io::Result<Self> {
        let content = fs::read_to_string(path)?;
        Self::from_text(&content)
    }

    /// Construye un mapa a partir de texto; resulta útil para pruebas y para
    /// niveles generados en tiempo de ejecución.
    pub fn from_text(content: &str) -> io::Result<Self> {
        let mut cells: Vec<Vec<char>> =
            content.lines().map(|line| line.chars().collect()).collect();

        let width = cells.iter().map(Vec::len).max().unwrap_or(0);
        if width == 0 || cells.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "el mapa está vacío",
            ));
        }
        // Las filas cortas se cierran con pared: un archivo mal alineado no
        // puede crear una salida accidental hacia el exterior del nivel.
        for row in &mut cells {
            row.resize(width, '#');
        }

        Ok(Self { cells, width })
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.cells.len()
    }

    pub fn cell(&self, x: i32, y: i32) -> Option<char> {
        if x < 0 || y < 0 {
            return None;
        }
        self.cells.get(y as usize)?.get(x as usize).copied()
    }

    pub fn is_wall_cell(&self, x: i32, y: i32) -> bool {
        self.cell(x, y)
            .is_none_or(|cell| !matches!(cell, ' ' | 'p' | 'g' | 'e' | 'b'))
    }

    /// Evita que el jugador atraviese paredes incluyendo un pequeño radio físico.
    pub fn collides_circle(&self, position: Vector2, radius: f32) -> bool {
        let samples = [
            (-radius, -radius),
            (radius, -radius),
            (-radius, radius),
            (radius, radius),
        ];
        samples.iter().any(|(x, y)| {
            let cell_x = ((position.x + x) / TILE_SIZE).floor() as i32;
            let cell_y = ((position.y + y) / TILE_SIZE).floor() as i32;
            self.is_wall_cell(cell_x, cell_y)
        })
    }

    /// Busca la marca `p`; si falta, usa el centro de la primera celda libre.
    pub fn player_spawn(&self) -> Vector2 {
        for (y, row) in self.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell == 'p' {
                    return Self::cell_center(x, y);
                }
            }
        }
        for (y, row) in self.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell == ' ' {
                    return Self::cell_center(x, y);
                }
            }
        }
        Vector2::new(TILE_SIZE / 2.0, TILE_SIZE / 2.0)
    }

    pub fn positions_of(&self, marker: char) -> Vec<Vector2> {
        self.cells
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, cell)| **cell == marker)
                    .map(move |(x, _)| Self::cell_center(x, y))
            })
            .collect()
    }

    pub fn cell_center(x: usize, y: usize) -> Vector2 {
        Vector2::new((x as f32 + 0.5) * TILE_SIZE, (y as f32 + 0.5) * TILE_SIZE)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn any_non_empty_symbol_is_a_wall() {
        let map = Map::from_text(" +").unwrap();
        assert!(!map.is_wall_cell(0, 0));
        assert!(map.is_wall_cell(1, 0));
        assert!(map.is_wall_cell(2, 0));
    }
}
