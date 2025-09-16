use objc2::{Encode, Encoding, RefEncode};
use objc2_foundation::{NSErrorDomain, NSString};

/// Common counters that, when present, have similar meanings across implementations.
pub type MTLCommonCounter = NSString;

#[allow(unused)]
unsafe extern "C" {
    pub static MTLCommonCounterTimestamp: &'static MTLCommonCounter;
    pub static MTLCommonCounterTessellationInputPatches: &'static MTLCommonCounter;
    pub static MTLCommonCounterVertexInvocations: &'static MTLCommonCounter;
    pub static MTLCommonCounterPostTessellationVertexInvocations: &'static MTLCommonCounter;
    pub static MTLCommonCounterClipperInvocations: &'static MTLCommonCounter;
    pub static MTLCommonCounterClipperPrimitivesOut: &'static MTLCommonCounter;
    pub static MTLCommonCounterFragmentInvocations: &'static MTLCommonCounter;
    pub static MTLCommonCounterFragmentsPassed: &'static MTLCommonCounter;
    pub static MTLCommonCounterComputeKernelInvocations: &'static MTLCommonCounter;
    pub static MTLCommonCounterTotalCycles: &'static MTLCommonCounter;
    pub static MTLCommonCounterVertexCycles: &'static MTLCommonCounter;
    pub static MTLCommonCounterTessellationCycles: &'static MTLCommonCounter;
    pub static MTLCommonCounterPostTessellationVertexCycles: &'static MTLCommonCounter;
    pub static MTLCommonCounterFragmentCycles: &'static MTLCommonCounter;
    pub static MTLCommonCounterRenderTargetWriteCycles: &'static MTLCommonCounter;
}

/// Common counter set names.
pub type MTLCommonCounterSet = NSString;

#[allow(unused)]
unsafe extern "C" {
    pub static MTLCommonCounterSetTimestamp: &'static MTLCommonCounterSet;
    pub static MTLCommonCounterSetStageUtilization: &'static MTLCommonCounterSet;
    pub static MTLCommonCounterSetStatistic: &'static MTLCommonCounterSet;
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MTLCounterResultTimestamp {
    pub timestamp: u64,
}

unsafe impl Encode for MTLCounterResultTimestamp {
    const ENCODING: Encoding = Encoding::Struct("{?=Q}", &[<u64>::ENCODING]);
}

unsafe impl RefEncode for MTLCounterResultTimestamp {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MTLCounterResultStageUtilization {
    pub total_cycles: u64,
    pub vertex_cycles: u64,
    pub tessellation_cycles: u64,
    pub post_tessellation_vertex_cycles: u64,
    pub fragment_cycles: u64,
    pub render_target_cycles: u64,
}

unsafe impl Encode for MTLCounterResultStageUtilization {
    const ENCODING: Encoding = Encoding::Struct(
        "{MTLCounterResultStageUtilization=QQQQQQ}",
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

unsafe impl RefEncode for MTLCounterResultStageUtilization {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MTLCounterResultStatistic {
    pub tessellation_input_patches: u64,
    pub vertex_invocations: u64,
    pub post_tessellation_vertex_invocations: u64,
    pub clipper_invocations: u64,
    pub clipper_primitives_out: u64,
    pub fragment_invocations: u64,
    pub fragments_passed: u64,
    pub compute_kernel_invocations: u64,
}

unsafe impl Encode for MTLCounterResultStatistic {
    const ENCODING: Encoding = Encoding::Struct(
        "{MTLCounterResultStatistic=QQQQQQQQ}",
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

unsafe impl RefEncode for MTLCounterResultStatistic {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

unsafe extern "C" {
    static MTLCounterErrorDomain: &'static NSErrorDomain;
}

#[inline]
pub fn counter_error_domain() -> &'static NSErrorDomain {
    unsafe { MTLCounterErrorDomain }
}
