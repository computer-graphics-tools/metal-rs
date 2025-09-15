use core::ptr::NonNull;
use objc2::{extern_protocol, runtime::ProtocolObject};
use objc2_foundation::NSRange;

use super::IntersectionFunctionSignature;
use crate::types::ResourceID;
use crate::{Buffer, FunctionHandle, Resource, VisibleFunctionTable};

extern_protocol!(
    /// Intersection function table
    #[name = "MTLIntersectionFunctionTable"]
    pub unsafe trait IntersectionFunctionTable: Resource {
        #[unsafe(method(setBuffer:offset:atIndex:))]
        #[unsafe(method_family = none)]
        unsafe fn set_buffer(
            &self,
            buffer: Option<&ProtocolObject<dyn Buffer>>,
            offset: usize,
            index: usize,
        );

        /// Safety: `buffers` and `offsets` must be valid pointers.
        #[unsafe(method(setBuffers:offsets:withRange:))]
        #[unsafe(method_family = none)]
        unsafe fn set_buffers(
            &self,
            buffers: NonNull<*const ProtocolObject<dyn Buffer>>,
            offsets: NonNull<usize>,
            range: NSRange,
        );

        /// Handle of the GPU resource suitable for storing in an Argument Buffer
        #[unsafe(method(gpuResourceID))]
        #[unsafe(method_family = none)]
        unsafe fn gpu_resource_id(&self) -> ResourceID;

        #[unsafe(method(setFunction:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_function_at_index(
            &self,
            function: Option<&ProtocolObject<dyn FunctionHandle>>,
            index: usize,
        );

        /// Safety: `functions` must be a valid pointer.
        #[unsafe(method(setFunctions:withRange:))]
        #[unsafe(method_family = none)]
        unsafe fn set_functions_with_range(
            &self,
            functions: NonNull<*const ProtocolObject<dyn FunctionHandle>>,
            range: NSRange,
        );

        #[unsafe(method(setOpaqueTriangleIntersectionFunctionWithSignature:atIndex:))]
        #[unsafe(method_family = none)]
        unsafe fn set_opaque_triangle_intersection_function_with_signature_at_index(
            &self,
            signature: IntersectionFunctionSignature,
            index: usize,
        );

        #[unsafe(method(setOpaqueTriangleIntersectionFunctionWithSignature:withRange:))]
        #[unsafe(method_family = none)]
        unsafe fn set_opaque_triangle_intersection_function_with_signature_with_range(
            &self,
            signature: IntersectionFunctionSignature,
            range: NSRange,
        );

        #[unsafe(method(setOpaqueCurveIntersectionFunctionWithSignature:atIndex:))]
        #[unsafe(method_family = none)]
        unsafe fn set_opaque_curve_intersection_function_with_signature_at_index(
            &self,
            signature: IntersectionFunctionSignature,
            index: usize,
        );

        #[unsafe(method(setOpaqueCurveIntersectionFunctionWithSignature:withRange:))]
        #[unsafe(method_family = none)]
        unsafe fn set_opaque_curve_intersection_function_with_signature_with_range(
            &self,
            signature: IntersectionFunctionSignature,
            range: NSRange,
        );

        #[unsafe(method(setVisibleFunctionTable:atBufferIndex:))]
        #[unsafe(method_family = none)]
        unsafe fn set_visible_function_table_at_buffer_index(
            &self,
            function_table: Option<&ProtocolObject<dyn VisibleFunctionTable>>,
            buffer_index: usize,
        );

        /// Safety: `function_tables` must be a valid pointer.
        #[unsafe(method(setVisibleFunctionTables:withBufferRange:))]
        #[unsafe(method_family = none)]
        unsafe fn set_visible_function_tables_with_buffer_range(
            &self,
            function_tables: NonNull<*const ProtocolObject<dyn VisibleFunctionTable>>,
            buffer_range: NSRange,
        );
    }
);
