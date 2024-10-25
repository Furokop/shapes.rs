use crate::basetype::{Angle3D, Coord, Vector3D};
use crate::math::trig;
use crate::shape::shape_gen::ShapeGen;

// Should also have max_x, max_y??
//
// Possible alternate shape that uses faces instead of points acting as voxels???????
#[derive(Clone)]
pub struct Shape<'a> {
    pub points: Vec<Point>,
    pub shape_generator: &'a dyn ShapeGen,
    generated: bool,
}

impl<'a> Shape<'a> {
    /// Default value for function argument is a function that always returns a null
    /// The points_vec is just an empty vector unless given an argument (probably not needed?)
    pub fn new(
        shape_generator: &'a dyn ShapeGen,
    ) -> Self {
        let points: Vec<Point> = Vec::new();

        let mut ret = Self {
            points,
            shape_generator,
            generated: false,
        };
        ret.generate();
        return ret;
    }

    /// Rotate the shape around itself
    pub fn rotate(&self, angles: Angle3D) -> Self {
        let mut new_shape = Shape {
            points: Vec::new(),
            shape_generator: self.shape_generator,
            generated: true,
        };
        for point in self.points.iter() {
            let rotated_point = point.rotate(angles);
            new_shape.points.push(rotated_point);
        }
        return new_shape;
    }

    fn generate_if_not(&mut self) {
        if !self.generated {
            self.shape_generator.generate_shape(self);
            self.generated = true;
        }
    }

    pub fn generate(&mut self) {
        self.generate_if_not();
    }

    pub fn generated(&self) -> bool {
        return self.generated;
    }
}

#[derive(Copy, Clone)]
pub struct Point {
    pub rel_coord: Coord,
    pub normal: Vector3D,
}

impl Point {
    pub fn new(rel_coord: Coord, normal: Vector3D) -> Self {
        return Self { rel_coord, normal };
    }
    pub fn rotate(&self, angles: Angle3D) -> Self {
        let (x, y, z) = self.rel_coord.get();
        let normal = self.normal;

        return Point::new(
            trig::rotate_3d(Coord::new(x, y, z), angles),
            normal.rotate(angles),
        );
    }
}

fn placeholder(_nothing1: i32, _meaningless: i32) -> bool {
    return false;
}
