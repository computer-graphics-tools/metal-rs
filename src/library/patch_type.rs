use objc2::{Encode, Encoding, RefEncode};

/// The kind of tessellation patch produced by a post-tessellation vertex function.
///
/// Availability: macOS 10.12+, iOS 10.0+
#[repr(usize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLPatchType {
    None = 0,
    Triangle = 1,
    Quad = 2,
}

unsafe impl Encode for MTLPatchType {
    const ENCODING: Encoding = usize::ENCODING;
}

unsafe impl RefEncode for MTLPatchType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[cfg(test)]
mod tests {
    use super::MTLPatchType;

    #[test]
    fn patch_type_values_match_header() {
        assert_eq!(MTLPatchType::None as usize, 0);
        assert_eq!(MTLPatchType::Triangle as usize, 1);
        assert_eq!(MTLPatchType::Quad as usize, 2);
    }
}
