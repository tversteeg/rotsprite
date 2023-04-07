use blit::BlitBuffer;

use crate::{Error, Rotsprite};

impl Rotsprite<u32> for BlitBuffer {
    fn rotsprite(&self, rotation: f64) -> Result<Self, Error> {
        // Rotate the current blitbuffer, using 0x00_00_00_00 for the empty color
        let (width, _, pixels) = crate::rotsprite(
            self.pixels(),
            &0x00_00_00_00,
            self.width() as usize,
            rotation,
        )?;

        // Create a new blitbuffer from the pixels
        Ok(Self::from_iter(pixels.into_iter(), width as i32, 1))
    }
}
