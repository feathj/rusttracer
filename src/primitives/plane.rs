use base::Vector3;
use base::Surface;
use base::Ray;
use primitives::Primitive;

#[derive(Debug, Copy, Clone)]
pub struct Plane {
    pub position:Vector3,
    pub normal:Vector3,
    pub surface:Surface
}

impl Plane {
    pub fn new(a_position:Vector3, a_normal:Vector3) -> Plane {
        return Plane {
            position: a_position,
            normal: a_normal,
            surface:  Surface::new()
        }
    }
}

impl Primitive for Plane {
    fn intersect_ray(&self, ray: Ray) -> f64 {
        let a = self.normal.x;
        let b = self.normal.y;
        let c = self.normal.z;

        let d = -((a*self.position.x) + (b*self.position.y) + (c*self.position.z));

        // solve for t
        let top = (a * ray.eye.x) + (b * ray.eye.y) + (c * ray.eye.z) + d;
        let bottom = (a * ray.direction.x) + (b * ray.direction.y) + (c * ray.direction.z);
        let t = -(top / bottom);

        if t < 0.0 {
            return 0.0;
        }
        return t;
    }
    fn get_surface_normal(&self, _p: Vector3) -> Vector3 {
        return self.normal;
    }
    fn get_surface(&self) -> Surface {
        return self.surface;
    }
}
