use std::f64;

// Algorithm for rotating the image
pub(crate) fn rotate<P>(
    buf: &[P],
    empty_color: &P,
    width: usize,
    height: usize,
    rotation: f64,
    down_scale_factor: usize,
) -> (usize, usize, Vec<P>)
where
    P: Clone,
{
    // Always keep the rotation in the 0.0-360.0 range
    let rotation = rotation.rem_euclid(360.0);

    // If rotation is any of 0.0, 90.0, 180.0 or 270.0 we can do a much faster calculation
    if rotation % 90.0 == 0.0 {
        let (width, height, downscaled) = downscale(buf, width, height, down_scale_factor);

        return if (rotation - 90.0).abs() < f64::EPSILON {
            rotate90(&downscaled, width, height)
        } else if (rotation - 180.0).abs() < f64::EPSILON {
            rotate180(&downscaled, width, height)
        } else if (rotation - 270.0).abs() < f64::EPSILON {
            rotate270(&downscaled, width, height)
        } else {
            (width, height, downscaled.to_vec())
        };
    }

    // The downscaled size
    let new_width = width as f64 / down_scale_factor as f64;
    let new_height = height as f64 / down_scale_factor as f64;

    let radians = rotation * f64::consts::PI / 180.0;
    let sin_angle = (radians).sin();
    let cos_angle = (radians).cos();

    // First calculate the new size
    let half_width = new_width / 2.0;
    let half_width_cos = half_width * cos_angle;
    let half_width_sin = half_width * sin_angle;
    let half_height = new_height / 2.0;
    let half_height_sin = half_height * sin_angle;
    let half_height_cos = half_height * cos_angle;
    let x_coords = [
        half_width_cos + half_height_sin,
        -half_width_cos + half_height_sin,
        -half_width_cos - half_height_sin,
        half_width_cos - half_height_sin,
    ];
    let y_coords = [
        half_width_sin + half_height_cos,
        -half_width_sin + half_height_cos,
        -half_width_sin - half_height_cos,
        half_width_sin - half_height_cos,
    ];

    // Get the min and max values of all the coordinates
    let min_x = x_coords.iter().cloned().fold(f64::INFINITY, f64::min) + f64::EPSILON;
    let max_x = x_coords.iter().cloned().fold(f64::NEG_INFINITY, f64::max) - f64::EPSILON;
    let min_y = y_coords.iter().cloned().fold(f64::INFINITY, f64::min) + f64::EPSILON;
    let max_y = y_coords.iter().cloned().fold(f64::NEG_INFINITY, f64::max) - f64::EPSILON;

    let result_width = (max_x - min_x).abs().ceil();
    let result_height = (max_y - min_y).abs().ceil();

    let center_x = new_width / 2.0;
    let center_y = new_height / 2.0;

    let result_center_x = result_width / 2.0;
    let result_center_y = result_height / 2.0;

    let result_width = result_width as usize;
    let result_height = result_height as usize;

    let widthf64 = width as f64;
    let heightf64 = height as f64;
    let down_scale_factorf64 = down_scale_factor as f64;

    // Create the downscaled and rotated result buffer
    let mut rotated = vec![empty_color.clone(); result_width * result_height];
    for y in 0..result_height {
        let yf64 = y as f64;

        let center_offset_y = yf64 - result_center_y;

        for x in 0..result_width {
            let xf64 = x as f64;

            let center_offset_x = xf64 - result_center_x;

            // Calculate the rotation of where we need to look for the pixel
            let dir = f64::atan2(center_offset_y, center_offset_x) - radians;
            // Calculate the distance of where we need to look
            let mag = (center_offset_x * center_offset_x + center_offset_y * center_offset_y)
                .sqrt()
                * down_scale_factorf64;

            let orig_x = center_x * down_scale_factorf64 + mag * dir.cos();
            if orig_x >= 0.0 && orig_x < widthf64 {
                let orig_y = center_y * down_scale_factorf64 + mag * dir.sin();
                if orig_y >= 0.0 && orig_y < heightf64 {
                    rotated[y * result_width + x] =
                        buf[orig_y as usize * width + orig_x as usize].clone();
                }
            }
        }
    }

    (result_width, result_height, rotated)
}

fn rotate90<P>(buf: &[P], width: usize, height: usize) -> (usize, usize, Vec<P>)
where
    P: Clone,
{
    // 1, 2, 3
    // 4, 5, 6
    // ->
    // 4, 1
    // 5, 2
    // 6, 3

    let mut rotated = buf.to_vec();
    for y in 0..height {
        for x in 0..width {
            rotated[x * height + (height - y - 1)] = buf[y * width + x].clone();
        }
    }

    (height, width, rotated)
}

fn rotate180<P>(buf: &[P], width: usize, height: usize) -> (usize, usize, Vec<P>)
where
    P: Clone,
{
    // 1, 2, 3
    // 4, 5, 6
    // ->
    // 6, 5, 4
    // 3, 2, 1

    let mut rotated = buf.to_vec();
    rotated.reverse();

    (width, height, rotated)
}

fn rotate270<P>(buf: &[P], width: usize, height: usize) -> (usize, usize, Vec<P>)
where
    P: Clone,
{
    // 1, 2, 3
    // 4, 5, 6
    // ->
    // 3, 6
    // 2, 5
    // 1, 2

    let (width, height, rotated) = rotate90(buf, width, height);

    rotate180(&rotated, width, height)
}

fn downscale<P>(buf: &[P], width: usize, height: usize, factor: usize) -> (usize, usize, Vec<P>)
where
    P: Clone,
{
    let new_width = width / factor;
    let new_height = height / factor;

    let mut scaled = vec![buf[0].clone(); new_width * new_height];

    for y in 0..new_height {
        let y_row = y * new_width;
        let y_row_scaled = y * factor * width;
        for x in 0..new_width {
            scaled[y_row + x] = buf[y_row_scaled + x * factor].clone();
        }
    }

    (new_width, new_height, scaled)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotation_90_deg() {
        let (w, h, new) = rotate90(&[1, 2, 3, 4, 5, 6], 3, 2);
        assert_eq!(w, 2);
        assert_eq!(h, 3);
        assert_eq!(new, [4, 1, 5, 2, 6, 3]);
    }

    #[test]
    fn rotation_180_deg() {
        let (w, h, new) = rotate180(&[1, 2, 3, 4, 5, 6], 3, 2);
        assert_eq!(w, 3);
        assert_eq!(h, 2);
        assert_eq!(new, [6, 5, 4, 3, 2, 1]);
    }

    #[test]
    fn rotation_270_deg() {
        let (w, h, new) = rotate270(&[1, 2, 3, 4, 5, 6], 3, 2);
        assert_eq!(w, 2);
        assert_eq!(h, 3);
        assert_eq!(new, [3, 6, 2, 5, 1, 4]);
    }
}
