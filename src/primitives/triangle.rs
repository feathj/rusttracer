use base::Vector3;
use base::Surface;
use base::Ray;
use primitives::Primitive;

#[derive(Debug, Copy, Clone)]
pub struct Triangle {
    pub p1:Vector3,
    pub p2:Vector3,
    pub p3:Vector3,
    pub surface:Surface
}

impl Triangle {
    pub fn new(a_p1:Vector3, a_p2:Vector3, a_p3:Vector3) -> Triangle {
        return Triangle {
            p1: a_p1,
            p2: a_p2,
            p3: a_p3,
            surface: Surface::new()
        }
    }
}

impl Primitive for Triangle {
    fn intersect_ray(&self, ray: Ray) -> f64 {
        // Grab normal
        let n = self.get_surface_normal(Vector3::new(0.0, 0.0, 0.0));
        let a = n.x;
        let b = n.y;
        let c = n.z;

        // solve for d on p1
        let d = -((a*self.p1.x) + (b*self.p1.y) + (c*self.p1.z));

        // solve for t
        let top = (a * ray.eye.x) + (b * ray.eye.y) + (c * ray.eye.z) + d;
        let bottom = (a * ray.direction.x) + (b * ray.direction.y) + (c * ray.direction.z);
        let t = -(top / bottom);

        // we didn't intersect with plane at all
        if t < 0.0 {
            return 0.0;
        }

        // check if we are in triangle using BC
        // use standard names (for my sanity)
        let ps = self.p1;
        let pt = self.p2;
        let pu = self.p3;
        let p =  ray.eye + (ray.direction * t);

        let area_triangle = ((pt - ps).cross_product(pu - ps)).magnitude();

        if area_triangle > 0.0 {

            let s1 = ((pt - p).cross_product(pu - p)).magnitude() / area_triangle;
            let t1 = ((pu - p).cross_product(ps - p)).magnitude() / area_triangle;
            let u1 = ((ps - p).cross_product(pt - p)).magnitude() / area_triangle;

            let total_area = s1 + t1 + u1;

            // all three areas will add up to about 1
            // check the difference against an error
            if (1.0 - total_area).abs() <= 0.0001 {
                return t;
            }
        }

        return 0.0;
    }
    fn get_surface_normal(&self, _p: Vector3) -> Vector3 {
        // n = (P2 - P1) X (P3 - P1) or the cross prod of two non parallel edges of the polygon
        let p2p1 = self.p2 - self.p1;
        let p3p1 = self.p3 - self.p1;
        let mut normal = p2p1.cross_product(p3p1);

        //normal = position + normal;
        normal.normalize();
        return normal;
    }
    fn get_surface(&self) -> Surface {
        return self.surface;
    }
}
