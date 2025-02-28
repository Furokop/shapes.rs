use crate::{math::trig::get_distance, shape::rotator::Rotator};
use core::panic;
use std::{
    f64::consts::PI,
    ops::{Add, Div, Mul, Sub}
};

/// Basic type which represents a given location in cartesian coordinates
/// The origin values do not represent a global origin and might instead be
/// a relative position from one point to another, depending on the context
/// in which it is used
///
/// In this notation, Z is supposed to represent up, looking at X positive Y
/// extends right side
#[derive(Copy, Clone)]
pub struct Coord {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Coord {
    /// Dissolves the struct into a tuple
    /// ### Example:
    /// ```
    /// use shapes_rs::base::Coord;
    /// let my_coord = Coord::new(5.0, 10.0, 0.0);
    /// let (x, y, z) = my_coord.get();
    /// assert_eq!(x, 5.0);
    /// ```
    pub fn get(&self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }

    /// Constructor function, self explanatory, imo
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Default function to construct an object using zeros
    pub fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// Returns an equal Vector3D from a given Coord for algorithmic purposes
    /// ### Example:
    /// ```
    /// use shapes_rs::base::Coord;
    /// use shapes_rs::base::Vector3D;
    /// use shapes_rs::base::Angle3D;
    /// use shapes_rs::base::Angle;
    /// use shapes_rs::base::Rotator;
    ///
    /// let my_coord = Coord::new(3.0, 0.0, 0.0);
    /// let rotation_angle = Angle3D::new(Angle::default(), Angle::from_degree(90.0), Angle::default());
    /// let rotator = Rotator::from_global(Angle3D::new(Angle::default(), Angle::from_radian(2.0), Angle::default()));
    /// let my_coord_rotated = rotator.apply(my_coord.to_vector()).as_coord();
    ///
    /// assert!(my_coord_rotated.x  < 0.00001);
    /// ```
    pub fn to_vector(&self) -> Vector3D {
        Vector3D::new(self.x, self.y, self.z)
    }

    /// Multiplies all coordinates with a given scalar
    /// ### Example:
    /// ```
    /// use shapes_rs::base::Coord;
    /// let my_coord = Coord::new(5.0, 2.0, 6.0);
    /// let doubled_coord = my_coord.mul(2.0);
    ///
    /// assert_eq!(doubled_coord.x, 10.0);
    /// assert_eq!(doubled_coord.y, 4.0);
    /// assert_eq!(doubled_coord.z, 12.0);
    /// ```
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

/// Struct that defines an angle in Euler angles
/// ### Example:
/// ```
/// use shapes_rs::base::{Angle3D, Angle};
/// use std::f64::consts::PI;
///
/// let angle = Angle3D::new(Angle::from_degree(180.0), Angle::from_radian(PI), Angle::default());
///
/// // PI in radians equals 180 degrees
/// assert_eq!(angle.roll.get(), angle.pitch.get());
/// ```
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
    /// Dissolves the struct into a tuple
    /// ### Example:
    /// ```
    /// use shapes_rs::base::{Angle3D, Angle};
    /// use std::f64::consts::PI;
    ///
    /// let my_angle = Angle3D::new(Angle::default(), Angle::from_degree(180.0), Angle::from_radian(1.0));
    /// let (roll, pitch, yaw) = my_angle.get();
    /// assert_eq!(roll.get(), 0.0);
    /// assert_eq!(pitch.get(), PI);
    /// assert_eq!(yaw.get(), 1.0);
    /// ```
    pub fn get(&self) -> (Angle, Angle, Angle) {
        (self.roll, self.pitch, self.yaw)
    }

    /// Constructor function
    pub fn new(roll: Angle, pitch: Angle, yaw: Angle) -> Self {
        Self { yaw, pitch, roll }
    }

    /// Multiplies the values of the struct with a given scalar.
    ///
    pub fn mul(&self, mul: f64) -> Self {
        let as_angle = Angle::from_radian(mul);
        Self {
            roll: self.roll * as_angle,
            pitch: self.pitch * as_angle,
            yaw: self.yaw * as_angle,
        }
    }

    /// Returns this struct with all values set to zero, the origin
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
    /// Dissolves the vector and returns its values as a tuple of three f64
    /// ### Example:
    /// ```
    /// use shapes_rs::base::Vector3D;
    /// let my_vec = Vector3D::new(3.0, 2.0, 1.0);
    /// let (x, y, z) = my_vec.get();
    ///
    /// assert_eq!(x, 3.0);
    /// assert_eq!(y, 2.0);
    /// assert_eq!(z, 1.0);
    /// ```
    pub fn get(&self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }

    /// Creates a new Vector from given parameters
    /// ### Example:
    /// ```
    /// use shapes_rs::base::Vector3D;
    /// let my_vec = Vector3D::new(3.0, 2.0, 1.0);
    ///
    /// assert_eq!(my_vec.x, 3.0);
    /// assert_eq!(my_vec.y, 2.0);
    /// assert_eq!(my_vec.z, 1.0);
    /// ```
    ///
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        if x == 0.0 && y == 0.0 && z == 0.0 {
            panic!("Uhh, idk if this should be a panic but you cannot initiate a vector with all zeros")
        }
        Self { x, y, z }
    }

