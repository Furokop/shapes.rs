use core::panic;
use std::{
    f64::consts::PI,
    ops::{Add, Div, Mul, Sub},
};

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
    pub roll: Angle,
    /// Pitch
    pub pitch: Angle,
    /// Yaw
    pub yaw: Angle,
}

impl Angle3D {
    pub fn get(&self) -> (Angle, Angle, Angle) {
        (self.roll, self.pitch, self.yaw)
    }

    pub fn new(roll: Angle, pitch: Angle, yaw: Angle) -> Self {
        Self { yaw, pitch, roll }
    }

    pub fn mul(&self, mul: f64) -> Self {
        let as_angle = Angle::from_radian(mul);
        Self {
            roll: self.roll * as_angle,
            pitch: self.pitch * as_angle,
            yaw: self.yaw * as_angle,
        }
    }

    pub fn default() -> Self {
        Self {
            roll: Angle::from_radian(0.0),
            pitch: Angle::from_radian(0.0),
            yaw: Angle::from_radian(0.0),
        }
    }
}

impl Add for Angle3D {
    type Output = Angle3D;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.roll + rhs.roll,
            self.pitch + rhs.pitch,
            self.yaw + rhs.yaw,
        )
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
        if x == 0.0 && y == 0.0 && z == 0.0 {
            panic!("Uhh, idk if this should be a panic but you cannot initiate a vector with all zeros")
        }
        Self { x, y, z }
    }

    pub fn default() -> Self {
        Self {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn rotate(&self, angle: Angle3D) -> Self {
        let new_vec = trig::rotate_3d(*self, angle);
        Vector3D::new(new_vec.x, new_vec.y, new_vec.z)
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
        get_distance(&Coord::default(), &self_as_coord)
    }

    pub fn dot(&self, other: &Self) -> f64 {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        if self.x != 0.0 && other.x != 0.0 {
            x = self.x * other.x;
        }
        if self.y != 0.0 && other.y != 0.0 {
            y = self.y * other.y;
        }
        if self.z != 0.0 && other.z != 0.0 {
            z = self.z * other.z;
        }
        x + y + z
    }

    pub fn as_coord(&self) -> Coord {
        Coord {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    pub fn normalise(&self) -> Self {
        let mag = self.magnitude();
        Self {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }

    pub fn angle_between(&self, other: Self) -> f64 {
        let mag_self = self.magnitude();
        let mag_other = other.magnitude();
        let dot = self.dot(&other);

        f64::acos(dot / (mag_self * mag_other))
    }

    pub fn angle(&self) -> Angle3D {
        let yaw = Angle::from_radian(self.y.atan2(self.x));
        let pitch = Angle::from_radian(self.z.atan2((self.x.powi(2) + self.y.powi(2)).sqrt()));
        Angle3D {
            roll: Angle::from_radian(0.0),
            pitch,
            yaw,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Angle {
    /// In radians
    angle: f64,
}

impl Angle {
    pub fn from_radian(angle: f64) -> Self {
        Self { angle }
    }

    pub fn from_degree(angle: f64) -> Self {
        Self {
            angle: angle / (360.0 / (2.0 * PI)),
        }
    }

    pub fn get(&self) -> f64 {
        self.angle
    }

    pub fn default() -> Self {
        Self { angle: 0.0 }
    }
}

impl Mul for Angle {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            angle: self.angle * rhs.angle,
        }
    }
}

impl Div for Angle {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            angle: self.angle / rhs.angle,
        }
    }
}

impl Add for Angle {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            angle: self.angle + rhs.angle,
        }
    }
}

impl Sub for Angle {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            angle: self.angle - rhs.angle,
        }
    }
}
