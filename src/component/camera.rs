use crate::basetype::*;
pub struct Camera {
    pub coord: Coord,
    pub facing: Vector3D,
    /// Horizontal fov
    pub fov: f64,
}

impl Camera {
    pub fn new(coord: Coord, facing: Vector3D, fov: f64) -> Self {
        let normalised_facing = facing.normalise();
        Self {
            coord,
            facing: normalised_facing,
            fov,
        }
    }

    /// For now, since roll is not implemented yet it will simply return the angle function of its
    /// facing field
    pub fn angle(&self) -> Angle3D {
        self.facing.angle()
    }
}
