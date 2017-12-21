use std::ops;

pub struct Vector3 {
    pub x:f64,
    pub y:f64,
    pub z:f64
}

impl Vector3 {
    pub fn new(a_x:f64, a_y:f64, a_z:f64) -> Vector3 {
        return Vector3 {
            x: a_x,
            y: a_y,
            z: a_z
        }
    }
    pub fn dot_product(&self, v:Vector3) -> f64 {
        return (self.x * v.x) + (self.y * v.y) + (self.z * v.z);
    }
    pub fn cross_product(&self, v:Vector3) -> Vector3 {
        return Vector3{
            x: (self.y * v.z) - (self.z * v.y),
            y: (self.z * v.x) - (self.x * v.z),
            z: (self.x * v.y) - (self.y * v.x)
        }
    }
    fn magnitude(&self) -> f64 {
        return ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt();
    }
    pub fn normalize(&mut self) {
        let l = 1.0 / self.magnitude();
        self.x *= l;
        self.y *= l;
        self.z *= l;
    }
}

impl ops::Add<Vector3> for Vector3 {
    type Output = Vector3;
    fn add(self, rhs:Vector3) -> Vector3 {
        return Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}
impl ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;
    fn sub(self, rhs:Vector3) -> Vector3 {
        return Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}
impl ops::Mul<Vector3> for Vector3 {
    type Output = Vector3;
    fn mul(self, rhs:Vector3) -> Vector3 {
        return Vector3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z
        }
    }
}
/*impl ops::Mul<Vector3> for Vector3 {
    type Output = Vector3;
    fn mul(self, rhs:f64) {
        return Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}*/
