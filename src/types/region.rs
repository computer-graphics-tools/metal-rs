use objc2::{Encode, Encoding, RefEncode};
use crate::types::{Origin, Size};

/// Identify a region in an image or texture.
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Region {
    pub origin: Origin,
    pub size: Size,
}

unsafe impl Encode for Region {
    const ENCODING: Encoding = Encoding::Struct(
        "{MTLRegion={MTLOrigin=QQQ}{MTLSize=QQQ}}",
        &[Origin::ENCODING, Size::ENCODING],
    );
}

unsafe impl RefEncode for Region {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}


