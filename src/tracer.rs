use base::Vector3;
use base::Light;
use base::Color;
use base::Surface;
use base::Ray;

use primitives::Primitive;

use std::f64;

const TRACER_MAXREC:i32 = 3;
const TRACER_EPSILON:f64 = 0.0001;

#[derive(Debug)]
pub struct Tracer {
    camera_eye: Vector3,
    camera_look_at: Vector3,
    camera_up: Vector3,
    camera_fov: f64,

    width: i64,
    height: i64,

    u: Vector3,
    v: Vector3,
    w: Vector3,
    lu: f64,
    lv: f64,

    primitives: Vec<Box<Primitive>>,
    lights: Vec<Light>,
    world_color: Color,

    changed: bool,
    pixels: Vec<(i64, i64, Color)>

}

impl Tracer {
    // public
    pub fn new(
        a_width: i64,
        a_height: i64,
        a_camera_eye: Vector3,
        a_camera_look_at: Vector3
    ) -> Tracer{
        let mut t = Tracer {
            camera_eye: a_camera_eye,
            camera_look_at: a_camera_look_at,
            camera_up: Vector3::new(0.0, 1.0, 0.0),
            camera_fov: 60.0,

            width: a_width,
            height: a_height,

            u: Vector3::new(0.0, 1.0, 0.0),
            v: Vector3::new(0.0, 1.0, 0.0),
            w: Vector3::new(0.0, 1.0, 0.0),
            lu: 0.0,
            lv: 0.0,

            primitives: Vec::new(),
            lights: Vec::new(),
            world_color: Color::new(0.0, 0.0, 0.0),

            changed: false,
            pixels: Vec::<(i64, i64, Color)>::with_capacity((a_width * a_height) as usize)
        };
        t.calc_vectors();
        return t;
    }
    pub fn get_camera_eye(&self) -> Vector3 {
        return self.camera_eye;
    }
    pub fn set_camera_eye(&mut self, a_eye: Vector3) {
        self.changed = true;
        self.camera_eye = a_eye;
    }
    pub fn get_camera_look_at(&self) -> Vector3 {
        return self.camera_look_at;
    }
    pub fn set_camera_look_at(&mut self, a_look_at: Vector3) {
        self.changed = true;
        self.camera_look_at = a_look_at;
    }
    pub fn get_world_color(&self) -> Color {
        return self.world_color;
    }
    pub fn set_world_color(&mut self, a_color: Color) {
        self.changed = true;
        self.world_color = a_color;
    }
    pub fn add_primitive(&mut self, a_primitive: Box<Primitive>) {
        self.changed = true;
        self.primitives.push(a_primitive);
    }
    pub fn clear_primitives(&mut self) {
        self.changed = true;
        self.primitives.clear();
    }
    pub fn add_light(&mut self, a_light: Light) {
        self.changed = true;
        self.lights.push(a_light);
    }
    pub fn clear_lights(&mut self) {
        self.changed = true;
        self.lights.clear();
    }
    pub fn get_pixels(&mut self) -> &Vec<(i64, i64, Color)> {
        if self.changed {
            self.changed = false;
            self.trace();
        }
        return &self.pixels;
    }

    // private
    fn trace(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let ray = self.generate_ray(x, y);
                let color = self.cast_ray(ray, 0);

                self.pixels.push((x, y, color));
            }
        }
    }
    fn calc_vectors(&mut self) {
        // calc u, v, w
        self.w = self.camera_eye - self.camera_look_at;
        self.w.normalize();

        self.u = self.camera_up.cross_product(self.camera_eye - self.camera_look_at);
        self.u.normalize();

        self.v = self.w.cross_product(self.u);

        // calc lu, lv
        let view_length = self.camera_eye - self.camera_look_at;
        self.lv = view_length.magnitude() * (self.camera_fov / 57.2958).sin();
        self.lu = (self.width as f64 / self.height as f64) * self.lv;
    }
    fn generate_ray(&self, a_x: i64, a_y: i64) -> Ray {
        let cu:f64 = (((2.0 * a_x as f64 + 1.0) / (2.0 * self.width as f64)) - 0.5) * self.lu;
        let cv:f64 = (((2.0 * a_y as f64 + 1.0) / (2.0 * self.height as f64)) - 0.5) * self.lv;

        let p = self.camera_look_at + (self.u * cu) + (self.v * cv);
        let mut t = p - self.camera_eye;
        t.normalize();

        return Ray {
            eye: self.camera_eye,
            direction: t
        };
    }
    fn cast_ray(&self, a_ray: Ray, a_rec_level: i32) -> Color {
        let mut shortest_dist = f64::MAX;
        let mut t:f64;
        let mut np = None;

        for p in self.primitives.iter() {
            t = p.intersect_ray(a_ray);
            if (t > 0.0) && t < shortest_dist {
                shortest_dist = t;
                np = Some(p);
            }
        }

        if np.is_some() && shortest_dist > 0.0 {
            let nearest_prim = np.unwrap();
            // Calculate components for phong model
            let p = a_ray.eye + (a_ray.direction * shortest_dist);
            let n = nearest_prim.get_surface_normal(p);
            let v = a_ray.direction;

            if a_rec_level < TRACER_MAXREC {
                // Reflection
                let r = v - n * 2.0 * v.dot_product(n);
                let reflection_ray = Ray {
                    eye: p + r * TRACER_EPSILON,
                    direction: r
                };

                let orig_color = self.calculate_phong(p, n, v, nearest_prim.get_surface());
                let ref_color = self.cast_ray(reflection_ray, a_rec_level + 1);
                let interp_color = (orig_color * (1.0 - nearest_prim.get_surface().reflect)) + (ref_color * nearest_prim.get_surface().reflect);

                return interp_color;
            } else {
                return self.calculate_phong(p, n, v, nearest_prim.get_surface());
            }
        }
        return self.world_color;
    }
    fn calculate_phong(&self, a_p: Vector3, a_n: Vector3, a_v: Vector3, a_surface: Surface) -> Color {
        // Sum ambient terms for lights for ia
        let mut ia = Color::new(0.0, 0.0, 0.0);
        for light in self.lights.iter() {
            ia = ia + light.ambient;
        }

        // Apply phong model for each light
        let mut sum_colors = Color::new(0.0, 0.0, 0.0);
        for light in self.lights.iter() {
            let mut l = a_p + light.position;
            l.normalize();

            let r = l - a_n * 2.0 * l.dot_product(a_n);
            if l.dot_product(a_n) > 0.0 {
                let diffuse = a_surface.diffuse * l.dot_product(a_n) * light.diffuse;
                let specular = a_surface.specular * r.dot_product(a_v).powf(a_surface.shiny) * light.specular;

                sum_colors = sum_colors + diffuse + specular;
            }
        }

        // Apply ambient term
        return ia + sum_colors;
    }
}
