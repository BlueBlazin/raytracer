use wasm_bindgen::prelude::*;

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
                let r = i as f64 / (self.width as f64 - 1.0);
                let g = j as f64 / (self.height as f64 - 1.0);
                let b = 0.25;

                let ir = (255.999 * r) as u8;
                let ig = (255.999 * g) as u8;
                let ib = (255.999 * b) as u8;

                let row = 255 - j;
                let col = i;

                self.screen[row * self.width * 4 + col * 4 + 0] = ir;
                self.screen[row * self.width * 4 + col * 4 + 1] = ig;
                self.screen[row * self.width * 4 + col * 4 + 2] = ib;
                self.screen[row * self.width * 4 + col * 4 + 3] = 255;
            }
        }
    }
}
