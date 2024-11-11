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

/// Basic binding class that constitutes a scene
/// Four things must be available (thus constructed if needed) beforehand in order to create a usable scene:
/// - Any type which implements buffer
/// - Camera class
/// - An object class from which image will be generated
/// - A light source (currently very simple, merely denotes the shading of the object)
/// - Renderer function which will use all of the above and return a new buffer
/// ### Example:
/// ```
/// use shapes_rs::base::*;
/// use shapes_rs::buffer::*;
/// use shapes_rs::components::*;
/// use shapes_rs::generators::TorusGenerator;
/// use shapes_rs::renderer;
/// use shapes_rs::{Object, Scene};
/// // Define a shape generator
/// let torusgen = TorusGenerator::new(10.0, 50.0);
/// // Construct an object using the shape generator
/// let my_torus_object = Object::new(Coord::new(70.0, 0.0, 0.0), &torusgen, Angle3D::default());
///
/// // Light and camera
/// let my_light = Light3D::new(Coord::new(100.0, 0.0, 500.0));
/// let camera = Camera::new(
///     Coord::new(0.0, 0.0, 0.0),               
///     Vector3D::new(1.0, 0.0, 0.0).normalise(),
///     Angle::from_degree(60.0),
/// );
///
/// // Create any buffer using its own constructor
/// let output_buffer = SimpleTerminalBuffer::new(150, 50);
///
/// // Assemble the scene
/// let mut scene = Scene::new(camera.clone(), output_buffer.clone(), renderer::pers_proj);
///
/// // Add light source
/// scene.add_light(my_light);
///
/// // Add object
/// scene.add_object(my_torus_object);
///
/// // Get the scene to call the render function. Can also do this manually
/// let output = scene.render();
///
/// // Print the buffer
/// output.print();
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

pub struct CubeGenerator {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub node_dis: f64,
}

impl CubeGenerator {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x,
            y,
            z,
            node_dis: 1.0,
        }
    }
}

impl ShapeGen for CubeGenerator {
    fn generate_shape(&self, shape: &mut Shape) {
        let edge_z = self.z / 2.0;
        let edge_y = self.y / 2.0;
        let edge_x = self.x / 2.0;
        let node_dis = self.node_dis;
        // Generate +X facing
        for z in 0..(self.z / self.node_dis) as usize - 1 {
            for y in 0..(self.y / self.node_dis) as usize - 1 {
                let normal = Vector3D::new(1.0, 0.0, 0.0);
                let coord = Coord::new(
                    edge_x,
                    edge_y - y as f64 * node_dis,
                    edge_z - z as f64 * node_dis,
                );
                shape.points.push(Point::new(coord, normal));
            }
        }
        // Generate -X facing
        for z in 0..(self.z / self.node_dis) as usize - 1 {
            for y in 0..(self.y / self.node_dis) as usize - 1 {
                let normal = Vector3D::new(-1.0, 0.0, 0.0);
                let coord = Coord::new(
                    -edge_x,
                    edge_y - y as f64 * node_dis,
                    edge_z - z as f64 * node_dis,
                );
                shape.points.push(Point::new(coord, normal));
            }
        }
        // Generate +Y facing
        for z in 0..(self.z / self.node_dis) as usize - 1 {
            for x in 0..(self.x / self.node_dis) as usize - 1 {
                let normal = Vector3D::new(0.0, 1.0, 0.0);
                let coord = Coord::new(
                    edge_x - x as f64 * node_dis,
                    edge_y,
                    edge_z - z as f64 * node_dis,
                );
                shape.points.push(Point::new(coord, normal));
            }
        }
        // Generate -Y facing
        for z in 0..(self.z / self.node_dis) as usize - 1 {
            for x in 0..(self.x / self.node_dis) as usize - 1 {
                let normal = Vector3D::new(0.0, -1.0, 0.0);
                let coord = Coord::new(
                    edge_x - x as f64 * node_dis,
                    -edge_y,
                    edge_z - z as f64 * node_dis,
                );
                shape.points.push(Point::new(coord, normal));
            }
        }
        // Generate +Z facing
        for y in 0..(self.y / self.node_dis) as usize - 1 {
            for x in 0..(self.x / self.node_dis) as usize - 1 {
                let normal = Vector3D::new(0.0, 0.0, 1.0);
                let coord = Coord::new(
                    edge_x - x as f64 * node_dis,
                    edge_y - y as f64 * node_dis,
                    edge_z,
                );
                shape.points.push(Point::new(coord, normal));
            }
        }
        // Generate -Z facing
        for y in 0..(self.y / self.node_dis) as usize - 1 {
            for x in 0..(self.x / self.node_dis) as usize - 1 {
                let normal = Vector3D::new(0.0, 0.0, -1.0);
                let coord = Coord::new(
                    edge_x - x as f64 * node_dis,
                    edge_y - y as f64 * node_dis,
                    -edge_z,
                );
                shape.points.push(Point::new(coord, normal));
            }
        }
    }
}
