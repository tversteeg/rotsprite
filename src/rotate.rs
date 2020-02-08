use std::f64;

const FLOAT_ROUNDING_ERR: f64 = 0.0001;

// Algorithm for rotating the image
pub(crate) fn rotate<P>(
    buf: &[P],
    empty_color: &P,
    width: f64,
    height: f64,
    rotation: f64,
) -> (usize, usize, Vec<P>)
where
    P: Eq + Clone,
{
    let radians = rotation * f64::consts::PI / 180.0;
    let sin_angle = (radians).sin();
    let cos_angle = (radians).cos();

    // First calculate the new size
    let half_width = width / 2.0;
    let half_width_cos = half_width * cos_angle;
    let half_width_sin = half_width * sin_angle;
    let half_height = height / 2.0;
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
    let min_x = x_coords.iter().cloned().fold(1.0f64 / 0.0f64, f64::min) + FLOAT_ROUNDING_ERR;
    let max_x = x_coords.iter().cloned().fold(0.0f64 / 0.0f64, f64::max) - FLOAT_ROUNDING_ERR;
    let min_y = y_coords.iter().cloned().fold(1.0f64 / 0.0f64, f64::min) + FLOAT_ROUNDING_ERR;
    let max_y = y_coords.iter().cloned().fold(0.0f64 / 0.0f64, f64::max) - FLOAT_ROUNDING_ERR;

    let new_width = (max_x - min_x).abs().ceil() as usize;
    let new_height = (max_y - min_y).abs().ceil() as usize;

    let mut rotated = vec![empty_color.clone(); new_width * new_height];

    (new_width, new_height, rotated)
}
