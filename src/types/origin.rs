use objc2::{Encode, Encoding, RefEncode};

/// Identify a pixel in an image. Usually used as the upper-left corner of a region of a texture.
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct MTLOrigin {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl MTLOrigin {
    /// Creates an origin with the given coordinates.
    ///
    /// This is the Rust equivalent of Metal's `MTLOriginMake` inline helper.
    pub const fn new(
        x: usize,
        y: usize,
        z: usize,
    ) -> Self {
        Self {
            x,
            y,
            z,
        }
    }
}

unsafe impl Encode for MTLOrigin {
    const ENCODING: Encoding = Encoding::Struct("?", &[usize::ENCODING, usize::ENCODING, usize::ENCODING]);
}

unsafe impl RefEncode for MTLOrigin {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
