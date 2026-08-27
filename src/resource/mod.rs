mod buffer_sparse_tier;
mod cpu_cache_mode;
mod hazard_tracking_mode;
mod purgeable_state;
mod resource;
mod resource_options;
mod sparse_page_size;
mod storage_mode;
mod texture_sparse_tier;

pub use buffer_sparse_tier::MTLBufferSparseTier;
pub use cpu_cache_mode::MTLCPUCacheMode;
pub use hazard_tracking_mode::MTLHazardTrackingMode;
pub use purgeable_state::MTLPurgeableState;
pub use resource::{MTLResource, MTLResourceExt};
pub use resource_options::{
    MTL_RESOURCE_CPU_CACHE_MODE_MASK, MTL_RESOURCE_CPU_CACHE_MODE_SHIFT, MTL_RESOURCE_HAZARD_TRACKING_MODE_MASK,
    MTL_RESOURCE_HAZARD_TRACKING_MODE_SHIFT, MTL_RESOURCE_STORAGE_MODE_MASK, MTL_RESOURCE_STORAGE_MODE_SHIFT,
    MTLResourceOptions,
};
pub use sparse_page_size::MTLSparsePageSize;
pub use storage_mode::MTLStorageMode;
pub use texture_sparse_tier::MTLTextureSparseTier;

#[cfg(test)]
mod tests {
    use core::mem::{align_of, size_of};

    use objc2::runtime::ProtocolObject;

    use super::*;
    use crate::{MTLAccelerationStructure, MTLAllocation, MTLBuffer, MTLTexture};

    #[test]
    fn enum_and_option_types_match_platform_integer_abi() {
        for (size, alignment) in [
            (size_of::<MTLPurgeableState>(), align_of::<MTLPurgeableState>()),
            (size_of::<MTLCPUCacheMode>(), align_of::<MTLCPUCacheMode>()),
            (size_of::<MTLStorageMode>(), align_of::<MTLStorageMode>()),
            (size_of::<MTLHazardTrackingMode>(), align_of::<MTLHazardTrackingMode>()),
            (size_of::<MTLResourceOptions>(), align_of::<MTLResourceOptions>()),
        ] {
            assert_eq!((size, alignment), (size_of::<usize>(), align_of::<usize>()));
        }

        for (size, alignment) in [
            (size_of::<MTLSparsePageSize>(), align_of::<MTLSparsePageSize>()),
            (size_of::<MTLBufferSparseTier>(), align_of::<MTLBufferSparseTier>()),
            (size_of::<MTLTextureSparseTier>(), align_of::<MTLTextureSparseTier>()),
        ] {
            assert_eq!((size, alignment), (size_of::<isize>(), align_of::<isize>()));
        }
    }

    #[test]
    #[allow(deprecated)]
    fn enum_values_match_xcode_27_header() {
        assert_eq!(
            [
                MTLPurgeableState::KeepCurrent as usize,
                MTLPurgeableState::NonVolatile as usize,
                MTLPurgeableState::Volatile as usize,
                MTLPurgeableState::Empty as usize,
            ],
            [1, 2, 3, 4],
        );
        assert_eq!([MTLCPUCacheMode::DefaultCache as usize, MTLCPUCacheMode::WriteCombined as usize], [0, 1]);
        assert_eq!(
            [
                MTLStorageMode::Shared as usize,
                MTLStorageMode::Managed as usize,
                MTLStorageMode::Private as usize,
                MTLStorageMode::Memoryless as usize,
            ],
            [0, 1, 2, 3],
        );
        assert_eq!(
            [
                MTLHazardTrackingMode::Default as usize,
                MTLHazardTrackingMode::Untracked as usize,
                MTLHazardTrackingMode::Tracked as usize,
            ],
            [0, 1, 2],
        );
        assert_eq!(
            [MTLSparsePageSize::KB16 as isize, MTLSparsePageSize::KB64 as isize, MTLSparsePageSize::KB256 as isize,],
            [101, 102, 103],
        );
        assert_eq!([MTLBufferSparseTier::None as isize, MTLBufferSparseTier::Tier1 as isize], [0, 1]);
        assert_eq!(
            [
                MTLTextureSparseTier::None as isize,
                MTLTextureSparseTier::Tier1 as isize,
                MTLTextureSparseTier::Tier2 as isize,
            ],
            [0, 1, 2],
        );
    }

    #[test]
    #[allow(deprecated)]
    fn resource_option_values_and_masks_match_xcode_27_header() {
        assert_eq!(MTL_RESOURCE_CPU_CACHE_MODE_SHIFT, 0);
        assert_eq!(MTL_RESOURCE_CPU_CACHE_MODE_MASK, 0xf);
        assert_eq!(MTL_RESOURCE_STORAGE_MODE_SHIFT, 4);
        assert_eq!(MTL_RESOURCE_STORAGE_MODE_MASK, 0xf0);
        assert_eq!(MTL_RESOURCE_HAZARD_TRACKING_MODE_SHIFT, 8);
        assert_eq!(MTL_RESOURCE_HAZARD_TRACKING_MODE_MASK, 0x300);

        assert_eq!(MTLResourceOptions::CPU_CACHE_MODE_DEFAULT_CACHE.bits(), 0);
        assert_eq!(MTLResourceOptions::CPU_CACHE_MODE_WRITE_COMBINED.bits(), 1);
        assert_eq!(MTLResourceOptions::STORAGE_MODE_SHARED.bits(), 0);
        assert_eq!(MTLResourceOptions::STORAGE_MODE_MANAGED.bits(), 1 << 4);
        assert_eq!(MTLResourceOptions::STORAGE_MODE_PRIVATE.bits(), 2 << 4);
        assert_eq!(MTLResourceOptions::STORAGE_MODE_MEMORYLESS.bits(), 3 << 4);
        assert_eq!(MTLResourceOptions::HAZARD_TRACKING_MODE_DEFAULT.bits(), 0);
        assert_eq!(MTLResourceOptions::HAZARD_TRACKING_MODE_UNTRACKED.bits(), 1 << 8);
        assert_eq!(MTLResourceOptions::HAZARD_TRACKING_MODE_TRACKED.bits(), 2 << 8);
        assert_eq!(MTLResourceOptions::OPTION_CPU_CACHE_MODE_DEFAULT.bits(), 0);
        assert_eq!(MTLResourceOptions::OPTION_CPU_CACHE_MODE_WRITE_COMBINED.bits(), 1);
    }

    #[test]
    fn resource_inheritance_and_extension_coverage_match_the_header() {
        fn assert_allocation<T: MTLAllocation + ?Sized>() {}
        fn assert_resource<T: MTLResource + ?Sized>() {}
        fn assert_resource_ext<T: MTLResourceExt>() {}

        assert_allocation::<dyn MTLResource>();
        assert_resource::<dyn MTLBuffer>();
        assert_resource::<dyn MTLTexture>();
        assert_resource::<dyn MTLAccelerationStructure>();
        assert_resource_ext::<ProtocolObject<dyn MTLResource>>();
        assert_resource_ext::<ProtocolObject<dyn MTLBuffer>>();
        assert_resource_ext::<ProtocolObject<dyn MTLTexture>>();
        assert_resource_ext::<ProtocolObject<dyn MTLAccelerationStructure>>();
    }
}
