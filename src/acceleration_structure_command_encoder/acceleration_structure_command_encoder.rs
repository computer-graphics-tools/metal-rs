use core::ptr::NonNull;
use objc2::{extern_protocol, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSObjectProtocol, NSRange};

use crate::{
    AccelerationStructure, Buffer, CommandEncoder, CounterSampleBuffer, DataType, Fence, Heap,
    Resource, ResourceUsage,
};

extern_protocol!(
    /// Command encoder for building and managing acceleration structures.
    #[name = "MTLAccelerationStructureCommandEncoder"]
    pub unsafe trait AccelerationStructureCommandEncoder: CommandEncoder {
        /// Encode an acceleration structure build.
        #[unsafe(method(buildAccelerationStructure:descriptor:scratchBuffer:scratchBufferOffset:))]
        #[unsafe(method_family = none)]
        fn build_acceleration_structure(
            &self,
            acceleration_structure: &ProtocolObject<dyn AccelerationStructure>,
            descriptor: &crate::acceleration::AccelerationStructureDescriptor,
            scratch_buffer: &ProtocolObject<dyn Buffer>,
            scratch_buffer_offset: usize,
        );

        /// Encode an acceleration structure refit (basic).
        #[unsafe(method(refitAccelerationStructure:descriptor:destination:scratchBuffer:scratchBufferOffset:))]
        #[unsafe(method_family = none)]
        unsafe fn refit_acceleration_structure(
            &self,
            source_acceleration_structure: &ProtocolObject<dyn AccelerationStructure>,
            descriptor: &crate::acceleration::AccelerationStructureDescriptor,
            destination_acceleration_structure: Option<&ProtocolObject<dyn AccelerationStructure>>,
            scratch_buffer: Option<&ProtocolObject<dyn Buffer>>,
            scratch_buffer_offset: usize,
        );

        /// Encode an acceleration structure refit with options.
        #[unsafe(method(refitAccelerationStructure:descriptor:destination:scratchBuffer:scratchBufferOffset:options:))]
        #[unsafe(method_family = none)]
        unsafe fn refit_acceleration_structure_with_options(
            &self,
            source_acceleration_structure: &ProtocolObject<dyn AccelerationStructure>,
            descriptor: &crate::acceleration::AccelerationStructureDescriptor,
            destination_acceleration_structure: Option<&ProtocolObject<dyn AccelerationStructure>>,
            scratch_buffer: Option<&ProtocolObject<dyn Buffer>>,
            scratch_buffer_offset: usize,
            options: crate::acceleration::AccelerationStructureRefitOptions,
        );

        /// Copy an acceleration structure.
        #[unsafe(method(copyAccelerationStructure:toAccelerationStructure:))]
        #[unsafe(method_family = none)]
        unsafe fn copy_acceleration_structure(
            &self,
            source_acceleration_structure: &ProtocolObject<dyn AccelerationStructure>,
            destination_acceleration_structure: &ProtocolObject<dyn AccelerationStructure>,
        );

        /// Write compacted acceleration structure size to a buffer as u32.
        #[unsafe(method(writeCompactedAccelerationStructureSize:toBuffer:offset:))]
        #[unsafe(method_family = none)]
        fn write_compacted_acceleration_structure_size(
            &self,
            acceleration_structure: &ProtocolObject<dyn AccelerationStructure>,
            buffer: &ProtocolObject<dyn Buffer>,
            offset: usize,
        );

        /// Write compacted acceleration structure size to a buffer with size data type.
        #[unsafe(method(writeCompactedAccelerationStructureSize:toBuffer:offset:sizeDataType:))]
        #[unsafe(method_family = none)]
        unsafe fn write_compacted_acceleration_structure_size_with_type(
            &self,
            acceleration_structure: &ProtocolObject<dyn AccelerationStructure>,
            buffer: &ProtocolObject<dyn Buffer>,
            offset: usize,
            size_data_type: DataType,
        );

        /// Copy and compact an acceleration structure.
        #[unsafe(method(copyAndCompactAccelerationStructure:toAccelerationStructure:))]
        #[unsafe(method_family = none)]
        fn copy_and_compact_acceleration_structure(
            &self,
            source_acceleration_structure: &ProtocolObject<dyn AccelerationStructure>,
            destination_acceleration_structure: &ProtocolObject<dyn AccelerationStructure>,
        );

        /// Update the fence to capture all GPU work so far enqueued by this encoder.
        #[unsafe(method(updateFence:))]
        #[unsafe(method_family = none)]
        fn update_fence(&self, fence: &ProtocolObject<dyn Fence>);

        /// Prevent further GPU work until the fence is reached.
        #[unsafe(method(waitForFence:))]
        #[unsafe(method_family = none)]
        fn wait_for_fence(&self, fence: &ProtocolObject<dyn Fence>);

        /// Declare that a resource may be accessed through an argument buffer by the encoder.
        #[unsafe(method(useResource:usage:))]
        #[unsafe(method_family = none)]
        fn use_resource(&self, resource: &ProtocolObject<dyn Resource>, usage: ResourceUsage);

        /// Declare that an array of resources may be accessed through an argument buffer by the encoder.
        /// Safety: `resources` must be valid.
        #[unsafe(method(useResources:count:usage:))]
        #[unsafe(method_family = none)]
        unsafe fn use_resources(
            &self,
            resources: NonNull<NonNull<ProtocolObject<dyn Resource>>>,
            count: usize,
            usage: ResourceUsage,
        );

        /// Declare that the resources allocated from a heap may be accessed as readonly.
        #[unsafe(method(useHeap:))]
        #[unsafe(method_family = none)]
        fn use_heap(&self, heap: &ProtocolObject<dyn Heap>);

        /// Declare that the resources allocated from an array of heaps may be accessed as readonly.
        /// Safety: `heaps` must be valid.
        #[unsafe(method(useHeaps:count:))]
        #[unsafe(method_family = none)]
        unsafe fn use_heaps(&self, heaps: NonNull<NonNull<ProtocolObject<dyn Heap>>>, count: usize);

        /// Sample hardware counters at this point in the encoder.
        #[unsafe(method(sampleCountersInBuffer:atSampleIndex:withBarrier:))]
        #[unsafe(method_family = none)]
        unsafe fn sample_counters_in_buffer(
            &self,
            sample_buffer: &ProtocolObject<dyn CounterSampleBuffer>,
            sample_index: usize,
            barrier: bool,
        );
    }
);
