use crate::{
    base::Vector3D,
    basetype::{Angle3D, Coord},
};

pub fn rotate_3d(rel_coord: Vector3D, angle: Angle3D) -> Vector3D {
    let n_vec = rel_coord;
    let n_vec1 = rotate3d_z(n_vec, angle.yaw.get());
    let n_vec2 = rotate3d_y(n_vec1, angle.pitch.get());
    rotate3d_x(n_vec2, angle.roll.get())
}

pub fn rotate3d_x(vector: Vector3D, angle: f64) -> Vector3D {
    let (x, y, z) = vector.get();

    let new_y = y * f64::cos(angle) - z * f64::sin(angle);
    let new_z = y * f64::sin(angle) + z * f64::cos(angle);

    Vector3D::new(x, new_y, new_z)
}

pub fn rotate3d_y(vector: Vector3D, angle: f64) -> Vector3D {
    let (x, y, z) = vector.get();

    let new_x = x * f64::cos(angle) + z * f64::sin(angle);
    let new_z = -x * f64::sin(angle) + z * f64::cos(angle);

    Vector3D::new(new_x, y, new_z)
}

pub fn rotate3d_z(vector: Vector3D, angle: f64) -> Vector3D {
    let (x, y, z) = vector.get();

    let new_x = x * f64::cos(angle) - y * f64::sin(angle);
    let new_y = x * f64::sin(angle) + y * f64::cos(angle);

    Vector3D::new(new_x, new_y, z)
}

pub fn get_distance(point1: &Coord, point2: &Coord) -> f64 {
    let diff_x = point1.x - point2.x;
    let diff_y = point1.y - point2.y;
    let diff_z = point1.z - point2.z;
    f64::sqrt(diff_x.powf(2.0) + diff_y.powf(2.0) + diff_z.powf(2.0))
}
