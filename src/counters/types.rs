use objc2::{Encode, Encoding, RefEncode};
use objc2_foundation::{NSErrorDomain, NSString};

/// Common counters that, when present, have similar meanings across implementations.
pub type CommonCounter = NSString;

extern "C" {
    pub static MTLCommonCounterTimestamp: &'static CommonCounter;
    pub static MTLCommonCounterTessellationInputPatches: &'static CommonCounter;
    pub static MTLCommonCounterVertexInvocations: &'static CommonCounter;
    pub static MTLCommonCounterPostTessellationVertexInvocations: &'static CommonCounter;
    pub static MTLCommonCounterClipperInvocations: &'static CommonCounter;
    pub static MTLCommonCounterClipperPrimitivesOut: &'static CommonCounter;
    pub static MTLCommonCounterFragmentInvocations: &'static CommonCounter;
    pub static MTLCommonCounterFragmentsPassed: &'static CommonCounter;
    pub static MTLCommonCounterComputeKernelInvocations: &'static CommonCounter;
    pub static MTLCommonCounterTotalCycles: &'static CommonCounter;
    pub static MTLCommonCounterVertexCycles: &'static CommonCounter;
    pub static MTLCommonCounterTessellationCycles: &'static CommonCounter;
    pub static MTLCommonCounterPostTessellationVertexCycles: &'static CommonCounter;
    pub static MTLCommonCounterFragmentCycles: &'static CommonCounter;
    pub static MTLCommonCounterRenderTargetWriteCycles: &'static CommonCounter;
}

/// Common counter set names.
pub type CommonCounterSet = NSString;

extern "C" {
    pub static MTLCommonCounterSetTimestamp: &'static CommonCounterSet;
    pub static MTLCommonCounterSetStageUtilization: &'static CommonCounterSet;
    pub static MTLCommonCounterSetStatistic: &'static CommonCounterSet;
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CounterResultTimestamp {
    pub timestamp: u64,
}

unsafe impl Encode for CounterResultTimestamp {
    const ENCODING: Encoding = Encoding::Struct("{?=Q}", &[<u64>::ENCODING]);
}

unsafe impl RefEncode for CounterResultTimestamp {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CounterResultStageUtilization {
    pub total_cycles: u64,
    pub vertex_cycles: u64,
    pub tessellation_cycles: u64,
    pub post_tessellation_vertex_cycles: u64,
    pub fragment_cycles: u64,
    pub render_target_cycles: u64,
}

unsafe impl Encode for CounterResultStageUtilization {
    const ENCODING: Encoding = Encoding::Struct(
        "{?=QQQQQQ}",
        &[
            <u64>::ENCODING,
            <u64>::ENCODING,
            <u64>::ENCODING,
            <u64>::ENCODING,
            <u64>::ENCODING,
            <u64>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for CounterResultStageUtilization {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CounterResultStatistic {
    pub tessellation_input_patches: u64,
    pub vertex_invocations: u64,
    pub post_tessellation_vertex_invocations: u64,
    pub clipper_invocations: u64,
    pub clipper_primitives_out: u64,
    pub fragment_invocations: u64,
    pub fragments_passed: u64,
    pub compute_kernel_invocations: u64,
}

unsafe impl Encode for CounterResultStatistic {
    const ENCODING: Encoding = Encoding::Struct(
        "{?=QQQQQQQQ}",
        &[
            <u64>::ENCODING,
            <u64>::ENCODING,
            <u64>::ENCODING,
            <u64>::ENCODING,
            <u64>::ENCODING,
            <u64>::ENCODING,
            <u64>::ENCODING,
            <u64>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for CounterResultStatistic {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    static MTLCounterErrorDomain: &'static NSErrorDomain;
}

#[inline]
pub fn counter_error_domain() -> &'static NSErrorDomain {
    unsafe { MTLCounterErrorDomain }
}


