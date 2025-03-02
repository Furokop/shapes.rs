# Shapes-rs

Odd project I made about projecting dots to a camera with support for rotation and translation

### Example usage
```
use shapes_rs::base::*;
use shapes_rs::buffer::*;
use shapes_rs::components::*;
use shapes_rs::generators::TorusGenerator;
use shapes_rs::renderer;
use shapes_rs::{Object, Scene};

use std::thread::sleep;
use std::time::Duration;

fn main() {
    // Define a shape generator
    let torusgen = TorusGenerator::new(10.0, 50.0);
    // Construct an object using the shape generator
    let my_torus_object = Object::new(Coord::new(70.0, 0.0, 0.0), &torusgen, Rotator::from_global(Angle3D::new(Angle::from_degree(5.0), Angle::from_degree(5.0), Angle::from_degree(8.0))));
    // Light and camera
    let my_light = Light3D::new(Coord::new(100.0, 0.0, 500.0));
    let camera = Camera::new(
        Coord::new(0.0, 0.0, 0.0),               
        Vector3D::new(1.0, 0.0, 0.0).normalise(),
        Angle::from_degree(120.0),
    );
    // Create any buffer using its own constructor
    let output_buffer = SimpleTerminalBuffer::new(150, 50);
    // Assemble the scene
    let mut scene = Scene::new(camera.clone(), output_buffer.clone(), renderer::pers_proj);
    // Add light source
    scene.add_light(my_light);
    // Add object
    scene.add_object(my_torus_object);

    loop {
        for obj in scene.objects.iter_mut() {
            obj.apply_rotation();
        }
        // Get the scene to call the render function. Can also do this manually
        let output = scene.render();
        // Print the buffer
        print!("{}[2J", 27 as char);
        output.print();
        sleep(Duration::from_millis(50));
    }
}
```
