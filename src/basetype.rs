use std::ops::{Sub, Add};

/// Z goes up, Y goes sideways
use crate::math::trig::{self, get_distance};
#[derive(Copy, Clone)]
pub struct Coord {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Coord {
    /// Dissolve struct into a tuple
    pub fn get(&self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn to_vector(&self) -> Vector3D {
        Vector3D::new(self.x, self.y, self.z)
    }

    pub fn mul(&self, mul: f64) -> Self {
        Self {
            x: self.x * mul,
            y: self.y * mul,
            z: self.z * mul,
        }
    }
}

impl Sub for Coord {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Angle3D {
    /// Roll
    pub x_angle: f64,
    /// Pitch
    pub y_angle: f64,
    /// Yaw
    pub z_angle: f64,
}

impl Angle3D {
    pub fn get(&self) -> (f64, f64, f64) {
        (self.x_angle, self.y_angle, self.z_angle)
    }

    pub fn new(x_angle: f64, y_angle: f64, z_angle: f64) -> Self {
        Self {
            z_angle,
            y_angle,
            x_angle,
        }
    }

    pub fn mul(&self, mul: f64) -> Self {
        Self {
            x_angle: self.x_angle * mul,
            y_angle: self.y_angle * mul,
            z_angle: self.z_angle * mul,
        }
    }

    pub fn default() -> Self {
        Self {
            x_angle: 0.0,
            y_angle: 0.0,
            z_angle: 0.0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3D {
    pub fn get(&self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn rotate(&self, angle: Angle3D) -> Self {
        let new_vec = trig::rotate_3d(Coord::new(self.x, self.y, self.z), angle).to_vector();
        return Vector3D::new(new_vec.x, new_vec.y, new_vec.z);
    }

    pub fn mul(&self, mul: f64) -> Self {
        Self {
            x: self.x * mul,
            y: self.y * mul,
            z: self.z * mul,
        }
    }

    pub fn magnitude(&self) -> f64 {
        let self_as_coord = self.as_coord();
        return get_distance(&Coord::default(), &self_as_coord);
    }

    pub fn dot(&self, other: &Self) -> f64 {
        return self.x * other.x + self.y * other.y + self.z + other.z;
    }

    pub fn as_coord(&self) -> Coord {
        Coord {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}
