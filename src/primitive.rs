trait Primitive {
    pub fn intersect_ray(&self, ray: Ray) -> f64;
    pub fn get_surface_normal(&self, p: Vector3) -> f64;
}
