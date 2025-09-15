use objc2::{Encode, Encoding, RefEncode};

/// Indirect arguments for dispatchThreadgroups (from `MTLDispatchThreadgroupsIndirectArguments`).
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DispatchThreadgroupsIndirectArguments {
    pub threadgroups_per_grid: [u32; 3],
}

unsafe impl Encode for DispatchThreadgroupsIndirectArguments {
    const ENCODING: Encoding = Encoding::Struct(
        "{MTLDispatchThreadgroupsIndirectArguments=[3I]}",
        &[<[u32; 3]>::ENCODING],
    );
}

unsafe impl RefEncode for DispatchThreadgroupsIndirectArguments {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Indirect arguments for dispatchThreads (from `MTLDispatchThreadsIndirectArguments`).
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DispatchThreadsIndirectArguments {
    pub threads_per_grid: [u32; 3],
    pub threads_per_threadgroup: [u32; 3],
}

unsafe impl Encode for DispatchThreadsIndirectArguments {
    const ENCODING: Encoding = Encoding::Struct(
        "{MTLDispatchThreadsIndirectArguments=[3I][3I]}",
        &[<[u32; 3]>::ENCODING, <[u32; 3]>::ENCODING],
    );
}

unsafe impl RefEncode for DispatchThreadsIndirectArguments {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Indirect arguments for stage-in region (from `MTLStageInRegionIndirectArguments`).
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct StageInRegionIndirectArguments {
    pub stage_in_origin: [u32; 3],
    pub stage_in_size: [u32; 3],
}

unsafe impl Encode for StageInRegionIndirectArguments {
    const ENCODING: Encoding = Encoding::Struct(
        "{MTLStageInRegionIndirectArguments=[3I][3I]}",
        &[<[u32; 3]>::ENCODING, <[u32; 3]>::ENCODING],
    );
}

unsafe impl RefEncode for StageInRegionIndirectArguments {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
