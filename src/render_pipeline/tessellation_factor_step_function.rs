use objc2::{Encode, Encoding, RefEncode};

/// Tessellation factor step function (from `MTLTessellationFactorStepFunction`).
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum TessellationFactorStepFunction {
    Constant = 0,
    PerPatch = 1,
    PerInstance = 2,
    PerPatchAndPerInstance = 3,
}

unsafe impl Encode for TessellationFactorStepFunction {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for TessellationFactorStepFunction {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
