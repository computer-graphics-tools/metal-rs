use objc2::{Message, extern_protocol, msg_send, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::NSString;

use crate::{
    MTLAllocation, MTLCPUCacheMode, MTLDevice, MTLHazardTrackingMode, MTLHeap, MTLPurgeableState, MTLResourceOptions,
    MTLStorageMode,
};

extern_protocol!(
    /// Common APIs available for MTLBuffer and MTLTexture instances
    ///
    /// Availability: macOS 10.11+, iOS 8.0+
    ///
    /// See also Apple's documentation: `https://developer.apple.com/documentation/metal/mtlresource?language=objc`
    ///
    /// # Safety
    ///
    /// Implementors must be valid Objective-C objects that conform to the
    /// `MTLResource` protocol.
    #[expect(
        clippy::missing_safety_doc,
        reason = "extern_protocol does not attach this safety section to its generated unsafe trait"
    )]
    pub unsafe trait MTLResource: MTLAllocation {
        /// The device this resource was created against.  This resource can only be used with this device.
        ///
        /// Availability: macOS 10.11+, iOS 8.0+
        #[unsafe(method(device))]
        #[unsafe(method_family = none)]
        fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;

        /// The cache mode used for the CPU mapping for this resource
        ///
        /// Availability: macOS 10.11+, iOS 8.0+
        #[unsafe(method(cpuCacheMode))]
        #[unsafe(method_family = none)]
        fn cpu_cache_mode(&self) -> MTLCPUCacheMode;

        /// The resource storage mode used for the CPU mapping for this resource
        ///
        /// Availability: macOS 10.11+, iOS 9.0+
        #[unsafe(method(storageMode))]
        #[unsafe(method_family = none)]
        fn storage_mode(&self) -> MTLStorageMode;

        /// Whether or not the resource is hazard tracked.
        ///
        /// This value can be either `MTLHazardTrackingModeUntracked` or `MTLHazardTrackingModeTracked`.
        /// Resources created from heaps are by default untracked, whereas resources created from the device are by default tracked.
        ///
        /// Availability: macOS 10.15+, iOS 13.0+
        #[unsafe(method(hazardTrackingMode))]
        #[unsafe(method_family = none)]
        fn hazard_tracking_mode(&self) -> MTLHazardTrackingMode;

        /// A packed tuple of the `storageMode`, `cpuCacheMode` and `hazardTrackingMode` properties.
        ///
        /// Availability: macOS 10.15+, iOS 13.0+
        #[unsafe(method(resourceOptions))]
        #[unsafe(method_family = none)]
        fn resource_options(&self) -> MTLResourceOptions;

        /// Set (or query) the purgeability state of a resource
        ///
        /// Synchronously set the purgeability state of a resource and return what the prior (or current) state is.
        /// FIXME: If the device is keeping a cached copy of the resource, both the shared copy and cached copy are made purgeable.  Any access to the resource by either the CPU or device will be undefined.
        ///
        /// Availability: macOS 10.11+, iOS 8.0+
        #[unsafe(method(setPurgeableState:))]
        #[unsafe(method_family = none)]
        fn set_purgeable_state(
            &self,
            state: MTLPurgeableState,
        ) -> MTLPurgeableState;

        /// The heap that owns this resource, or `None` when the resource is not
        /// backed by a heap.
        ///
        /// Availability: macOS 10.13+, iOS 10.0+
        #[unsafe(method(heap))]
        #[unsafe(method_family = none)]
        fn heap(&self) -> Option<Retained<ProtocolObject<dyn MTLHeap>>>;

        /// The offset inside the heap at which this resource was created.
        ///
        /// Zero when this resource was not created on a heap with `MTLHeapTypePlacement`.
        ///
        /// Availability: macOS 10.15+, iOS 13.0+
        #[unsafe(method(heapOffset))]
        #[unsafe(method_family = none)]
        fn heap_offset(&self) -> usize;

        /// The size in bytes occupied by this resource
        ///
        /// Availability: macOS 10.13+, iOS 11.0+
        #[unsafe(method(allocatedSize))]
        #[unsafe(method_family = none)]
        fn allocated_size(&self) -> usize;

        /// Allow future heap sub-allocations to alias against this resource's memory.
        ///
        /// It is illegal to call this method on a non heap-based resource.
        /// It is also illegal to call this method on texture views created from heap-based textures.
        /// The debug layer will raise an exception. Calling this method on textures sub-allocated
        /// from Buffers backed by heap memory has no effect.
        /// Once a resource is made aliasable, the decision cannot be reverted.
        ///
        /// Availability: macOS 10.13+, iOS 10.0+
        #[unsafe(method(makeAliasable))]
        #[unsafe(method_family = none)]
        fn make_aliasable(&self);

        /// Returns whether future heap sub-allocations may alias against this resource's memory.
        ///
        /// Returns: YES if `makeAliasable` was previously successfully called on this resource. NO otherwise.
        /// If resource is sub-allocated from other resource created on the heap, isAliasable returns
        /// aliasing state of that base resource. Also returns NO when storage mode is memoryless.
        ///
        /// Availability: macOS 10.13+, iOS 10.0+
        #[unsafe(method(isAliasable))]
        #[unsafe(method_family = none)]
        fn is_aliasable(&self) -> bool;

        /// Assigns ownership of the resource's underlying memory to another task for the purposes of VM accounting.
        ///
        /// This corresponds to `- (kern_return_t)setOwnerWithIdentity:(task_id_token_t)task_id_token`.
        /// The argument is represented as `u32` to match Mach port name width on Apple platforms.
        ///
        /// Availability: macOS 14.4+, iOS 17.4+
        #[unsafe(method(setOwnerWithIdentity:))]
        #[unsafe(method_family = none)]
        fn set_owner_with_identity(
            &self,
            task_id_token: u32,
        ) -> i32;
    }
);

/// Convenience wrappers for the nullable Objective-C label property.
///
/// Availability: macOS 10.11+, iOS 8.0+
pub trait MTLResourceExt: MTLResource + Message {
    fn label(&self) -> Option<String>
    where
        Self: Sized,
    {
        let label: Option<Retained<NSString>> = unsafe { msg_send![self, label] };
        label.map(|label| label.to_string())
    }

    fn set_label(
        &self,
        label: Option<&str>,
    ) where
        Self: Sized,
    {
        unsafe {
            let _: () = msg_send![self, setLabel: label.map(NSString::from_str).as_deref()];
        }
    }
}

impl<T: MTLResource + Message> MTLResourceExt for T {}
