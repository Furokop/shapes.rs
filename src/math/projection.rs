use crate::out::terminal::SimpleTerminalBuffer;
use crate::scene::Scene;

use core::f64;
use std::f64::consts::PI;

/// Perspective renderer implementation
pub fn pers_proj(view: &Scene) -> SimpleTerminalBuffer {
    let luminance_str = ".,-~:;=!*#$@@@";
    let luminance: &[u8] = luminance_str.as_bytes();
    let lumi_length = luminance_str.len();

    let (size_x, size_y) = view.get_buffer_size();

    let view_coord = view.camera.coord;
    let v_a = view.camera.angle();

    let mut z_buffer = vec![f64::MAX; size_y * size_x];

    let mut projected_buffer = SimpleTerminalBuffer::new(size_x, size_y);

    let pb_dis = 1.0 / f64::tan(view.camera.fov.get() / 2.0) * ((size_y as f64) / 2.0);

    for obj in &view.objects {
        let object_rotation = obj.rotation;
        let object_coord = obj.location;
        for point in &obj.shape.points {
            let point_coord = point
                .rel_coord
                .to_vector()
                .rotate(object_rotation)
                .as_coord()
                + object_coord;

            // Distance between point and camera in vector
            let pv = (point_coord - view_coord).to_vector();

            // Distance between point and camera in scalar
            let pv_dis = pv.magnitude();

            // Camera transform by rotating pv with negative angle of camera
            let cpv = pv.rotate(v_a.mul(-1.0));
            let (cpv_x, cpv_y, cpv_z) = cpv.get();

            let buffer_x = ((cpv_y / cpv_x) * pb_dis + (size_x as f64 / 2.0)) as usize;
            let buffer_y = (-(cpv_z / cpv_x) * pb_dis + (size_y as f64 / 2.0)) as usize;

            // Prevent going out of bounds
            if buffer_x >= size_x || buffer_y >= size_y {
                continue;
            }

            if z_buffer[buffer_y * size_x + buffer_x] > pv_dis {
                z_buffer[buffer_y * size_x + buffer_x] = pv_dis;

                let p_normal = point.normal.rotate(object_rotation).normalise();

                let mut lumi_index: i32 = 0;
                for light in &view.lights {
                    let light_coord = light.coord;

                    let lp = (light_coord - point_coord).to_vector().normalise();

                    let angle =
                        f64::acos(p_normal.dot(lp) / (p_normal.magnitude() * lp.magnitude()));

                    lumi_index = ((1.0 - (angle / PI)) * (lumi_length as f64)) as i32;
                }

                projected_buffer[buffer_y * size_x + buffer_x] =
                    luminance[lumi_index as usize] as char;
            }
        }
    }
    projected_buffer
}
