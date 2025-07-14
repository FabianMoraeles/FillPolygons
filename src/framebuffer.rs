use raylib::prelude::*;

pub struct Framebuffer {
    image: Image,
    width: i32,
    height: i32,
    current_color: Color,
    background_color: Color,
}

impl Framebuffer {
    pub fn new(width: i32, height: i32) -> Self {
        let background_color = Color::BLACK;
        let mut image = Image::gen_image_color(width, height, background_color);
        Self {
            image,
            width,
            height,
            current_color: Color::WHITE,
            background_color,
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn clear(&mut self) {
        self.image = Image::gen_image_color(self.width, self.height, self.background_color);
    }

    pub fn put_pixel(&mut self, x: i32, y: i32) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.image.draw_pixel(x, y, self.current_color);
        }
    }

    pub fn render_to_file(&self, filename: &str) {
        self.image.export_image(filename);
    }
}
