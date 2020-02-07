//! # Pixel Art Rotation
//!
//! This library allows you to rotate pixel art using the [rotsprite](https://en.wikipedia.org/wiki/Pixel-art_scaling_algorithms#RotSprite) algorithm.

use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("image size doesn't match with supplied width")]
    ImageSizeMismatch,
}

// Convert a single pixel to an upscaled 2x2 block
#[inline(always)]
fn calculate_scale2x_block<P>(p: &P, a: &P, b: &P, c: &P, d: &P) -> (P, P, P, P)
where
    P: Eq + Clone,
{
    (
        (if c == a && c != d && a != b { a } else { p }).clone(),
        (if a == b && a != c && b != d { b } else { p }).clone(),
        (if d == c && d != b && c != a { c } else { p }).clone(),
        (if b == d && b != a && d != c { d } else { p }).clone(),
    )
}

// Apply the block on the buffer
#[inline(always)]
fn apply_scale2x_block<P>(scaled: &mut [P], pos: usize, width: usize, pixels: (P, P, P, P))
where
    P: Eq + Clone,
{
    scaled[pos] = pixels.0;
    scaled[pos + 1] = pixels.1;
    scaled[pos + width] = pixels.2;
    scaled[pos + width + 1] = pixels.3;
}

// Upscale the image using the scale2x algorithm
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
            let p = calculate_scale2x_block(
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
            );
            apply_scale2x_block(&mut scaled, scaled_y + x * 2, width2, p);
        }

        let y_prev = y_this - width;
        let y_next = y_this + width;

        // Left most column
        let p = &buf[y_this];
        let p = calculate_scale2x_block(p, &buf[y_prev], p, &buf[y_next], &buf[y_this + 1]);
        apply_scale2x_block(&mut scaled, scaled_y, width2, p);

        // Right most column
        let p = &buf[y_next - 1];
        let p = calculate_scale2x_block(
            p,
            &buf[y_this - 1],
            &buf[y_next - 2],
            &buf[y_next + width - 1],
            p,
        );
        apply_scale2x_block(&mut scaled, scaled_y + width2 - 2, width2, p);
    }

    for x in 1..width - 1 {
        // Apply the algorithm to the first row
        let p = &buf[x];
        let p = calculate_scale2x_block(p, p, &buf[x - 1], &buf[x + width], &buf[x + 1]);
        apply_scale2x_block(&mut scaled, x * 2, width2, p);

        // Apply the algorithm to the last row
        let pos = (height - 1) * width + x;
        let p = &buf[pos];
        let p = calculate_scale2x_block(p, &buf[pos - width], &buf[pos - 1], p, &buf[pos + 1]);
        let scaled_y_this = ((height - 1) * 2) * width2;
        apply_scale2x_block(&mut scaled, scaled_y_this + x * 2, width2, p);
    }

    // Apply the algorithms to the corners

    // Top left corner
    let p = &buf[0];
    let p = calculate_scale2x_block(p, p, p, &buf[width], &buf[1]);
    apply_scale2x_block(&mut scaled, 0, width2, p);

    // Top right corner
    let x_right = width - 1;
    let p = &buf[x_right];
    let p = calculate_scale2x_block(p, p, &buf[x_right - 1], &buf[x_right + width], p);
    apply_scale2x_block(&mut scaled, width2 - 2, width2, p);

    // Bottom left corner
    let y_bottom = (height - 1) * width;
    let p = &buf[y_bottom];
    let p = calculate_scale2x_block(p, &buf[y_bottom - width], p, p, &buf[y_bottom + 1]);
    apply_scale2x_block(&mut scaled, (height2 - 2) * width2, width2, p);

    // Bottom right corner
    let y_bottom_right = y_bottom + x_right;
    let p = &buf[y_bottom_right];
    let p = calculate_scale2x_block(
        p,
        &buf[y_bottom_right - width],
        &buf[y_bottom_right - 1],
        p,
        p,
    );
    apply_scale2x_block(&mut scaled, (height2 - 2) * width2 + width2 - 2, width2, p);

    scaled
}

/// Rotate a sprite based on any pixel format implementing the `Eq` and `Clone` traits.
///
/// Rotation is in degrees (0-360).
/// The size of the resulting vector will be bigger if the rotation isn't exactly 0.0, 90.0, 180.0 or 270.0 degrees.
pub fn rotsprite<P>(buf: &[P], width: usize, rotation: f64) -> Result<Vec<P>, Error>
where
    P: Eq + Clone,
{
    // If there's no rotation we don't have to do anything
    if rotation == 0.0 {
        return Ok(buf.to_vec());
    }

    let len = buf.len();
    if len % width != 0 {
        return Err(Error::ImageSizeMismatch);
    }
    let height = len / width;

    // 2x
    let scaled = scale2x(buf, width, height);
    // 4x
    let scaled = scale2x(&scaled, width, height);
    // 8x
    let scaled = scale2x(&scaled, width, height);

    Ok(scaled)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_rotation_test() -> Result<(), Error> {
        let buf = [1, 0, 0, 1, 1, 0].to_vec();
        let new = rotsprite(&buf, 2, 0.0)?;
        assert_eq!(buf, new);

        Ok(())
    }

    #[test]
    fn size_mismatch_error_test() {
        assert_eq!(
            rotsprite(&[0, 0, 0, 0, 0], 2, 1.0).unwrap_err(),
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
    }
}
