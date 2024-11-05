pub mod terminal;

pub trait Buffer {
    type Data;
    type Container;

    fn get(&self, y: usize, x: usize) -> Self::Data;
    fn set(&mut self, y: usize, x: usize, val: Self::Data);
    fn print(&self);
    fn replace_buffer(&mut self, new_buffer: Self::Container);
    fn replace_buffer_self(&mut self, new_buffer: Self);
    fn new_with_buffer(size_x: usize, size_y: usize, buffer: Self::Container) -> Self;
}
