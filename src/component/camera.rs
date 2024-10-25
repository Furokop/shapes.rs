use super::*;
use crate::basetype::*;
pub struct Camera {
    pub coord: Coord,
    pub angle: Angle3D,
    /// Horizontal fov
    pub fov: f64,
}

impl Camera {
    pub fn new(coord: Coord, angle: Angle3D, fov: f64) -> Self {
        Self { coord, angle, fov }
    }
}
