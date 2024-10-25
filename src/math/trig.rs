use crate::basetype::{Angle3D, Coord};

pub fn rotate_3d(rel_coord: Coord, angle: Angle3D) -> Coord {
    let (x, y, z) = rel_coord.get();
    // Return values

    // yaw = around z
    let cos_yaw = f64::cos(angle.z_angle);
    let sin_yaw = f64::sin(angle.z_angle);

    // pitch = around y
    let cos_pitch = f64::cos(angle.y_angle);
    let sin_pitch = f64::sin(angle.y_angle);

    // roll = around x
    let cos_roll = f64::cos(angle.x_angle);
    let sin_roll = f64::sin(angle.x_angle);

    let new_x: f64 = x * cos_yaw * cos_pitch
        + y * (cos_yaw * sin_pitch * sin_roll - sin_yaw * cos_roll)
        + z * (cos_yaw * sin_pitch * cos_roll + sin_yaw * sin_roll);

    let new_y: f64 = x * sin_yaw * cos_pitch
        + y * (sin_yaw * sin_pitch * sin_roll + cos_yaw * cos_roll)
        + z * (sin_yaw * sin_pitch * cos_roll - cos_yaw * sin_roll);

    let new_z: f64 = -x * sin_pitch + y * cos_pitch * sin_roll + z * cos_pitch * cos_roll;

    return Coord::new(new_x, new_y, new_z);
}

pub fn get_distance(point1: &Coord, point2: &Coord) -> f64 {
    let diff_x = point1.x - point2.x;
    let diff_y = point1.y - point2.y;
    let diff_z = point1.z - point2.z;
    return f64::sqrt(diff_x.powf(2.0) + diff_y.powf(2.0) + diff_z.powf(2.0));
}
