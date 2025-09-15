use objc2::{Encode, Encoding, RefEncode};

/// Shader validation mode (from `MTLShaderValidation`).
#[repr(i64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum ShaderValidation {
    Default = 0,
    Enabled = 1,
    Disabled = 2,
}

unsafe impl Encode for ShaderValidation {
    const ENCODING: Encoding = i64::ENCODING;
}

unsafe impl RefEncode for ShaderValidation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
