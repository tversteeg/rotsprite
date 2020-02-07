//! # Pixel Art Rotation
//!
//! This library allows you to rotate pixel art using the [rotsprite](https://en.wikipedia.org/wiki/Pixel-art_scaling_algorithms#RotSprite) algorithm.

use thiserror::Error;

use std::f64;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("image size doesn't match with supplied width")]
    ImageSizeMismatch,
}

/// Rotate a sprite based on any pixel format implementing the `Eq` and `Clone` traits.
///
/// Rotation is in degrees (0-360).
/// The size of the resulting vector will be bigger if the rotation isn't exactly 0.0, 90.0, 180.0 or 270.0 degrees.
/// The width and the height will be swapped at angles of 90.0 and 270.0.
pub fn rotsprite<P>(
    buf: &[P],
    empty_color: &P,
    width: usize,
    rotation: f64,
) -> Result<(usize, usize, Vec<P>), Error>
where
    P: Eq + Clone,
{
    // If there's no rotation we don't have to do anything
    if rotation == 0.0 {
        return Ok((width, buf.len() / width, buf.to_vec()));
    }

    let len = buf.len();
    if len % width != 0 {
        return Err(Error::ImageSizeMismatch);
    }
    let height = len / width;

    // Upscale the image using the scale2x algorithm
    // 2x
    let scaled = scale2x(buf, width, height);
    // 4x
    let scaled = scale2x(&scaled, width, height);
    // 8x
    let scaled = scale2x(&scaled, width, height);

    // Rotate the image
    let rotated = rotate(
        &scaled,
        empty_color,
        width as f64,
        height as f64,
        rotation % 360.0,
    );

    Ok(rotated)
}

// Algorithm for rotating the image
fn rotate<P>(
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
    let min_x = x_coords.iter().cloned().fold(1.0f64 / 0.0f64, f64::min);
    let max_x = x_coords.iter().cloned().fold(0.0f64 / 0.0f64, f64::max);
    let min_y = y_coords.iter().cloned().fold(1.0f64 / 0.0f64, f64::min);
    let max_y = y_coords.iter().cloned().fold(0.0f64 / 0.0f64, f64::max);

    let new_width = (max_x - min_x).abs().ceil() as usize;
    let new_height = (max_y - min_y).abs().ceil() as usize;

    let mut rotated = vec![empty_color.clone(); new_width * new_height];

    (new_width, new_height, rotated)
}

// Algorithm for fast upscaling of pixel art sprites
fn scale2x<P>(buf: &[P], width: usize, height: usize) -> Vec<P>
where
    P: Eq + Clone,
{
    let width2 = width * 2;
    let height2 = height * 2;

    let mut scaled = vec![buf[0].clone(); width2 * height2];

    // Apply the algorithm to the center
    for y in 1..height - 1 {
        let y_this = y * width;
        let scaled_y = y * 2 * width2;

        for x in 1..width - 1 {
            let pos = y_this + x;
            apply_scale2x_block(
                &mut scaled,
                scaled_y + x * 2,
                width2,
                (
                    // Center
                    &buf[pos],
                    // Up
                    &buf[pos - width],
                    // Left
                    &buf[pos - 1],
                    // Down
                    &buf[pos + width],
                    // Right
                    &buf[pos + 1],
                ),
            );
        }

        let y_prev = y_this - width;
        let y_next = y_this + width;

        // Left most column
        let p = &buf[y_this];
        apply_scale2x_block(
            &mut scaled,
            scaled_y,
            width2,
            (p, &buf[y_prev], p, &buf[y_next], &buf[y_this + 1]),
        );

        // Right most column
        let p = &buf[y_next - 1];
        apply_scale2x_block(
            &mut scaled,
            scaled_y + width2 - 2,
            width2,
            (
                p,
                &buf[y_this - 1],
                &buf[y_next - 2],
                &buf[y_next + width - 1],
                p,
            ),
        );
    }

    for x in 1..width - 1 {
        // Apply the algorithm to the first row
        let p = &buf[x];
        apply_scale2x_block(
            &mut scaled,
            x * 2,
            width2,
            (p, p, &buf[x - 1], &buf[x + width], &buf[x + 1]),
        );

        // Apply the algorithm to the last row
        let pos = (height - 1) * width + x;
        let p = &buf[pos];
        let scaled_y_this = ((height - 1) * 2) * width2;
        apply_scale2x_block(
            &mut scaled,
            scaled_y_this + x * 2,
            width2,
            (p, &buf[pos - width], &buf[pos - 1], p, &buf[pos + 1]),
        );
    }

    // Apply the algorithms to the corners

    // Top left corner
    let p = &buf[0];
    apply_scale2x_block(&mut scaled, 0, width2, (p, p, p, &buf[width], &buf[1]));

    // Top right corner
    let x_right = width - 1;
    let p = &buf[x_right];
    apply_scale2x_block(
        &mut scaled,
        width2 - 2,
        width2,
        (p, p, &buf[x_right - 1], &buf[x_right + width], p),
    );

    // Bottom left corner
    let y_bottom = (height - 1) * width;
    let p = &buf[y_bottom];
    apply_scale2x_block(
        &mut scaled,
        (height2 - 2) * width2,
        width2,
        (p, &buf[y_bottom - width], p, p, &buf[y_bottom + 1]),
    );

    // Bottom right corner
    let y_bottom_right = y_bottom + x_right;
    let p = &buf[y_bottom_right];
    apply_scale2x_block(
        &mut scaled,
        (height2 - 2) * width2 + width2 - 2,
        width2,
        (
            p,
            &buf[y_bottom_right - width],
            &buf[y_bottom_right - 1],
            p,
            p,
        ),
    );

    scaled
}

