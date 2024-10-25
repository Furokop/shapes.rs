use std::ops::{Index, IndexMut};

#[derive(Clone)]
pub struct SimpleTerminalBuffer {
    pub size_x: usize,
    pub size_y: usize,
    pub buffer: Vec<char>,
}

impl SimpleTerminalBuffer {
    pub fn new(size_x: usize, size_y: usize) -> Self {
        // Precreate vectors
        let buffer = vec![' '; (size_y * size_x) as usize];
        return Self {
            size_x,
            size_y,
            buffer,
        };
    }

    pub fn new_with_buffer(size_x: usize, size_y: usize, buffer: Vec<char>) -> Self {
        let mut ret = Self::new(size_x, size_y);
        ret.replace_buffer(buffer);
        return ret;
    }

    pub fn replace_buffer(&mut self, new_buffer: Vec<char>) {
        self.buffer = new_buffer;
    }

    pub fn replace_buffer_self(&mut self, new_buffer: Self) {
        self.replace_buffer(new_buffer.buffer);
    }

    pub fn get(&self, y: usize, x: usize) -> char {
        assert!(y < self.size_y);
        assert!(x < self.size_x);
        return self.buffer[y * self.size_x + x];
    }

    pub fn set(&mut self, y: usize, x: usize, val: char) {
        assert!(y < self.size_y);
        assert!(x < self.size_x);
        self.buffer[y * self.size_x + x] = val;
    }
}

impl Index<usize> for SimpleTerminalBuffer {
    type Output = char;
    fn index(&self, index: usize) -> &Self::Output {
        &self.buffer[index]
    }
}

impl IndexMut<usize> for SimpleTerminalBuffer {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.buffer[index]
    }
}
