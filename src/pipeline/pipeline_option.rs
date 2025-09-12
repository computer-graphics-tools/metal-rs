use bitflags::bitflags;
use objc2::{Encode, Encoding, RefEncode};

bitflags! {
    /// Pipeline creation options (ported from `MTLPipelineOption`).
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
    pub struct PipelineOption: usize {
        const BINDING_INFO = 1 << 0;
        const BUFFER_TYPE_INFO = 1 << 1;
        const FAIL_ON_BINARY_ARCHIVE_MISS = 1 << 2;
    }
}

unsafe impl Encode for PipelineOption {
    const ENCODING: Encoding = usize::ENCODING;
}

unsafe impl RefEncode for PipelineOption {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}


