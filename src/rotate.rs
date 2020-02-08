use std::f64;

const FLOAT_ROUNDING_ERR: f64 = 0.0001;

// Algorithm for rotating the image
pub(crate) fn rotate<P>(
    buf: &[P],
    empty_color: &P,
    width: usize,
    height: usize,
    rotation: f64,
) -> (usize, usize, Vec<P>)
where
    P: Eq + Clone,
{
    let rotation = rotation % 360.0;

    // TODO other rotation degrees
    if rotation == 90.0 {
        return rotate90(buf, width, height);
    } else if rotation == 180.0 {
    } else if rotation == 270.0 {
    }

    let radians = rotation * f64::consts::PI / 180.0;
    let sin_angle = (radians).sin();
    let cos_angle = (radians).cos();

    // First calculate the new size
    let half_width = width as f64 / 2.0;
    let half_width_cos = half_width * cos_angle;
    let half_width_sin = half_width * sin_angle;
    let half_height = height as f64 / 2.0;
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
    let min_x = x_coords.iter().cloned().fold(f64::INFINITY, f64::min) + FLOAT_ROUNDING_ERR;
    let max_x = x_coords.iter().cloned().fold(f64::NEG_INFINITY, f64::max) - FLOAT_ROUNDING_ERR;
    let min_y = y_coords.iter().cloned().fold(f64::INFINITY, f64::min) + FLOAT_ROUNDING_ERR;
    let max_y = y_coords.iter().cloned().fold(f64::NEG_INFINITY, f64::max) - FLOAT_ROUNDING_ERR;

    let new_width = (max_x - min_x).abs().ceil() as usize;
    let new_height = (max_y - min_y).abs().ceil() as usize;

    let mut rotated = vec![empty_color.clone(); new_width * new_height];

    (new_width, new_height, rotated)
}

fn rotate90<P>(buf: &[P], width: usize, height: usize) -> (usize, usize, Vec<P>)
where
    P: Eq + Clone,
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
}
