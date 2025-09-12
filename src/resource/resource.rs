use objc2::{msg_send, extern_protocol, rc::Retained, Message, runtime::ProtocolObject};
use objc2_foundation::NSString;

use crate::device::Device;
use crate::allocation::Allocation;

use super::{
    CpuCacheMode,
    StorageMode,
    HazardTrackingMode,
    ResourceOptions,
    PurgeableState,
};

extern_protocol!(
    /// Common APIs available for MTLBuffer and MTLTexture instances
    ///
    /// See also Apple's documentation: `https://developer.apple.com/documentation/metal/mtlresource?language=objc`
    #[name = "MTLResource"]
    pub unsafe trait Resource: Allocation {
        /// The device this resource was created against.  This resource can only be used with this device.
        #[unsafe(method(device))]
        #[unsafe(method_family = none)]
        fn device(&self) -> Retained<ProtocolObject<dyn Device>>;

        /// The cache mode used for the CPU mapping for this resource
        #[unsafe(method(cpuCacheMode))]
        #[unsafe(method_family = none)]
        fn cpu_cache_mode(&self) -> CpuCacheMode;

        /// The resource storage mode used for the CPU mapping for this resource
        #[unsafe(method(storageMode))]
        #[unsafe(method_family = none)]
        fn storage_mode(&self) -> StorageMode;

        /// Whether or not the resource is hazard tracked.
        ///
        /// This value can be either `MTLHazardTrackingModeUntracked` or `MTLHazardTrackingModeTracked`.
        /// Resources created from heaps are by default untracked, whereas resources created from the device are by default tracked.
        #[unsafe(method(hazardTrackingMode))]
        #[unsafe(method_family = none)]
        fn hazard_tracking_mode(&self) -> HazardTrackingMode;

        /// A packed tuple of the `storageMode`, `cpuCacheMode` and `hazardTrackingMode` properties.
        #[unsafe(method(resourceOptions))]
        #[unsafe(method_family = none)]
        fn resource_options(&self) -> ResourceOptions;

        /// Set (or query) the purgeability state of a resource
        ///
        /// Synchronously set the purgeability state of a resource and return what the prior (or current) state is.
        /// FIXME: If the device is keeping a cached copy of the resource, both the shared copy and cached copy are made purgeable.  Any access to the resource by either the CPU or device will be undefined.
        #[unsafe(method(setPurgeableState:))]
        #[unsafe(method_family = none)]
        fn set_purgeable_state(&self, state: PurgeableState) -> PurgeableState;

        /// The offset inside the heap at which this resource was created.
        ///
        /// Zero when this resource was not created on a heap with `MTLHeapTypePlacement`.
        #[unsafe(method(heapOffset))]
        #[unsafe(method_family = none)]
        fn heap_offset(&self) -> usize;

        /// The size in bytes occupied by this resource
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
        #[unsafe(method(makeAliasable))]
        #[unsafe(method_family = none)]
        fn make_aliasable(&self);

        /// Returns whether future heap sub-allocations may alias against this resource's memory.
        ///
        /// Returns: YES if `makeAliasable` was previously successfully called on this resource. NO otherwise.
        /// If resource is sub-allocated from other resource created on the heap, isAliasable returns
        /// aliasing state of that base resource. Also returns NO when storage mode is memoryless.
        #[unsafe(method(isAliasable))]
        #[unsafe(method_family = none)]
        fn is_aliasable(&self) -> bool;

        /// Assigns ownership of the resource's underlying memory to another task for the purposes of VM accounting.
        ///
        /// This corresponds to `- (kern_return_t)setOwnerWithIdentity:(task_id_token_t)task_id_token`.
        /// The argument is represented as `u32` to match Mach port name width on Apple platforms.
        #[unsafe(method(setOwnerWithIdentity:))]
        #[unsafe(method_family = none)]
        fn set_owner_with_identity(&self, task_id_token: u32) -> i32;
    }
);

pub trait ResourceExt: Resource + Message {
    fn label(&self) -> Option<String>;
    fn set_label(&self, label: Option<&str>);
}

impl ResourceExt for ProtocolObject<dyn Resource> {
    fn label(&self) -> Option<String> {
        let label: Option<Retained<NSString>> = unsafe {
            msg_send![self, label]
        };
        label.map(|label| label.to_string())
    }
    fn set_label(&self, label: Option<&str>) {
        unsafe {
            let _: () = msg_send![
                self, 
                setLabel: label.map(NSString::from_str).as_deref()
            ];
        }
    }
}
