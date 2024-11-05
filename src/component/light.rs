use crate::basetype::*;
#[derive(Clone)]
pub struct Light3D {
    pub coord: Coord,
}

impl Light3D {
    pub fn new(coord: Coord) -> Self {
        Light3D { coord }
    }
}
