use objc2::{Encode, Encoding, RefEncode};

/// Color write mask (from `MTLColorWriteMask`).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ColorWriteMask(pub u64);

bitflags::bitflags! {
    impl ColorWriteMask: u64 {
        const None = 0;
        const Red = 0x1<<3;
        const Green = 0x1<<2;
        const Blue = 0x1<<1;
        const Alpha = 0x1<<0;
        const All = 0xF;
        /// Defers assigning the color write mask. Behaves as All until specialized.
        const Unspecialized = 0x10;
    }
}

unsafe impl Encode for ColorWriteMask {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for ColorWriteMask {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
