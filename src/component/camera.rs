use crate::basetype::*;
#[derive(Clone)]
pub struct Camera {
    pub coord: Coord,
    pub facing: Vector3D,
    /// Vertical fov
    pub fov: Angle,
}

impl Camera {
    /// NOTE: Fov is in radians!!! Also is vertical FOV.
    pub fn new(coord: Coord, facing: Vector3D, fov: Angle) -> Self {
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
