use objc2::{Encode, Encoding, RefEncode};

/// A set of dimensions to declare the size of an object.
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct MTLSize {
    pub width: usize,
    pub height: usize,
    pub depth: usize,
}

unsafe impl Encode for MTLSize {
    const ENCODING: Encoding = Encoding::Struct("?", &[usize::ENCODING, usize::ENCODING, usize::ENCODING]);
}

unsafe impl RefEncode for MTLSize {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

impl MTLSize {
    /// Creates a size with the given dimensions.
    ///
    /// This is the Rust equivalent of Metal's `MTLSizeMake` inline helper.
    pub const fn new(
        width: usize,
        height: usize,
        depth: usize,
    ) -> Self {
        Self {
            width,
            height,
            depth,
        }
    }
}