// Apply the block on the buffer
#[inline(always)]
fn apply_scale2x_block<P>(scaled: &mut [P], pos: usize, width: usize, pixels: (&P, &P, &P, &P, &P))
where
    P: Eq + Clone,
{
    let block_pixels = calculate_scale2x_block(pixels.0, pixels.1, pixels.2, pixels.3, pixels.4);
    scaled[pos] = block_pixels.0;
    scaled[pos + 1] = block_pixels.1;
    scaled[pos + width] = block_pixels.2;
    scaled[pos + width + 1] = block_pixels.3;
}

// Convert a single pixel to an upscaled 2x2 block
#[inline(always)]
fn calculate_scale2x_block<P>(center: &P, up: &P, left: &P, down: &P, right: &P) -> (P, P, P, P)
where
    P: Eq + Clone,
{
    (
        (if down == up && down != right && up != left {
            up
        } else {
            center
        })
        .clone(),
        (if up == left && up != down && left != right {
            left
        } else {
            center
        })
        .clone(),
        (if right == down && right != left && down != up {
            down
        } else {
            center
        })
        .clone(),
        (if left == right && left != up && right != down {
            right
        } else {
            center
        })
        .clone(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotation_equality() -> Result<(), Error> {
        let buf = [1, 2, 3, 4, 5, 6].to_vec();
        assert_eq!(
            rotsprite(&buf, &0, 3, 45.0)?,
            rotsprite(&buf, &0, 3, 45.0 + 360.0)?
        );
        assert_eq!(
            rotsprite(&buf, &0, 3, 45.0)?,
            rotsprite(&buf, &0, 3, 45.0 - 360.0)?
        );
        assert_eq!(
            rotsprite(&buf, &0, 3, 12.0)?,
            rotsprite(&buf, &0, 3, 12.0 - 360.0)?
        );

        Ok(())
    }

    #[test]
    fn rotation_size() -> Result<(), Error> {
        let buf = [1, 2, 3, 4, 5, 6].to_vec();
        let (w, h, _) = rotsprite(&buf, &0, 3, 45.0)?;
        assert_eq!(w, 4);
        assert_eq!(h, 4);
        let (w, h, _) = rotsprite(&buf, &0, 3, 90.0)?;
        assert_eq!(w, 2);
        assert_eq!(h, 3);
        let (w, h, _) = rotsprite(&buf, &0, 3, 180.0)?;
        assert_eq!(w, 3);
        assert_eq!(h, 2);

        Ok(())
    }

    #[test]
    fn rotation_test() -> Result<(), Error> {
        let buf = [1, 2, 3, 4, 5, 6].to_vec();
        let (w, h, new) = rotsprite(&buf, &0, 3, 90.0)?;
        assert_eq!(w, 2);
        assert_eq!(h, 3);
        assert_eq!(new, [4, 1, 5, 2, 6, 3]);

        Ok(())
    }

    #[test]
    fn no_rotation_test() -> Result<(), Error> {
        let buf = [1, 0, 0, 1, 1, 0].to_vec();
        let (w, h, new) = rotsprite(&buf, &-1, 2, 0.0)?;
        assert_eq!(w, 2);
        assert_eq!(h, 3);
        assert_eq!(buf, new);

        Ok(())
    }

    #[test]
    fn size_mismatch_error_test() {
        assert_eq!(
            rotsprite(&[0, 0, 0, 0, 0], &-1, 2, 1.0).unwrap_err(),
            Error::ImageSizeMismatch
        );
    }

    #[test]
    fn scale2x_test() {
        let buf = [1, 2, 3, 4];
        let new = scale2x(&buf, 2, 2);
        assert_eq!(new, [1, 1, 2, 2, 1, 1, 2, 2, 3, 3, 4, 4, 3, 3, 4, 4]);

        let buf = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        let new = scale2x(&buf, 3, 3);
        let mut cmp = Vec::<usize>::new();
        cmp.extend([1, 1, 2, 2, 3, 3].iter());
        cmp.extend([1, 1, 2, 2, 3, 3].iter());
        cmp.extend([4, 4, 5, 5, 6, 6].iter());
        cmp.extend([4, 4, 5, 5, 6, 6].iter());
        cmp.extend([7, 7, 8, 8, 9, 9].iter());
        cmp.extend([7, 7, 8, 8, 9, 9].iter());
        assert_eq!(new, cmp);

        let buf = [1, 2, 3, 4, 5, 6];
        let new = scale2x(&buf, 3, 2);
        assert_eq!(
            new,
            [1, 1, 2, 2, 3, 3, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 4, 4, 5, 5, 6, 6]
        );
    }
}
