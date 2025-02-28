use crate::component::*;
use crate::out::terminal::SimpleTerminalBuffer;
use crate::out::Buffer;

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
/// let my_torus_object = Object::new(Coord::new(70.0, 0.0, 0.0), &torusgen, Rotator::default());
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
pub struct Scene<'a> {
    pub camera: Camera,
    pub lights: Vec<Light3D>,
    pub objects: Vec<Object<'a>>,
    pub buffer: SimpleTerminalBuffer,
    pub renderer: fn(view: &Scene) -> SimpleTerminalBuffer,
}

impl<'a> Scene<'a> {
    /// Constructor function for a given scene.
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
    /// let my_torus_object = Object::new(Coord::new(70.0, 0.0, 0.0), &torusgen, Rotator::default());
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
    /// ```
    pub fn new(
        camera: Camera,
        buffer: SimpleTerminalBuffer,
        renderer: fn(view: &Scene) -> SimpleTerminalBuffer,
    ) -> Self {
        Scene {
            camera,
            lights: Vec::new(),
            objects: Vec::new(),
            buffer,
            renderer,
        }
    }

    /// Calls the renderer function given, passes itself to it as an argument
    pub fn render(&self) -> SimpleTerminalBuffer {
        (self.renderer)(self)
    }

    /// Returns the size of the bound buffer
    pub fn get_buffer_size(&self) -> (usize, usize) {
        (self.buffer.size_x, self.buffer.size_y)
    }

    /// Append an object
    pub fn add_object(&mut self, object: Object<'a>) {
        self.objects.push(object);
    }

    /// Add a light source
    pub fn add_light(&mut self, light: Light3D) {
        self.lights.push(light);
    }

    /// Calls the bound buffer's print function. It will do what the buffer is preconfigured to do.
    pub fn print(&self) {
        self.buffer.print();
    }
}
