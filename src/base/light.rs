use base::color::Color;
use base::vector3::Vector3;

#[derive(Debug)]
pub struct Light {
    pub ambient: Color,
    pub diffuse: Color,
    pub specular: Color,
    pub position: Vector3
}

impl Light {
    pub fn new(a_x: f64, a_y: f64, a_z: f64) -> Light {
        return Light {
            ambient: Color::new(0.1, 0.1, 0.1),
            diffuse: Color::new(1.0, 1.0, 1.0),
            specular: Color::new(1.0, 1.0, 1.0),
            position: Vector3::new(a_x, a_y, a_z)
        }
    }
}

impl PartialEq for Light {
    fn eq(&self, other:&Light) -> bool {
        return self.ambient == other.ambient
            && self.diffuse == other.diffuse
            && self.specular == other.specular
            && self.position == other.position
    }
}
