use objc2::{Encode, Encoding, RefEncode};

use crate::types::{MTLOrigin, MTLSize};

/// Identify a region in an image or texture.
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct MTLRegion {
    pub origin: MTLOrigin,
    pub size: MTLSize,
}

unsafe impl Encode for MTLRegion {
    const ENCODING: Encoding = Encoding::Struct("?", &[MTLOrigin::ENCODING, MTLSize::ENCODING]);
}

unsafe impl RefEncode for MTLRegion {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

impl MTLRegion {
    /// Creates a one-dimensional region.
    ///
    /// This is the Rust equivalent of Metal's `MTLRegionMake1D` inline helper.
    pub const fn new_1d(
        x: usize,
        width: usize,
    ) -> Self {
        Self {
            origin: MTLOrigin::new(x, 0, 0),
            size: MTLSize::new(width, 1, 1),
        }
    }

    /// Creates a two-dimensional region.
    ///
    /// This is the Rust equivalent of Metal's `MTLRegionMake2D` inline helper.
    pub const fn new_2d(
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) -> Self {
        Self {
            origin: MTLOrigin::new(x, y, 0),
            size: MTLSize::new(width, height, 1),
        }
    }

    /// Creates a three-dimensional region.
    ///
    /// This is the Rust equivalent of Metal's `MTLRegionMake3D` inline helper.
    pub const fn new_3d(
        x: usize,
        y: usize,
        z: usize,
        width: usize,
        height: usize,
        depth: usize,
    ) -> Self {
        Self {
            origin: MTLOrigin::new(x, y, z),
            size: MTLSize::new(width, height, depth),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructors_match_metal_inline_helpers() {
        assert_eq!(
            MTLRegion::new_1d(2, 8),
            MTLRegion {
                origin: MTLOrigin::new(2, 0, 0),
                size: MTLSize::new(8, 1, 1),
            }
        );
        assert_eq!(
            MTLRegion::new_2d(2, 3, 8, 9),
            MTLRegion {
                origin: MTLOrigin::new(2, 3, 0),
                size: MTLSize::new(8, 9, 1),
            }
        );
        assert_eq!(
            MTLRegion::new_3d(2, 3, 4, 8, 9, 10),
            MTLRegion {
                origin: MTLOrigin::new(2, 3, 4),
                size: MTLSize::new(8, 9, 10),
            }
        );
    }
}
