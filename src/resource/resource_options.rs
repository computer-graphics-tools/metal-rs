use bitflags::bitflags;
use objc2::{Encode, Encoding, RefEncode};

/// Bit offset of [`MTLCPUCacheMode`](super::MTLCPUCacheMode) in [`MTLResourceOptions`].
pub const MTL_RESOURCE_CPU_CACHE_MODE_SHIFT: usize = 0;
/// Mask of the [`MTLCPUCacheMode`](super::MTLCPUCacheMode) field in [`MTLResourceOptions`].
pub const MTL_RESOURCE_CPU_CACHE_MODE_MASK: usize = 0xf << MTL_RESOURCE_CPU_CACHE_MODE_SHIFT;
/// Bit offset of [`MTLStorageMode`](super::MTLStorageMode) in [`MTLResourceOptions`].
pub const MTL_RESOURCE_STORAGE_MODE_SHIFT: usize = 4;
/// Mask of the [`MTLStorageMode`](super::MTLStorageMode) field in [`MTLResourceOptions`].
pub const MTL_RESOURCE_STORAGE_MODE_MASK: usize = 0xf << MTL_RESOURCE_STORAGE_MODE_SHIFT;
/// Bit offset of [`MTLHazardTrackingMode`](super::MTLHazardTrackingMode) in [`MTLResourceOptions`].
pub const MTL_RESOURCE_HAZARD_TRACKING_MODE_SHIFT: usize = 8;
/// Mask of the [`MTLHazardTrackingMode`](super::MTLHazardTrackingMode) field in [`MTLResourceOptions`].
pub const MTL_RESOURCE_HAZARD_TRACKING_MODE_MASK: usize = 0x3 << MTL_RESOURCE_HAZARD_TRACKING_MODE_SHIFT;

bitflags! {
    /// A set of optional arguments to influence the creation of a resource (from `MTLResourceOptions`).
    ///
    /// Discussion:
    /// Note that resource options are a property of MTLTextureDescriptor (resourceOptions), so apply to texture creation.
    /// they are also passed directly into MTLBuffer creation methods.
    ///
    /// Availability: macOS 10.11+, iOS 8.0+
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
    pub struct MTLResourceOptions: usize {
        /// The default CPU cache mode for the resource.
        /// Applications should only investigate changing the cache mode if writing to normally cached buffers is known to
        /// cause performance issues due to cache pollution, as write combined memory can have surprising performance pitfalls.
        /// Another approach is to use non-temporal stores to normally cached memory (STNP on ARMv8, _mm_stream_* on x86_64).
        const CPU_CACHE_MODE_DEFAULT_CACHE  = 0 << MTL_RESOURCE_CPU_CACHE_MODE_SHIFT;
        /// Write combined memory is optimized for resources that the CPU will write into, but never read.
        /// On some implementations, writes may bypass caches avoiding cache pollution, and reads perform very poorly.
        const CPU_CACHE_MODE_WRITE_COMBINED = 1 << MTL_RESOURCE_CPU_CACHE_MODE_SHIFT;

        /// In this mode, CPU and device will nominally both use the same underlying memory when accessing the contents of the resource.
        /// However, coherency is only guaranteed at command buffer boundaries to minimize the required flushing of CPU and GPU caches.
        /// This is the default storage mode for iOS Textures.
        const STORAGE_MODE_SHARED    = 0 << MTL_RESOURCE_STORAGE_MODE_SHIFT;
        /// This mode relaxes the coherency requirements and requires that the developer make explicit requests to maintain
        /// coherency between a CPU and GPU version of the resource.  Changes due to CPU stores outside of the Metal API must be
        /// indicated to Metal via MTLBuffer::didModifyRange:(NSRange)range.  In order for CPU to access up to date GPU results,
        /// first, a blit synchronizations must be completed (see synchronize methods of MTLBlitCommandEncoder).
        /// Blit overhead is only incurred if GPU has modified the resource.
        /// This storage mode is only defined for OS X.
        /// This is the default storage mode for OS X Textures.
        ///
        /// Availability: macOS 10.11–27.0 and Mac Catalyst 13.0–27.0; unavailable on iOS.
        #[deprecated(note = "managed storage has no effect on Apple Silicon; use STORAGE_MODE_SHARED")]
        const STORAGE_MODE_MANAGED   = 1 << MTL_RESOURCE_STORAGE_MODE_SHIFT;
        /// This mode allows the data to be kept entirely to GPU (or driver) private memory that will never be accessed by the CPU directly, so no
        /// coherency of any kind must be maintained.
        const STORAGE_MODE_PRIVATE   = 2 << MTL_RESOURCE_STORAGE_MODE_SHIFT;
        /// This mode allows creation of resources that do not have a GPU or CPU memory backing, but do have on-chip storage for TBDR
        /// devices. The contents of the on-chip storage is undefined and does not persist, but its configuration is controlled by the
        /// MTLTexture descriptor. Textures created with MTLStorageModeMemoryless dont have an IOAccelResource at any point in their
        /// lifetime. The only way to populate such resource is to perform rendering operations on it. Blit operations are disallowed.
        ///
        /// Availability: macOS 11.0+, Mac Catalyst 14.0+, iOS 10.0+
        const STORAGE_MODE_MEMORYLESS= 3 << MTL_RESOURCE_STORAGE_MODE_SHIFT;

        /// This mode is the default for the context in which it is specified,
        /// which may be either MTLResourceHazardTrackingModeUntracked or MTLResourceHazardTrackingModeTracked.
        /// Refer to the point of use to determing the meaning of this flag.
        const HAZARD_TRACKING_MODE_DEFAULT   = 0 << MTL_RESOURCE_HAZARD_TRACKING_MODE_SHIFT;
        /// In this mode, command encoder dependencies for this resource are tracked manually with MTLFence.
        /// This value is the default for MTLHeap and resources sub-allocated from a MTLHeap,
        /// and may optionally be specified for non-heap resources.
        ///
        /// Availability: macOS 10.13+, iOS 10.0+
        const HAZARD_TRACKING_MODE_UNTRACKED = 1 << MTL_RESOURCE_HAZARD_TRACKING_MODE_SHIFT;
        /// In this mode, command encoder dependencies for this resource are tracked automatically.
        /// This value is the default for resources allocated from a MTLDevice,
        /// and may optionally be specified for MTLHeap and resources sub-allocated from such heaps.
        ///
        /// Availability: macOS 10.15+, iOS 13.0+
        const HAZARD_TRACKING_MODE_TRACKED   = 2 << MTL_RESOURCE_HAZARD_TRACKING_MODE_SHIFT;

        /// Deprecated spelling of [`Self::CPU_CACHE_MODE_DEFAULT_CACHE`].
        ///
        /// Deprecated in macOS 13.0 and iOS 16.0.
        #[deprecated(note = "use CPU_CACHE_MODE_DEFAULT_CACHE")]
        const OPTION_CPU_CACHE_MODE_DEFAULT = 0 << MTL_RESOURCE_CPU_CACHE_MODE_SHIFT;
        /// Deprecated spelling of [`Self::CPU_CACHE_MODE_WRITE_COMBINED`].
        ///
        /// Deprecated in macOS 13.0 and iOS 16.0.
        #[deprecated(note = "use CPU_CACHE_MODE_WRITE_COMBINED")]
        const OPTION_CPU_CACHE_MODE_WRITE_COMBINED = 1 << MTL_RESOURCE_CPU_CACHE_MODE_SHIFT;
    }
}

unsafe impl Encode for MTLResourceOptions {
    const ENCODING: Encoding = usize::ENCODING;
}

unsafe impl RefEncode for MTLResourceOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
