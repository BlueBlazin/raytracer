use crate::vec3::Vec3;
use wasm_bindgen::prelude::*;

fn get_rbg(c: &Vec3) -> (u8, u8, u8) {
    (
        (255.999 * c[0]) as u8,
        (255.999 * c[1]) as u8,
        (255.999 * c[2]) as u8,
    )
}

#[wasm_bindgen]
pub struct RayTracer {
    width: usize,
    height: usize,
    screen: Vec<u8>,
}

#[wasm_bindgen]
impl RayTracer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            screen: vec![0; width * height * 4],
        }
    }

    pub fn screen(&self) -> *const u8 {
        self.screen.as_ptr()
    }

    pub fn render(&mut self) {
        for j in (0..self.height).rev() {
            for i in 0..self.width {
                let pixel_color = Vec3(
                    i as f64 / (self.width as f64 - 1.0),
                    j as f64 / (self.height as f64 - 1.0),
                    0.25,
                );

                let (r, g, b) = get_rbg(&pixel_color);

                let row = 255 - j;
                let col = i;

                self.screen[row * self.width * 4 + col * 4 + 0] = r;
                self.screen[row * self.width * 4 + col * 4 + 1] = g;
                self.screen[row * self.width * 4 + col * 4 + 2] = b;
                self.screen[row * self.width * 4 + col * 4 + 3] = 255;
            }
        }
    }
}
