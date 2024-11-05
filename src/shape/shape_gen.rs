/* File consisting of example functions for trivial shapes.
 *
 * A shape generator is a struct that implements the shapegen trait.
 * It is expected to define/use any given struct and initialise them with appropriate parameters,
 * After which, the function will be passed to a shape object from where it will act as a callback
 * for when the shape::generate() function is called.
 *
 * fn generate_whatever(max_x: i32, max_y: i32) -> bool;
 *
 * struct whatever {
 *   relevant data for object generation
 *   blah
 * }
 *
 * impl ShapeGen whatver {
 *   fn generate_shape(&self, shape: &mut Shape) {
 *       blah.blah = idk;
 *       use blah and blah to generate shape.
 *   }
 *
 *   fn helper_function() {
 *       yeah
 *   }
 *
 * }
 */

use crate::basetype::{Coord, Vector3D};
use crate::shape::shape_base::Point;
use std::f64::consts::PI;

use super::shape_base::Shape;

pub trait ShapeGen {
    fn generate_shape(&self, shape: &mut Shape);
}

/// Generates a torus using its fields when passed to an object.
/// ## Example:
/// ```
/// use shapes_rs::base::*;
/// use shapes_rs::buffer::*;
/// use shapes_rs::components::*;
/// use shapes_rs::generators::TorusGenerator;
/// use shapes_rs::renderer;
/// use shapes_rs::{Object, Viewport};
/// let torusgen = TorusGenerator::new(10.0, 50.0);
///
/// let my_torus_object = Object::new(Coord::new(70.0, 0.0, 0.0), &torusgen, Angle3D::default());
/// let my_light = Light3D::new(Coord::new(100.0, 0.0, 500.0));
/// let camera = Camera::new(
///     Coord::new(0.0, 0.0, 0.0),               
///     Vector3D::new(1.0, 0.0, 0.0).normalise(),
///     Angle::from_degree(60.0),
/// );
/// let output_buffer = SimpleTerminalBuffer::new(150, 50);
/// let mut viewport = Viewport::new(camera.clone(), output_buffer.clone(), renderer::pers_proj);
/// viewport.add_light(my_light);
/// viewport.add_object(my_torus_object);
/// let mut rot1 = 0.0;
/// let mut rot2 = 0.0;
/// loop {
///    rot1 += 0.1;
///    rot2 += 0.2;
///    viewport
///        .objects
///        .last_mut()
///        .expect("Lord save us should this occur")
///        .rotation = Angle3D::new(
///        Angle::default(),
///        Angle::from_radian(rot1),
///        Angle::from_radian(rot2),
///    );
///    let output = viewport.render();
///    print!("{}[2J", 27 as char);
///    output.print();
///    sleep(Duration::from_millis(50));
///    }
/// ```
pub struct TorusGenerator {
    pub thickness: f64,
    pub radius: f64,
    pub angle_iter: (f64, f64),
}

impl TorusGenerator {
    pub fn new(thickness: f64, size: f64) -> Self {
        TorusGenerator {
            thickness,
            radius: size,
            angle_iter: (0.04, 0.04),
        }
    }
}

impl ShapeGen for TorusGenerator {
    fn generate_shape(&self, shape: &mut Shape) {
        let mut a = 0.0;
        while a < 2.0 * PI {
            // Major circle
            let sin_a = f64::sin(a);
            let cos_a = f64::cos(a);
            let mut b = 0.0;
            while b < 2.0 * PI {
                // Minor circle
                let sin_b = f64::sin(b);
                let cos_b = f64::cos(b);

                let point_x: f64 = self.thickness * cos_b;
                let point_y: f64 = (self.radius + (self.thickness * sin_b)) * cos_a;
                let point_z: f64 = (self.radius + (self.thickness * sin_b)) * sin_a;
                let point_coord = Coord::new(point_x, point_y, point_z);

                let normal_x: f64 = cos_b;
                let normal_y: f64 = sin_b * cos_a;
                let normal_z: f64 = sin_b * sin_a;
                let normal = Vector3D::new(normal_x, normal_y, normal_z);

                let new_point: Point = Point::new(point_coord, normal);
                shape.points.push(new_point);
                b += self.angle_iter.1;
            }
            a += self.angle_iter.0;
        }
    }
}
