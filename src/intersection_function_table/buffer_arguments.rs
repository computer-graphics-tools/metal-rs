use objc2::{Encode, Encoding, RefEncode};

/// Arguments for intersection function buffers (from `MTLIntersectionFunctionBufferArguments`).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MTLIntersectionFunctionBufferArguments {
    pub intersection_function_buffer: u64,
    pub intersection_function_buffer_size: u64,
    pub intersection_function_stride: u64,
}

unsafe impl Encode for MTLIntersectionFunctionBufferArguments {
    const ENCODING: Encoding = Encoding::Struct(
        "{MTLIntersectionFunctionBufferArguments=QQQ}",
        &[u64::ENCODING, u64::ENCODING, u64::ENCODING],
    );
}

unsafe impl RefEncode for MTLIntersectionFunctionBufferArguments {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
