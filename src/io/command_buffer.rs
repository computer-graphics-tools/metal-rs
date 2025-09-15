use core::ffi::c_void;
use core::ptr::NonNull;

use objc2::{extern_protocol, rc::Retained, runtime::ProtocolObject, Encode, Encoding, RefEncode};
use objc2_foundation::{NSError, NSObjectProtocol, NSString, NSUInteger};

use crate::{Buffer, Texture};
use crate::types::{Origin, Size};

/// Status of an IO command buffer (ported from `MTLIOStatus`).
#[repr(i64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum IoStatus {
    Pending = 0,
    Cancelled = 1,
    Error = 2,
    Complete = 3,
}

unsafe impl Encode for IoStatus {
    const ENCODING: Encoding = i64::ENCODING;
}

unsafe impl RefEncode for IoStatus {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    /// Represents a list of IO commands for a queue to execute.
    #[name = "MTLIOCommandBuffer"]
    pub unsafe trait IoCommandBuffer: NSObjectProtocol {
        /// Encodes a command that loads from a handle and offset into a memory location.
        ///
        /// Safety: `pointer` must be valid for writes of `size` bytes.
        #[unsafe(method(loadBytes:size:sourceHandle:sourceHandleOffset:))]
        #[unsafe(method_family = none)]
        unsafe fn load_bytes_size_source_handle_source_handle_offset(
            &self,
            pointer: NonNull<c_void>,
            size: NSUInteger,
            source_handle: &ProtocolObject<dyn crate::io::IoFileHandle>,
            source_handle_offset: NSUInteger,
        );

        /// Encodes a command that loads from a handle and offset into a buffer and an offset.
        #[unsafe(method(loadBuffer:offset:size:sourceHandle:sourceHandleOffset:))]
        #[unsafe(method_family = none)]
        unsafe fn load_buffer_offset_size_source_handle_source_handle_offset(
            &self,
            buffer: &ProtocolObject<dyn Buffer>,
            offset: NSUInteger,
            size: NSUInteger,
            source_handle: &ProtocolObject<dyn crate::io::IoFileHandle>,
            source_handle_offset: NSUInteger,
        );

        /// Encodes a command that loads a region from a handle into a texture.
        #[unsafe(method(loadTexture:slice:level:size:sourceBytesPerRow:sourceBytesPerImage:destinationOrigin:sourceHandle:sourceHandleOffset:))]
        #[unsafe(method_family = none)]
        unsafe fn load_texture_slice_level_size_source_bytes_per_row_source_bytes_per_image_destination_origin_source_handle_source_handle_offset(
            &self,
            texture: &ProtocolObject<dyn Texture>,
            slice: NSUInteger,
            level: NSUInteger,
            size: Size,
            source_bytes_per_row: NSUInteger,
            source_bytes_per_image: NSUInteger,
            destination_origin: Origin,
            source_handle: &ProtocolObject<dyn crate::io::IoFileHandle>,
            source_handle_offset: NSUInteger,
        );

        /// Encodes a command that writes the status of this command buffer upon completion to a buffer at a given offset.
        #[unsafe(method(copyStatusToBuffer:offset:))]
        #[unsafe(method_family = none)]
        unsafe fn copy_status_to_buffer_offset(&self, buffer: &ProtocolObject<dyn Buffer>, offset: NSUInteger);

        /// Commit so it can be executed as soon as possible.
        #[unsafe(method(commit))]
        #[unsafe(method_family = none)]
        unsafe fn commit(&self);

        /// Synchronously wait for completion.
        #[unsafe(method(waitUntilCompleted))]
        #[unsafe(method_family = none)]
        unsafe fn wait_until_completed(&self);

        /// Request cancellation of an in-flight command buffer.
        #[unsafe(method(tryCancel))]
        #[unsafe(method_family = none)]
        unsafe fn try_cancel(&self);

        /// Add a barrier to order previously encoded commands before subsequent ones start.
        #[unsafe(method(addBarrier))]
        #[unsafe(method_family = none)]
        unsafe fn add_barrier(&self);

        /// Optional label.
        #[unsafe(method(label))]
        #[unsafe(method_family = none)]
        unsafe fn label(&self) -> Option<Retained<NSString>>;

        /// Setter for label.
        #[unsafe(method(setLabel:))]
        #[unsafe(method_family = none)]
        unsafe fn set_label(&self, label: Option<&NSString>);

        /// Completion status of the command buffer.
        #[unsafe(method(status))]
        #[unsafe(method_family = none)]
        unsafe fn status(&self) -> IoStatus;

        /// If an error occurred during execution, the NSError may contain more details.
        #[unsafe(method(error))]
        #[unsafe(method_family = none)]
        unsafe fn error(&self) -> Option<Retained<NSError>>;
    }
);


