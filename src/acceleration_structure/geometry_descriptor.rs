use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::{NSObject, ProtocolObject},
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol, NSString};

use crate::MTLBuffer;

extern_class!(
    /// Base class for all geometry descriptors. Do not use this class directly. Use one of the derived
    /// classes instead.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtlaccelerationstructuregeometrydescriptor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLAccelerationStructureGeometryDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLAccelerationStructureGeometryDescriptor {}
);

unsafe impl CopyingHelper for MTLAccelerationStructureGeometryDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLAccelerationStructureGeometryDescriptor {}
);

impl MTLAccelerationStructureGeometryDescriptor {
    extern_methods!(
        #[unsafe(method(intersectionFunctionTableOffset))]
        #[unsafe(method_family = none)]
        pub unsafe fn intersection_function_table_offset(&self) -> usize;

        /// Setter for [`intersectionFunctionTableOffset`][Self::intersectionFunctionTableOffset].
        #[unsafe(method(setIntersectionFunctionTableOffset:))]
        #[unsafe(method_family = none)]
        pub fn set_intersection_function_table_offset(
            &self,
            intersection_function_table_offset: usize,
        );

        /// Whether the geometry is opaque
        #[unsafe(method(opaque))]
        #[unsafe(method_family = none)]
        pub unsafe fn opaque(&self) -> bool;

        /// Setter for [`opaque`][Self::opaque].
        #[unsafe(method(setOpaque:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_opaque(&self, opaque: bool);

        /// Whether intersection functions may be invoked more than once per ray/primitive
        /// intersection. Defaults to YES.
        #[unsafe(method(allowDuplicateIntersectionFunctionInvocation))]
        #[unsafe(method_family = none)]
        pub unsafe fn allow_duplicate_intersection_function_invocation(&self) -> bool;

        /// Setter for [`allowDuplicateIntersectionFunctionInvocation`][Self::allowDuplicateIntersectionFunctionInvocation].
        #[unsafe(method(setAllowDuplicateIntersectionFunctionInvocation:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_allow_duplicate_intersection_function_invocation(
            &self,
            allow_duplicate_intersection_function_invocation: bool,
        );

        /// Label
        #[unsafe(method(label))]
        #[unsafe(method_family = none)]
        pub unsafe fn label(&self) -> Option<Retained<NSString>>;

        /// Setter for [`label`][Self::label].
        ///
        /// This is [copied][objc2_foundation::NSCopying::copy] when set.
        #[unsafe(method(setLabel:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_label(&self, label: Option<&NSString>);

        /// Data buffer containing per-primitive data. May be nil.
        #[unsafe(method(primitiveDataBuffer))]
        #[unsafe(method_family = none)]
        pub unsafe fn primitive_data_buffer(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn MTLBuffer>>>;

        /// Setter for [`primitiveDataBuffer`][Self::primitiveDataBuffer].
        #[unsafe(method(setPrimitiveDataBuffer:))]
        #[unsafe(method_family = none)]
        pub fn set_primitive_data_buffer(
            &self,
            primitive_data_buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
        );

        /// Primitive data buffer offset in bytes. Must be aligned to the platform's buffer offset alignment. Defaults to 0 bytes.
        #[unsafe(method(primitiveDataBufferOffset))]
        #[unsafe(method_family = none)]
        pub unsafe fn primitive_data_buffer_offset(&self) -> usize;

        /// Setter for [`primitiveDataBufferOffset`][Self::primitiveDataBufferOffset].
        #[unsafe(method(setPrimitiveDataBufferOffset:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_primitive_data_buffer_offset(&self, primitive_data_buffer_offset: usize);

        /// Stride, in bytes, between per-primitive data in the primitive data buffer. Must be at least primitiveDataElementSize and must be a
        /// multiple of 4 bytes. Defaults to 0 bytes. Assumed to be equal to primitiveDataElementSize if zero.
        #[unsafe(method(primitiveDataStride))]
        #[unsafe(method_family = none)]
        pub unsafe fn primitive_data_stride(&self) -> usize;

        /// Setter for [`primitiveDataStride`][Self::primitiveDataStride].
        #[unsafe(method(setPrimitiveDataStride:))]
        #[unsafe(method_family = none)]
        pub fn set_primitive_data_stride(&self, primitive_data_stride: usize);

        /// Size, in bytes, of the data for each primitive in the primitive data buffer. Must be at most primitiveDataStride and must be a
        /// multiple of 4 bytes. Defaults to 0 bytes.
        #[unsafe(method(primitiveDataElementSize))]
        #[unsafe(method_family = none)]
        pub unsafe fn primitive_data_element_size(&self) -> usize;

        /// Setter for [`primitiveDataElementSize`][Self::primitiveDataElementSize].
        #[unsafe(method(setPrimitiveDataElementSize:))]
        #[unsafe(method_family = none)]
        pub fn set_primitive_data_element_size(&self, primitive_data_element_size: usize);
    );
}

/// Methods declared on superclass `NSObject`.
impl MTLAccelerationStructureGeometryDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
