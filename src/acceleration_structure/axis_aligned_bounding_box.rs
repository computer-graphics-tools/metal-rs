use objc2::{Encode, Encoding, RefEncode};

use super::packed::MTLPackedFloat3;

/// Axis aligned bounding box with min and max points.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MTLAxisAlignedBoundingBox {
    pub min: MTLPackedFloat3,
    pub max: MTLPackedFloat3,
}

unsafe impl Encode for MTLAxisAlignedBoundingBox {
    const ENCODING: Encoding = Encoding::Struct(
        "_MTLAxisAlignedBoundingBox",
        &[<MTLPackedFloat3>::ENCODING, <MTLPackedFloat3>::ENCODING],
    );
}

unsafe impl RefEncode for MTLAxisAlignedBoundingBox {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
