use crate::ray::Ray;
use crate::vec3::Vec3;
use wasm_bindgen::prelude::*;

fn get_rbg(c: &Vec3) -> (u8, u8, u8) {
    (
        (255.999 * c[0]) as u8,
        (255.999 * c[1]) as u8,
        (255.999 * c[2]) as u8,
    )
}

fn hit_sphere(center: Vec3, radius: f64, ray: &Ray) -> bool {
    let oc = &ray.origin - center;
    let a = ray.dir.length_squared();
    let b = 2.0 * oc.dot(&ray.dir);
    let c = oc.length_squared() - radius * radius;

    let discriminant = (b * b) - (4.0 * a * c);

    discriminant > 0.0
}

fn ray_color(ray: &Ray) -> Vec3 {
    if hit_sphere(Vec3(0.0, 0.0, -1.0), 0.5, ray) {
        return Vec3(1.0, 0.0, 0.0);
    }

    let unit_dir = &ray.dir.unit_vector();

    let t = 0.5 * (unit_dir.1 + 1.0);

    (1.0 - t) * Vec3(1.0, 1.0, 1.0) + t * Vec3(0.5, 0.7, 1.0)
}

struct Image {
    pub aspect_ratio: f64,
    pub width: usize,
    pub height: usize,
}

impl Image {
    pub fn new(width: usize) -> Self {
        let aspect_ratio = 16.0 / 9.0;

        Self {
            aspect_ratio,
            width,
            height: (width as f64 / aspect_ratio) as usize,
        }
    }
}

struct Camera {
    pub viewport_height: f64,
    pub viewport_width: f64,
    pub focal_length: f64,
    pub origin: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left_corner: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64) -> Self {
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Vec3(0.0, 0.0, 0.0);
        let horizontal = Vec3(viewport_width, 0.0, 0.0);
        let vertical = Vec3(0.0, viewport_height, 0.0);
        let lower_left_corner =
            &origin - (&horizontal / 2.0) - (&vertical / 2.0) - Vec3(0.0, 0.0, focal_length);

        Self {
            viewport_height,
            viewport_width,
            focal_length,
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }
}

#[wasm_bindgen]
pub struct Scene {
    image: Image,
    camera: Camera,
    screen: Vec<u8>,
}

#[wasm_bindgen]
impl Scene {
    pub fn new(width: usize) -> Self {
        let image = Image::new(width);
        let screen = vec![0; image.width * image.height * 4];
        let camera = Camera::new(image.aspect_ratio);

        Self {
            image,
            screen,
            camera,
        }
    }

    pub fn screen(&self) -> *const u8 {
        self.screen.as_ptr()
    }

    pub fn render(&mut self) {
        for j in (0..self.image.height).rev() {
            for i in 0..self.image.width {
                // let (i, j) = (col, self.image.height - row - 1);

                let u = i as f64 / (self.image.width as f64 - 1.0);
                let v = j as f64 / (self.image.height as f64 - 1.0);

                let origin = Vec3(0.0, 0.0, 0.0);

                let direction = &self.camera.lower_left_corner
                    + u * &self.camera.horizontal
                    + v * &self.camera.vertical
                    - &origin;

                let ray = Ray::new(origin, direction);

                let color = ray_color(&ray);

                self.write_pixel(color, self.image.height - 1 - j, i);
            }
        }
    }

    fn write_pixel(&mut self, color: Vec3, row: usize, col: usize) {
        let (r, g, b) = get_rbg(&color);

        self.screen[row * self.image.width * 4 + col * 4 + 0] = r;
        self.screen[row * self.image.width * 4 + col * 4 + 1] = g;
        self.screen[row * self.image.width * 4 + col * 4 + 2] = b;
        self.screen[row * self.image.width * 4 + col * 4 + 3] = 255;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scene() {
        let mut scene = Scene::new(400);
        scene.render();
    }
}
