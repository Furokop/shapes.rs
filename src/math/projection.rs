use crate::basetype::Vector3D;

use crate::util::buffer::SimpleTerminalBuffer;
use crate::view::Viewport;

use std::f64::consts::PI;

/// Perspective renderer implementation
pub fn pers_proj(view: &Viewport) -> SimpleTerminalBuffer {
    let luminance_str = ".,-~:;=!*#$@";
    let luminance: &[u8] = luminance_str.as_bytes();
    let lumi_length = luminance_str.len();

    let (size_x, size_y) = view.get_buffer_size();

    let (v_x, v_y, v_z) = view.camera.coord.get();
    let v_a = view.camera.angle();

    let mut z_buffer = vec![0.0; size_y * size_x];

    let mut projected_buffer = SimpleTerminalBuffer::new(size_x, size_y);

    let pb_dis = 1.0 / f64::tan(view.camera.fov) * (size_x as f64) / 2.0;

    for obj in &view.objects {
        let (o_x, o_y, o_z) = obj.location.get();
        for point in &obj.shape.points {
            let (mut p_x, mut p_y, mut p_z) = point.rel_coord.get();
            p_x += o_x;
            p_y += o_y;
            p_z += o_z;

            // Distance between point and camera in vector
            let pv_x = p_x - v_x;
            let pv_y = p_y - v_y;
            let pv_z = p_z - v_z;
            let pv = Vector3D::new(pv_x, pv_y, pv_z);

            // Distance between point and camera in scalar
            let pv_dis = pv.magnitude();

            // Camera transform by rotating pv with negative angle of camera
            let cpv = pv.rotate(v_a);
            let (cpv_x, cpv_y, cpv_z) = cpv.get();

            let buffer_x = ((cpv_y / cpv_x) * pb_dis + (size_x as f64 / 2.0)) as usize;
            let buffer_y = ((cpv_z / cpv_x) * pb_dis + (size_y as f64 / 2.0)) as usize;

            // Prevent going out of bounds
            if buffer_x >= size_x || buffer_y >= size_y {
                continue;
            }

            if z_buffer[buffer_y * size_x + buffer_x] < pv_dis {
                z_buffer[buffer_y * size_x + buffer_x] = pv_dis;

                let p_normal = point.normal.normalise();

                let mut lumi_index: i32 = 0;
                for light in &view.lights {
                    let (l_x, l_y, l_z) = light.coord.get();

                    let lp_x = l_x - p_x;
                    let lp_y = l_y - p_y;
                    let lp_z = l_z - p_z;
                    let lp = Vector3D::new(lp_x, lp_y, lp_z).normalise();

                    let angle = (p_normal.dot(&lp) / (p_normal.magnitude() * lp.magnitude()))
                        .clamp(-1.0, 1.0);

                    let angle = angle.acos();

                    lumi_index = ((1.0 - angle.abs() / PI) * lumi_length as f64) as i32;
                }

                if lumi_index < 0 || lumi_index >= lumi_length as i32 {
                    continue;
                }
                projected_buffer[buffer_y * size_x + buffer_x] =
                    luminance[lumi_index as usize] as char;
            }
        }
    }
    return projected_buffer;
}
