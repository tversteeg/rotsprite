//! Pixel Art rotation algorithms that works with many types of pixel buffers.
//!
//! This library allows you to rotate pixel art using the [rotsprite](https://en.wikipedia.org/wiki/Pixel-art_scaling_algorithms#RotSprite) algorithm.

// Make the modules public for benchmarks but don't document it
#[cfg(feature = "blit")]
mod blit;
#[doc(hidden)]
pub mod rotate;
#[doc(hidden)]
pub mod scale2x;

use crate::{rotate::*, scale2x::*};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("image size doesn't match with supplied width")]
    ImageSizeMismatch,
}

/// Expose `rotsprite` method on some image types.
pub trait Rotsprite<P>
where
    P: Clone + Eq,
{
    /// Clone and rotate a sprite.
    ///
    /// Rotation is in degrees (0-360).
    /// The size of the resulting vector will be bigger if the rotation isn't exactly 0.0, 90.0, 180.0 or 270.0 degrees.
    /// The width and the height will be swapped at angles of 90.0 and 270.0.
    fn rotsprite(&self, rotation: f64) -> Result<Self, Error>
    where
        Self: Sized;
}

/// Rotate a sprite based on any pixel format implementing the `Eq` and `Clone` traits.
///
/// Rotation is in degrees (0-360).
/// The size of the resulting vector will be bigger if the rotation isn't exactly 0.0, 90.0, 180.0 or 270.0 degrees.
/// The width and the height will be swapped at angles of 90.0 and 270.0.
#[multiversion::multiversion(
    targets("x86_64+sse3", "x86_64+sse3+avx", "x86_64+sse3+avx2"),
    dispatcher = "static"
)]
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
    let (scaled_width, scaled_height, scaled) = scale2x(buf, width, height);
    // 4x
    let (scaled_width, scaled_height, scaled) = scale2x(&scaled, scaled_width, scaled_height);
    // 8x
    let (scaled_width, scaled_height, scaled) = scale2x(&scaled, scaled_width, scaled_height);

    // Rotate the image
    let rotated = rotate(
        &scaled,
        empty_color,
        scaled_width,
        scaled_height,
        rotation,
        8,
    );

    Ok(rotated)
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
}
