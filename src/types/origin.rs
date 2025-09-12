use objc2::{Encode, Encoding, RefEncode};

/// Identify a pixel in an image. Usually used as the upper-left corner of a region of a texture.
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Origin {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

unsafe impl Encode for Origin {
    const ENCODING: Encoding = Encoding::Struct("{MTLOrigin=QQQ}", &[usize::ENCODING, usize::ENCODING, usize::ENCODING]);
}

unsafe impl RefEncode for Origin {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}


