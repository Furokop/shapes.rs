use crate::basetype::Coord;

pub fn get_distance(point1: &Coord, point2: &Coord) -> f64 {
    let diff_x = point1.x - point2.x;
    let diff_y = point1.y - point2.y;
    let diff_z = point1.z - point2.z;
    f64::sqrt(diff_x.powf(2.0) + diff_y.powf(2.0) + diff_z.powf(2.0))
}
