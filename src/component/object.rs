use crate::basetype::*;
use crate::shape::shape_base::Shape;
use crate::shape::shape_gen::ShapeGen;
use crate::shape::rotator::Rotator;

#[derive(Clone)]
pub struct Object<'a> {
    pub location: Coord,
    pub shape: Shape<'a>,
    pub rotation: Rotator,
}

impl<'a> Object<'a> {
    pub fn new(location: Coord, shape_generator: &'a dyn ShapeGen, rotation: Rotator) -> Self {
        let shape: Shape<'a> = Shape::new(shape_generator);
        Self {
            location,
            shape,
            rotation,
        }
    }
    pub fn new_with_shape(location: Coord, shape: Shape<'a>, rotation: Rotator) -> Self {
        Self {
            location,
            shape,
            rotation,
        }
    }
    pub fn new_from_rotate_around(&self, around: Coord, rotator: &Rotator) -> Self {
        let loc_sub = self.location - around;
        let rotated_loc = loc_sub.to_vector().rotate(rotator).as_coord();
        let new_loc = self.location + rotated_loc;
        Self::new_with_shape(new_loc, self.shape.rotate(rotator), self.rotation.clone())
    }
    pub fn new_from_rotated(&self, rotator: &Rotator) -> Self {
        Self::new_with_shape(
            self.location,
            self.shape.rotate(rotator),
            self.rotation.clone() + rotator.clone(),
        )
    }
}
