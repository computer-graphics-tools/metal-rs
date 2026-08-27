use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::{NSObject, ProtocolObject},
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

use super::MTLTensorPlaneType;
use crate::MTLBuffer;

extern_class!(
    /// Buffer and byte-offset attachments for the planes of a buffer-backed tensor.
    ///
    /// Availability: macOS 27.0+, iOS 27.0+
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLTensorBufferAttachments;
);

extern_conformance!(
    unsafe impl NSCopying for MTLTensorBufferAttachments {}
);

unsafe impl CopyingHelper for MTLTensorBufferAttachments {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLTensorBufferAttachments {}
);

impl MTLTensorBufferAttachments {
    extern_methods!(
        /// Associates a plane with a backing buffer and byte offset.
        #[unsafe(method(setBuffer:offset:forPlane:))]
        #[unsafe(method_family = none)]
        pub fn set_buffer_offset_for_plane(
            &self,
            buffer: &ProtocolObject<dyn MTLBuffer>,
            offset: usize,
            plane: MTLTensorPlaneType,
        );

        /// Returns the buffer backing a plane, or `None` if none has been set.
        #[unsafe(method(bufferForPlane:))]
        #[unsafe(method_family = none)]
        pub fn buffer_for_plane(
            &self,
            plane: MTLTensorPlaneType,
        ) -> Option<Retained<ProtocolObject<dyn MTLBuffer>>>;

        /// Returns the byte offset into the buffer for a plane.
        #[unsafe(method(offsetForPlane:))]
        #[unsafe(method_family = none)]
        pub fn offset_for_plane(
            &self,
            plane: MTLTensorPlaneType,
        ) -> usize;

        /// Removes every buffer attachment.
        #[unsafe(method(reset))]
        #[unsafe(method_family = none)]
        pub fn reset(&self);
    );
}

impl MTLTensorBufferAttachments {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
