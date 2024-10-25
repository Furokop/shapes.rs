use super::*;
use crate::basetype::*;
use crate::shape::shape_base::Shape;
use crate::shape::shape_gen::ShapeGen;

#[derive(Clone)]
pub struct Object<'a> {
    pub location: Coord,
    pub shape: Shape<'a>,
    pub rotation: Angle3D,
    pub movement: Coord,
}

impl<'a> Object<'a> {
    pub fn new(location: Coord, shape_generator: &'a dyn ShapeGen, rotation: Angle3D, movement: Coord) -> Self {
        let shape: Shape<'a> = Shape::new(shape_generator);
        return Self {
            location,
            shape,
            rotation,
            movement,
        };
    }
    pub fn new_with_shape(location: Coord, shape: Shape<'a>, rotation: Angle3D, movement: Coord) -> Self {
        return Self {
            location,
            shape,
            rotation,
            movement,
        };
    }
    pub fn rotate_around(&self,  around: Coord, angles: Angle3D) -> Self {
        let loc_sub = self.location - around;
        let rotated_loc = loc_sub.to_vector().rotate(angles).as_coord();
        let new_loc = self.location + rotated_loc;
        Self::new_with_shape(new_loc, self.shape.rotate(angles), self.rotation, self.movement)
    }
    pub fn rotate(&self, angles: Angle3D) -> Self {
        Self::new_with_shape(self.location, self.shape.rotate(angles), self.rotation, self.movement)
    }
}