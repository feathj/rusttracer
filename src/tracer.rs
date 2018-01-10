use base::vector3::Vector3;
use base::light::Light;
use base::color::Color;
use base::surface::Surface;

use std::fmt::Debug;

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

    width: f64,
    height: f64,

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
        a_width: f64,
        a_height: f64,
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
    fn generate_ray(a_x: i64, a_y: i64) -> Ray {
        // TODO
        return Ray {
            eye: Vector3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(0.0, 0.0, 0.0)
        }
    }
    fn cast_ray(a_ray: Ray, a_rec_level: i64) -> Color {
        // TODO
        return Color::new(0.0, 0.0, 0.0);
    }
    fn calculate_phong(a_p: Vector3, a_n: Vector3, a_v: Vector3, a_surface: Surface) -> Color {
        // TODO
        return Color::new(0.0, 0.0, 0.0);
    }
}