    /// Returns a default vector, pointing towards X as the default direction.
    /// A vector needs to point to something.
    pub fn default() -> Self {
        Self {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// Rotates a vector
    /// ### Example:
    /// ```
    /// use shapes_rs::base::{Vector3D, Angle3D, Angle, Rotator};
    ///
    /// let my_vec = Vector3D::new(3.0, 0.0, 0.0);
    /// let my_angle = Rotator::from_global(Angle3D::new(Angle::default(), Angle::from_degree(90.0), Angle::default()));
    ///
    /// let my_rotated_vec = my_vec.rotate(&my_angle);
    /// assert_eq!(my_rotated_vec.z as i32, 3);
    /// ```
    pub fn rotate(&self, rotator: &Rotator) -> Self {
        rotator.apply(*self)
    }

    pub fn mul(&self, mul: f64) -> Self {
        Self {
            x: self.x * mul,
            y: self.y * mul,
            z: self.z * mul,
        }
    }

    /// Returns the magnitude scalar of the vector
    /// ### Example:
    /// ```
    /// use shapes_rs::base::Vector3D;
    ///
    /// let my_vec = Vector3D::new(2.0,3.0,6.0);
    /// assert_eq!(my_vec.magnitude(), 7.0);
    /// ````
    pub fn magnitude(&self) -> f64 {
        let self_as_coord = self.as_coord();
        get_distance(&Coord::default(), &self_as_coord)
    }

    /// Retunrs the dot product of this vector with another.
    /// ### Example:
    /// ```
    /// use shapes_rs::base::Vector3D;
    ///
    /// let my_vec1 = Vector3D::new(3.0,0.0,0.0);
    /// let my_vec2 = Vector3D::new(0.0,3.0,0.0);
    ///
    /// // Perpendicular vectors have their dot product zero
    /// assert_eq!(my_vec1.dot(my_vec2),0.0);
    ///
    /// let my_vec3 = Vector3D::new(3.0,0.0,0.0);
    /// let my_vec4 = Vector3D::new(3.0,0.0,0.0);
    ///
    /// // Parallel vectors have their dot product multiplied by their components
    /// assert_eq!(my_vec3.dot(my_vec4), 3.0 * 3.0);
    /// ```
    pub fn dot(&self, other: Self) -> f64 {
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

    pub fn cross(&self, other: Self) -> Self {
        Self {
            x: self.y * other.z,
            y: self.z * other.x,
            z: self.x * other.y
        }
    }

    /// Converts the Vector into coordinates, used for algorithmic purposes
    pub fn as_coord(&self) -> Coord {
        Coord {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    /// Normalizes a vector
    /// ### Example:
    /// ```
    /// use shapes_rs::base::Vector3D;
    ///
    /// let my_vec = Vector3D::new(3.0,2.0,1.0);
    /// let my_normalised_vec = my_vec.normalise();
    ///
    /// assert_eq!(my_normalised_vec.magnitude(), 1.0);
    /// assert!(my_vec.magnitude() > my_normalised_vec.magnitude());
    /// ```
    pub fn normalise(&self) -> Self {
        let mag = self.magnitude();
        Self {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }

    /// Returns the angle between two vectors
    /// ### Example:
    /// ```
    /// use shapes_rs::base::Vector3D;
    /// use std::f64::consts::PI;
    ///
    /// let my_vec1 = Vector3D::new(3.0, 0.0, 0.0);
    /// let my_vec2 = Vector3D::new(0.0, 0.0, 3.0);
    ///
    /// let my_angle = my_vec1.angle_between(my_vec2);
    ///
    /// // Equals 90 degrees
    /// assert_eq!(my_angle, PI / 2.0);
    /// ```
    pub fn angle_between(&self, other: Self) -> f64 {
        let mag_self = self.magnitude();
        let mag_other = other.magnitude();
        let dot = self.dot(other);

        f64::acos(dot / (mag_self * mag_other))
    }

    /// Converts the vector into spherical coordinates and returns the angles of said coordinate
    /// ### Example:
    /// ```
    /// use shapes_rs::base::Vector3D;
    /// use std::f64::consts::PI;
    ///
    /// let x_vector = Vector3D::new(3.0, 0.0, 0.0);
    /// let x_vector2 = Vector3D::new(9.0, 0.0, 0.0);
    ///
    /// assert_eq!(x_vector.angle().yaw.get(), 0.0);
    /// assert_eq!(x_vector2.angle().pitch.get(), 0.0);
    ///
    /// let y_vector = Vector3D::new(0.0, 3.0, 0.0);
    /// let z_vector = Vector3D::new(0.0, 0.0, 3.0);
    ///
    /// assert_eq!(y_vector.angle().yaw.get(), PI / 2.0);
    /// assert_eq!(z_vector.angle().pitch.get(), PI / 2.0);
    /// ```
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

impl Mul<f64> for Vector3D {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}

impl Mul<Vector3D> for f64 {
    type Output = Vector3D;
    fn mul(self, rhs: Vector3D) -> Self::Output {
        Vector3D {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self
        }
    }
}

impl Add for Vector3D {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

/// Angle class that represents, well, an angle
/// The angle is stored as a radian
#[derive(Clone, Copy)]
pub struct Angle {
    /// In radians
    angle: f64,
}

impl Angle {
    /// Creates a new angle from a given float as radian
    pub fn from_radian(angle: f64) -> Self {
        Self { angle }
    }

    /// Creates a new angle from a given float as degrees
    pub fn from_degree(angle: f64) -> Self {
        Self {
            angle: angle / (360.0 / (2.0 * PI)),
        }
    }

    /// Converts the angle to float as radian
    /// Does the same thing as get_radian
    pub fn get(&self) -> f64 {
        self.angle
    }

    /// Converts the angle to float as radian
    /// Does the same thing as get
    pub fn get_radian(&self) -> f64 {
        self.angle
    }

    /// Converts the angle to float as degrees
    pub fn get_degrees(&self) -> f64 {
        use std::f64::consts::PI;
        self.angle * PI
    }

    /// Constructs an angle with the value at zero
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
