use base::Vector3;
use base::Light;
use base::Color;
use base::Surface;

use std::fmt::Debug;
use std::f64;

const TRACER_MAXREC:i32 = 3;
const TRACER_EPSILON:f64 = 0.0001;

pub struct Ray {
    pub eye: Vector3,
    pub direction: Vector3
}

pub trait Primitive: Debug {
    fn intersect_ray(&self, ray: Ray) -> f64;
    fn get_surface_normal(&self, p: Vector3) -> f64;
}

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
}

impl Tracer {
    // public
    pub fn new(
        a_width: i64,
        a_height: i64,
        a_camera_eye: Vector3,
        a_camera_look_at: Vector3
    ) -> Tracer{
        return Tracer {
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
            world_color: Color::new(0.0, 0.0, 0.0)
        }
    }
    pub fn set_camera(&mut self, a_eye: Vector3, a_look_at: Vector3) {
        self.camera_eye = a_eye;
        self.camera_look_at = a_look_at;
    }
    pub fn set_world_color(&mut self, a_color: Color) {
        self.world_color = a_color;
    }
    pub fn add_primitive(&mut self, a_primitive: Box<Primitive>) {
        self.primitives.push(a_primitive);
    }
    pub fn clear_primitives(&mut self) {
        self.primitives.clear();
    }
    pub fn add_light(&mut self, a_light: Light) {
        self.lights.push(a_light);
    }
    pub fn clear_lights(&mut self) {
        self.lights.clear();
    }
    pub fn trace(&self, on_calc_pixel: fn(x: f64, y: f64, color: Color)) {
        // TODO
    }

    // private
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
        let shortest_dist = f64::MAX;
        let t:f64 = 0.0;
        let np  = None;

        for p in self.primitives {
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

            if(a_rec_level < TRACER_MAXREC){
                // Reflection
                let r = v - n * 2.0 * v.dot_product(n);
                let reflection_ray = Ray {
                    eye: p + r * TRACER_EPSILON,
                    direction: r
                };

                let orig_color = self.calculate_phong(p, n, v, nearest_prim.surface);
                let ref_color = self.cast_ray(reflection_ray, a_rec_level + 1);
                let interp_color = (orig_color * (1 - nearest_prim.surface.reflect)) + (ref_color * nearest_prim.surface.reflect);
                return interpColor;
            } else {
                return self.calculate_phong(p, n, v, nearest_prim.surface);
            }
        }
        return self.world_color;
    }
    fn calculate_phong(&self, a_p: Vector3, a_n: Vector3, a_v: Vector3, a_surface: Surface) -> Color {
        // TODO
        return Color::new(0.0, 0.0, 0.0);
    }
}
