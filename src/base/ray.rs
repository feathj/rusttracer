use base::Vector3;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub eye: Vector3,
    pub direction: Vector3
}
