use bitflags::bitflags;
use objc2::{Encode, Encoding, RefEncode};

bitflags! {
    /// Pipeline creation options (ported from `MTLPipelineOption`).
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
    pub struct MTLPipelineOption: usize {
        const NONE = 0;
        #[deprecated(note = "use BINDING_INFO")]
        const ARGUMENT_INFO = 1 << 0;
        const BINDING_INFO = 1 << 0;
        const BUFFER_TYPE_INFO = 1 << 1;
        const FAIL_ON_BINARY_ARCHIVE_MISS = 1 << 2;
    }
}

unsafe impl Encode for MTLPipelineOption {
    const ENCODING: Encoding = usize::ENCODING;
}

unsafe impl RefEncode for MTLPipelineOption {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[cfg(test)]
#[allow(deprecated)]
mod tests {
    use super::MTLPipelineOption;

    #[test]
    fn values_match_metal_header() {
        assert_eq!(MTLPipelineOption::NONE.bits(), 0);
        assert_eq!(MTLPipelineOption::ARGUMENT_INFO, MTLPipelineOption::BINDING_INFO);
        assert_eq!(MTLPipelineOption::BUFFER_TYPE_INFO.bits(), 1 << 1);
        assert_eq!(MTLPipelineOption::FAIL_ON_BINARY_ARCHIVE_MISS.bits(), 1 << 2);
    }
}
