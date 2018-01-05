use vector3::Vector3;
use light::Light;
use color::Color;
use surface::Surface;
use primitive::Primitive;

pub struct Ray {
    pub eye: Vector3,
    pub direction: Vector3
}

#[derive(Debug)]
pub struct Tracer {
    camera_eye: Vector3,
    camera_look_at: Vector3,
    camera_up: Vector3,
    camera_fov: Vector3,

    width: f64,
    height: f64,

    u: Vector3,
    v: Vector3,
    w: Vector3,
    lu: f64,
    lv: f64,

    primitives: Vec<Primitive>,
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
    ) {
        return Tracer {
            width: a_width,
            height: a_height,
            camera_eye: a_camera_eye,
            camera_look_at: a_camera_look_at,

            camera_up: Vector3::new(0, 1, 0),
            camera_fov: 60,
            primitives: Vec::new(),
            lights: Vec::new(),
            world_color: Color::new(0, 0, 0)
        }
    }
    pub fn set_camera(&mut self, a_eye: Vector3, a_look_at: Vector3) {
        self.camera_eye = a_eye;
        self.camera_look_at = a_look_at;
    }
    pub fn set_world_color(&mut self, a_color: Color) {
        self.world_color = a_color;
    }
    pub fn add_primitive(&mut self, a_primitive: Primitive) {
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
    }

    // private
    fn generate_ray(a_x: i64, a_y: i64) -> Ray {
        // TODO
        return Ray {
            eye: Vector3::new(0, 0, 0),
            direction: Vector3::new(0, 0, 0)
        }
    }
    fn cast_ray(a_ray: Ray, a_rec_level: i64) -> Color {
        // TODO
        return Color::new(0, 0, 0);
    }
    fn calculate_phong(a_p: Vector3, a_n: Vector3, a_v: Vector3, a_surface: Surface) -> Color {
        // TODO
        return Color::new(0, 0, 0);
    }
}
