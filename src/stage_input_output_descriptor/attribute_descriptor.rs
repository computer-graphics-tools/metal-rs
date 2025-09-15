use objc2::{extern_class, extern_conformance, extern_methods, runtime::NSObject};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

use super::AttributeFormat;

extern_class!(
    /// Attribute descriptor
    #[unsafe(super(NSObject))]
    #[name = "MTLAttributeDescriptor"]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AttributeDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for AttributeDescriptor {}
);

unsafe impl CopyingHelper for AttributeDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for AttributeDescriptor {}
);

impl AttributeDescriptor {
    extern_methods!(
        /// The attribute data format.
        #[unsafe(method(format))]
        #[unsafe(method_family = none)]
        pub fn format(&self) -> AttributeFormat;

        /// Setter for [`format`][Self::format].
        #[unsafe(method(setFormat:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_format(&self, format: AttributeFormat);

        /// Byte offset of this attribute within the vertex.
        #[unsafe(method(offset))]
        #[unsafe(method_family = none)]
        pub fn offset(&self) -> usize;

        /// Setter for [`offset`][Self::offset].
        #[unsafe(method(setOffset:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_offset(&self, offset: usize);

        /// The index of the buffer from which this attribute reads.
        #[unsafe(method(bufferIndex))]
        #[unsafe(method_family = none)]
        pub fn buffer_index(&self) -> usize;

        /// Setter for [`buffer_index`][Self::buffer_index].
        #[unsafe(method(setBufferIndex:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_buffer_index(&self, buffer_index: usize);
    );
}
