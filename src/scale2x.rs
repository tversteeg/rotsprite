// Algorithm for fast upscaling of pixel art sprites
pub fn scale2x<P>(buf: &[P], width: usize, height: usize) -> (usize, usize, Vec<P>)
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

    (width2, height2, scaled)
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
        (if left == up && left != down && up != right {
            up
        } else {
            center
        })
        .clone(),
        (if up == right && up != left && right != down {
            right
        } else {
            center
        })
        .clone(),
        (if down == left && down != right && left != up {
            left
        } else {
            center
        })
        .clone(),
        (if right == down && right != up && down != left {
            down
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
    fn scale2x_test() {
        let buf = [1, 2, 3, 4];
        let (w, h, new) = scale2x(&buf, 2, 2);
        assert_eq!(w, 4);
        assert_eq!(h, 4);
        assert_eq!(new, [1, 1, 2, 2, 1, 1, 2, 2, 3, 3, 4, 4, 3, 3, 4, 4]);

        let buf = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        let (_, _, new) = scale2x(&buf, 3, 3);
        let mut cmp = Vec::<usize>::new();
        cmp.extend([1, 1, 2, 2, 3, 3].iter());
        cmp.extend([1, 1, 2, 2, 3, 3].iter());
        cmp.extend([4, 4, 5, 5, 6, 6].iter());
        cmp.extend([4, 4, 5, 5, 6, 6].iter());
        cmp.extend([7, 7, 8, 8, 9, 9].iter());
        cmp.extend([7, 7, 8, 8, 9, 9].iter());
        assert_eq!(new, cmp);

        let buf = [1, 2, 3, 4, 5, 6];
        let (_, _, new) = scale2x(&buf, 3, 2);
        assert_eq!(
            new,
            [1, 1, 2, 2, 3, 3, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 4, 4, 5, 5, 6, 6]
        );
    }
}
