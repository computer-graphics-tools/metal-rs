use objc2::{Encode, Encoding, RefEncode};

/// Controls the blit operation
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLBlitOption(pub u64);

bitflags::bitflags! {
    impl MTLBlitOption: u64 {
        const None = 0;
        const DepthFromDepthStencil = 1<<0;
        const StencilFromDepthStencil = 1<<1;
        const RowLinearPVRTC = 1<<2;
    }
}

unsafe impl Encode for MTLBlitOption {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLBlitOption {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[cfg(test)]
mod tests {
    use core::mem::{align_of, size_of};

    use super::MTLBlitOption;

    #[test]
    fn option_values_and_abi_match_the_xcode_27_header() {
        assert_eq!(MTLBlitOption::None.bits(), 0);
        assert_eq!(MTLBlitOption::DepthFromDepthStencil.bits(), 1 << 0);
        assert_eq!(MTLBlitOption::StencilFromDepthStencil.bits(), 1 << 1);
        assert_eq!(MTLBlitOption::RowLinearPVRTC.bits(), 1 << 2);
        assert_eq!(size_of::<MTLBlitOption>(), size_of::<usize>());
        assert_eq!(align_of::<MTLBlitOption>(), align_of::<usize>());
    }
}
