use std::f64;

// Algorithm for rotating the image
#[multiversion::multiversion]
#[clone(target = "[x86|x86_64]+sse3")]
#[clone(target = "[x86|x86_64]+sse3+avx")]
#[clone(target = "[x86|x86_64]+sse3+avx2")]
pub fn rotate<P>(
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

    let fwidth = width as f64;
    let fheight = height as f64;

    let radians = rotation.to_radians();
    let sin = radians.sin();
    let cos = radians.cos();

    let p1 = (-fheight * sin, fheight * cos);
    let p2 = (fwidth * cos - fheight * sin, fheight * cos + fwidth * sin);
    let p3 = (fwidth * cos, fwidth * sin);

    let min_x = [p1.0, p2.0, p3.0].iter().cloned().fold(0.0, f64::min);
    let min_y = [p1.1, p2.1, p3.1].iter().cloned().fold(0.0, f64::min);
    let max_x = if rotation > 90.0 && rotation < 180.0 {
        0.0
    } else {
        [p1.0, p2.0, p3.0]
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max)
    };
    let max_y = if rotation > 180.0 && rotation < 270.0 {
        0.0
    } else {
        [p1.1, p2.1, p3.1]
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max)
    };

    // Rotated sizie without scaling
    let result_width = (max_x.abs() - min_x).ceil();
    let result_height = (max_y.abs() - min_y).ceil();

    // Rotated size with scaling
    let fscale = down_scale_factor as f64;
    let result_buffer_width = (result_width / fscale).ceil() as usize;
    let result_buffer_height = (result_height / fscale).ceil() as usize;

    // Create the downscaled and rotated result buffer
    let mut rotated = vec![empty_color.clone(); result_buffer_width * result_buffer_height];

    for y in 0..result_height as usize {
        let fy = y as f64;

        let y_with_min = fy + min_y;

        let y_min_sin = y_with_min * sin;
        let y_min_cos = y_with_min * cos;

        for x in 0..result_width as usize {
            let fx = x as f64;

            let x_with_min = fx + min_x;

            let x_min_sin = x_with_min * sin;
            let x_min_cos = x_with_min * cos;

            let source_x = x_min_cos + y_min_sin;
            let source_y = y_min_cos - x_min_sin;

            if source_x >= 0.0 && source_x < fwidth && source_y >= 0.0 && source_y < fheight {
                let x_dst_pos = fx / fscale;
                let y_dst_pos = fy / fscale;
                rotated[y_dst_pos as usize * result_buffer_width + x_dst_pos as usize] =
                    buf[source_y as usize * width + source_x as usize].clone();
            }
        }
    }

    (result_buffer_width, result_buffer_height, rotated)
}

pub fn rotate90<P>(buf: &[P], width: usize, height: usize) -> (usize, usize, Vec<P>)
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

pub fn rotate180<P>(buf: &[P], width: usize, height: usize) -> (usize, usize, Vec<P>)
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

pub fn rotate270<P>(buf: &[P], width: usize, height: usize) -> (usize, usize, Vec<P>)
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

pub fn downscale<P>(buf: &[P], width: usize, height: usize, factor: usize) -> (usize, usize, Vec<P>)
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
