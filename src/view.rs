use crate::component::*;
use crate::out::terminal::SimpleTerminalBuffer;
use crate::out::Buffer;

pub struct Viewport<'a> {
    pub camera: Camera,
    pub lights: Vec<Light3D>,
    pub objects: Vec<Object<'a>>,
    pub buffer: SimpleTerminalBuffer,
    pub renderer: fn(view: &Viewport) -> SimpleTerminalBuffer,
}

impl<'a> Viewport<'a> {
    pub fn new(
        camera: Camera,
        buffer: SimpleTerminalBuffer,
        renderer: fn(view: &Viewport) -> SimpleTerminalBuffer,
    ) -> Self {
        Viewport {
            camera,
            lights: Vec::new(),
            objects: Vec::new(),
            buffer,
            renderer,
        }
    }

    pub fn render(&self) -> SimpleTerminalBuffer {
        (self.renderer)(self)
    }

    pub fn get_buffer_size(&self) -> (usize, usize) {
        (self.buffer.size_x, self.buffer.size_y)
    }

    pub fn add_object(&mut self, object: Object<'a>) {
        self.objects.push(object);
    }

    pub fn add_light(&mut self, light: Light3D) {
        self.lights.push(light);
    }

    pub fn print(&self) {
        self.buffer.print();
    }
}
