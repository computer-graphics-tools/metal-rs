use core::ptr::NonNull;
use objc2::{extern_protocol, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSObjectProtocol, NSString};

use crate::{MTLAllocation, MTLDevice};

extern_protocol!(
    /// A residency set manages resource and heap residency, referenced by a command buffer or queue.
    ///
    /// Apple's documentation: `https://developer.apple.com/documentation/metal/mtlresidencyset?language=objc`
    pub unsafe trait MTLResidencySet: NSObjectProtocol + Send + Sync {
        /// The device that created the residency set
        #[unsafe(method(device))]
        #[unsafe(method_family = none)]
        fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;

        /// The label specified at creation.
        #[unsafe(method(label))]
        #[unsafe(method_family = none)]
        fn label(&self) -> Option<Retained<NSString>>;

        /// The memory footprint of the set in bytes at the last commit operation.
        #[unsafe(method(allocatedSize))]
        #[unsafe(method_family = none)]
        fn allocated_size(&self) -> u64;

        /// Requests that the set and all the committed resources and heaps are made resident.
        #[unsafe(method(requestResidency))]
        #[unsafe(method_family = none)]
        fn request_residency(&self);

        /// Requests that the set and all the committed resources and heaps are made non-resident.
        #[unsafe(method(endResidency))]
        #[unsafe(method_family = none)]
        fn end_residency(&self);

        /// Adds one allocation to the set, leaving it uncommitted until commit is called.
        #[unsafe(method(addAllocation:))]
        #[unsafe(method_family = none)]
        fn add_allocation(&self, allocation: &ProtocolObject<dyn MTLAllocation>);

        /// Adds allocations to the set, leaving them uncommitted until commit is called.
        ///
        /// Safety: `allocations` must be a valid pointer.
        #[unsafe(method(addAllocations:count:))]
        #[unsafe(method_family = none)]
        unsafe fn add_allocations(
            &self,
            allocations: NonNull<NonNull<ProtocolObject<dyn MTLAllocation>>>,
            count: usize,
        );

        /// Marks an allocation to be removed from the set on the next commit call.
        #[unsafe(method(removeAllocation:))]
        #[unsafe(method_family = none)]
        fn remove_allocation(&self, allocation: &ProtocolObject<dyn MTLAllocation>);

        /// Marks allocations to be removed from the set on the next commit call.
        ///
        /// Safety: `allocations` must be a valid pointer.
        #[unsafe(method(removeAllocations:count:))]
        #[unsafe(method_family = none)]
        unsafe fn remove_allocations(
            &self,
            allocations: NonNull<NonNull<ProtocolObject<dyn MTLAllocation>>>,
            count: usize,
        );

        /// Marks all allocations to be removed from the set on the next commit call.
        #[unsafe(method(removeAllAllocations))]
        #[unsafe(method_family = none)]
        fn remove_all_allocations(&self);

        /// Returns a boolean indicating whether the allocation is present in the set or not.
        /// This check includes non-committed allocations in the set.
        #[unsafe(method(containsAllocation:))]
        #[unsafe(method_family = none)]
        fn contains_allocation(&self, allocation: &ProtocolObject<dyn MTLAllocation>) -> bool;

        /// Array of all allocations associated with the set.
        /// This property includes non-committed allocations in the set.
        #[unsafe(method(allAllocations))]
        #[unsafe(method_family = none)]
        fn all_allocations(
            &self,
        ) -> Retained<objc2_foundation::NSArray<ProtocolObject<dyn MTLAllocation>>>;

        /// Returns the current number of unique allocations present in the set.
        /// This property includes non-committed allocations in the set.
        #[unsafe(method(allocationCount))]
        #[unsafe(method_family = none)]
        fn allocation_count(&self) -> usize;

        /// Commits any pending adds/removes.
        #[unsafe(method(commit))]
        #[unsafe(method_family = none)]
        fn commit(&self);
    }
);
