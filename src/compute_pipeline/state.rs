use objc2::{extern_protocol, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSObjectProtocol, NSString};

use crate::intersection_function_table::MTLIntersectionFunctionTableDescriptor;
use crate::{
    MTLDevice, MTLResourceID,
    visible_function_table::{MTLVisibleFunctionTable, MTLVisibleFunctionTableDescriptor},
};

extern_protocol!(
    /// A handle to compiled code for a compute function.
    pub unsafe trait MTLComputePipelineState: NSObjectProtocol + Send + Sync {
        /// A string to help identify this object.
        #[unsafe(method(label))]
        #[unsafe(method_family = none)]
        fn label(&self) -> Option<Retained<NSString>>;

        /// The device this resource was created against.
        #[unsafe(method(device))]
        #[unsafe(method_family = none)]
        fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;

        /// The maximum total number of threads that can be in a single compute threadgroup.
        #[unsafe(method(maxTotalThreadsPerThreadgroup))]
        #[unsafe(method_family = none)]
        fn max_total_threads_per_threadgroup(&self) -> usize;

        /// For most efficient execution, the threadgroup size should be a multiple of this.
        #[unsafe(method(threadExecutionWidth))]
        #[unsafe(method_family = none)]
        fn thread_execution_width(&self) -> usize;

        /// The length in bytes of threadgroup memory that is statically allocated.
        #[unsafe(method(staticThreadgroupMemoryLength))]
        #[unsafe(method_family = none)]
        fn static_threadgroup_memory_length(&self) -> usize;

        /// Handle of the GPU resource suitable for storing in an Argument Buffer
        #[unsafe(method(gpuResourceID))]
        #[unsafe(method_family = none)]
        unsafe fn gpu_resource_id(&self) -> MTLResourceID;

        /// Allocate a visible function table for the pipeline with the provided descriptor.
        #[unsafe(method(newVisibleFunctionTableWithDescriptor:))]
        #[unsafe(method_family = new)]
        fn new_visible_function_table_with_descriptor(
            &self,
            descriptor: &MTLVisibleFunctionTableDescriptor,
        ) -> Option<Retained<ProtocolObject<dyn MTLVisibleFunctionTable>>>;

        /// Allocate an intersection function table for the pipeline with the provided descriptor.
        #[unsafe(method(newIntersectionFunctionTableWithDescriptor:))]
        #[unsafe(method_family = new)]
        fn new_intersection_function_table_with_descriptor(
            &self,
            descriptor: &MTLIntersectionFunctionTableDescriptor,
        ) -> Option<Retained<ProtocolObject<dyn crate::MTLIntersectionFunctionTable>>>;
    }
);
