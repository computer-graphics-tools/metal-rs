use objc2::{Encode, Encoding, RefEncode};

/// Reduction mode for sampler filtering (from `MTLSamplerReductionMode`).
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum ReductionMode {
    WeightedAverage = 0,
    Minimum = 1,
    Maximum = 2,
}

unsafe impl Encode for ReductionMode {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for ReductionMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
