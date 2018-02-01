use base::Vector3;
use base::Surface;
use base::Ray;

use std::fmt::Debug;

pub trait Primitive: Debug {
    fn intersect_ray(&self, ray: Ray) -> f64;
    fn get_surface_normal(&self, p: Vector3) -> Vector3;
    fn get_surface(&self) -> Surface;
}
