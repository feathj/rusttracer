use base::Vector3;
use base::Surface;
use base::Ray;
use primitives::Primitive;

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    pub position:Vector3,
    pub r:f64,
    pub surface:Surface
}

impl Sphere {
    pub fn new(a_position:Vector3, a_r:f64) -> Sphere {
        return Sphere {
            position: a_position,
            r: a_r,
            surface:  Surface::new()
        }
    }
}


impl Primitive for Sphere {
    fn intersect_ray(&self, ray: Ray) -> f64 {
        let a = (ray.direction.x * ray.direction.x) + (ray.direction.y * ray.direction.y) + (ray.direction.z * ray.direction.z);
        let b = (((ray.eye.x - self.position.x) * ray.direction.x) + ((ray.eye.y - self.position.y) * ray.direction.y) + ((ray.eye.z - self.position.z) * ray.direction.z)) * 2.0;
        let c = ((ray.eye.x - self.position.x) * (ray.eye.x - self.position.x)) + ((ray.eye.y - self.position.y) * (ray.eye.y - self.position.y)) + ((ray.eye.z - self.position.z) * (ray.eye.z - self.position.z)) - (self.r * self.r);

        let discriminant = (b * b) - (4.0 * a * c);

        if discriminant >= 0.0 {
            let r1 = (-b + discriminant.sqrt()) / (2.0 * a);
            let r2 = (-b - discriminant.sqrt()) / (2.0 * a);

            if r1 <= r2 {
                return r1;
            } else {
                return r2;
            }
        }
        return 0.0;
    }
    fn get_surface_normal(&self, p: Vector3) -> Vector3 {
        let mut normal = p - self.position;
        normal.normalize();
        return normal;
    }
    fn get_surface(&self) -> Surface {
        return self.surface;
    }
}
