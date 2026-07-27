//! Buffer de píxeles de CPU y puente de presentación hacia Raylib.

use raylib::prelude::*;

pub struct Framebuffer {
    width: i32,
    height: i32,
    /// RGBA8 contiguo: formato que `Texture2D::update_texture` espera.
    pixels: Vec<u8>,
}

impl Framebuffer {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            pixels: vec![0; (width * height * 4) as usize],
        }
    }

    pub fn width(&self) -> i32 {
        self.width
    }
    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn clear(&mut self) {
        self.pixels.fill(0);
    }

    pub fn pixel(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && y >= 0 && x < self.width && y < self.height {
            let offset = ((y * self.width + x) * 4) as usize;
            self.pixels[offset..offset + 4].copy_from_slice(&[color.r, color.g, color.b, color.a]);
        }
    }

    pub fn vertical_line(&mut self, x: i32, from: i32, to: i32, color: Color) {
        for y in from.max(0)..to.min(self.height) {
            self.pixel(x, y, color);
        }
    }

    pub fn rectangle(&mut self, x: i32, y: i32, width: i32, height: i32, color: Color) {
        for py in y.max(0)..(y + height).min(self.height) {
            for px in x.max(0)..(x + width).min(self.width) {
                self.pixel(px, py, color);
            }
        }
    }

    pub fn present(
        &self,
        window: &mut RaylibHandle,
        thread: &RaylibThread,
        texture: &mut Texture2D,
    ) {
        texture
            .update_texture(&self.pixels)
            .expect("el tamaño del framebuffer y de la textura debe coincidir");
        let mut draw = window.begin_drawing(thread);
        draw.clear_background(Color::BLACK);
        draw.draw_texture(texture, 0, 0, Color::WHITE);
    }
}
