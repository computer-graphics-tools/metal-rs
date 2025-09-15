use objc2::{Encode, Encoding, RefEncode};

/// Controls the blit operation (from `MTLBlitOption`).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BlitOption(pub u64);

bitflags::bitflags! {
    impl BlitOption: u64 {
        const None = 0;
        const DepthFromDepthStencil = 1<<0;
        const StencilFromDepthStencil = 1<<1;
        const RowLinearPVRTC = 1<<2;
    }
}

unsafe impl Encode for BlitOption {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for BlitOption {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
