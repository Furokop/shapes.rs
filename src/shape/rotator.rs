/*  Rotator file with utility functions
 *
 *  Ideas: Rotator struct with timers
 *  Ideas: On-demand rotation
 *  Ideas: Above but with pregenerated
 *  Ideas: Buffer results
 */
use crate::basetype::Angle3D;
use crate::basetype::Vector3D;
use std::ops::{Add, Sub};
use std::f64;

#[derive(Clone)]
pub struct Rotator {
    pub x : f64,
    pub y : f64,
    pub z : f64,
    pub w : f64,
}

impl Rotator {
    pub fn new(axis: Vector3D, angle: f64) -> Self {        
        let norm = f64::sqrt(axis.x * axis.x + axis.y * axis.y + axis.z * axis.z);
        if norm == 0.0 {
            panic!("Zero-length axis provided");
        }
        let half_angle = angle / 2.0;
        let s = f64::sin(half_angle);
        let c = f64::cos(half_angle);
        Rotator {
            x: (axis.x / norm) * s,
            y: (axis.y / norm) * s,
            z: (axis.z / norm) * s,
            w: c,
        }
    }
    pub fn conjugate(&self) -> Self {
        Rotator {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: self.w,
        }
    }
    pub fn from_global(angle: Angle3D) -> Self {
        let cr = f64::cos(angle.roll.get() * 0.5);
        let sr = f64::sin(angle.roll.get() * 0.5);
        let cp = f64::cos(angle.pitch.get() * 0.5);
        let sp = f64::sin(angle.pitch.get() * 0.5);
        let cy = f64::cos(angle.yaw.get() * 0.5);
        let sy = f64::sin(angle.yaw.get() * 0.5);
        Rotator {
            x: sr * cp * cy - cr * sp * sy,  // x
            y: cr * sp * cy + sr * cp * sy,  // y
            z: cr * cp * sy - sr * sp * cy,  // z
            w: cr * cp * cy + sr * sp * sy,  // w (scalar)
        }
    }
    pub fn normalize(&self) -> Self {
        let mag = f64::sqrt(self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z);
        if mag == 0.0 {
            panic!("Attempted to normalize a zero-magnitude quaternion");
        }
        Rotator {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
            w: self.w / mag,
        }
    }
    pub fn apply(&self, point: Vector3D) -> Vector3D {
        // Ensure the quaternion is normalized.
        let q_norm = self.normalize();
        // Represent the point as a pure quaternion (zero scalar part).
        let p = Rotator {
            x: point.x,
            y: point.y,
            z: point.z,
            w: 0.0,
        };
        // Rotate: p' = q * p * q_conjugate
        let q_conj = q_norm.conjugate();
        let res = q_norm.multiply(&p).multiply(&q_conj);
        Vector3D::new(res.x, res.y, res.z)
    }
    pub fn multiply(&self, other: &Rotator) -> Self {
        Rotator {
            w: self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z,
            x: self.w * other.x + self.x * other.w + self.y * other.z - self.z * other.y,
            y: self.w * other.y - self.x * other.z + self.y * other.w + self.z * other.x,
            z: self.w * other.z + self.x * other.y - self.y * other.x + self.z * other.w,
        }
    }
}

impl Default for Rotator {
    fn default() -> Self {
        Self {
            x: 1.0,
            y: 0.0,
            z: 0.0,
            w: 0.0
        }
    }
}

impl Add for Rotator {
    type Output = Rotator;
    fn add(self, rhs: Self) -> Self::Output {
        Rotator {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w
        }
    }
}

impl Sub for Rotator {
    type Output = Rotator;
    fn sub(self, rhs: Self) -> Self::Output {
        Rotator {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w
        }
    }
}