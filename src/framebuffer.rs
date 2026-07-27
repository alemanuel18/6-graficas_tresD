use raylib::prelude::*;

pub struct Framebuffer {
    width: i32,
    height: i32,
    background_color: Color,
    current_color: Color,
    image: Image,
}

impl Framebuffer {
    pub fn new(width: i32, height: i32) -> Self {
        let background_color = Color::BLACK;
        let current_color = Color::WHITE;
        let image = Image::gen_image_color(width, height, background_color);
        Framebuffer {
            width,
            height,
            background_color,
            current_color,
            image,
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn clear(&mut self) {
        self.image = Image::gen_image_color(self.width, self.height, self.background_color);
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn point(&mut self, x: i32, y: i32) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.image.draw_pixel(x, y, self.current_color);
        }
    }

    /// Devuelve el color de una celda. Las coordenadas fuera del framebuffer
    /// se consideran fondo, lo que hace que los bordes del mundo sean finitos.
    pub fn get_color(&mut self, x: i32, y: i32) -> Color {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.image.get_color(x, y)
        } else {
            self.background_color
        }
    }

    pub fn render_to_file(&self, filename: &str) {
        self.image.export_image(filename);
    }

    pub fn swap_buffers(&self, window: &mut RaylibHandle, raylib_thread: &RaylibThread) {
        if let Ok(texture) = window.load_texture_from_image(raylib_thread, &self.image) {
            let mut renderer = window.begin_drawing(raylib_thread);
            let screen_width = renderer.get_screen_width() as f32;
            let screen_height = renderer.get_screen_height() as f32;
            renderer.clear_background(Color::BLACK);
            renderer.draw_texture_pro(
                &texture,
                Rectangle::new(0.0, 0.0, self.width as f32, self.height as f32),
                Rectangle::new(0.0, 0.0, screen_width, screen_height),
                Vector2::zero(),
                0.0,
                Color::WHITE,
            );
        }
    }
}