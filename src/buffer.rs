use std::{ops::Range, os::raw::c_void, ptr::NonNull};

use objc2::{Message, extern_protocol, msg_send, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSError, NSRange, NSString};

#[cfg(target_os = "macos")]
use crate::MTLDevice;
use crate::{
    MTLBufferSparseTier, MTLResource, MTLTensor, MTLTexture, MTLTextureDescriptor, MetalError,
    tensor::MTLTensorDescriptor,
};

extern_protocol!(
    /// A typeless allocation accessible by both the CPU and the GPU (MTLDevice) or by only the GPU when the storage mode is
    /// MTLResourceStorageModePrivate.
    ///
    ///
    /// Unlike in OpenGL and OpenCL, access to buffers is not synchronized.  The caller may use the CPU to modify the data at any time
    /// but is also responsible for ensuring synchronization and coherency.
    ///
    /// The contents become undefined if both the CPU and GPU write to the same buffer without a synchronizing action between those writes.
    /// This is true even when the regions written do not overlap.
    ///
    /// Availability: macOS 10.11+, iOS 8.0+
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtlbuffer?language=objc)
    pub unsafe trait MTLBuffer: MTLResource {
        /// The length of the buffer in bytes.
        #[unsafe(method(length))]
        #[unsafe(method_family = none)]
        fn length(&self) -> usize;

        /// Returns the data pointer of this buffer's shared copy.
        ///
        /// The buffer owns this pointer; callers must not deallocate it. The
        /// pointer is usable only while the buffer remains alive and its
        /// storage mode permits CPU access. Dereferencing it requires the
        /// caller to uphold Metal's CPU/GPU synchronization rules.
        #[unsafe(method(contents))]
        #[unsafe(method_family = none)]
        fn contents(&self) -> NonNull<c_void>;

        /// Create a 2D texture or texture buffer that shares storage with this buffer.
        #[unsafe(method(newTextureWithDescriptor:offset:bytesPerRow:))]
        #[unsafe(method_family = new)]
        fn new_texture(
            &self,
            descriptor: &MTLTextureDescriptor,
            offset: usize,
            bytes_per_row: usize,
        ) -> Option<Retained<ProtocolObject<dyn MTLTexture>>>;

        /// Removes all debug markers from a buffer.
        #[unsafe(method(removeAllDebugMarkers))]
        #[unsafe(method_family = none)]
        fn remove_all_debug_markers(&self);

        /// For Metal buffer objects that are remote views, this returns the buffer associated with the storage on the originating device.
        #[cfg(target_os = "macos")]
        #[deprecated(note = "remote buffer views do not apply to Apple silicon")]
        #[unsafe(method(remoteStorageBuffer))]
        #[unsafe(method_family = none)]
        fn remote_storage_buffer(&self) -> Option<Retained<ProtocolObject<dyn MTLBuffer>>>;

        /// On Metal devices that support peer to peer transfers, this method is used to create a remote buffer view on another device
        /// within the peer group.  The receiver must use MTLStorageModePrivate or be backed by an IOSurface.
        #[cfg(target_os = "macos")]
        #[deprecated(note = "remote buffer views do not apply to Apple silicon")]
        #[unsafe(method(newRemoteBufferViewForDevice:))]
        #[unsafe(method_family = new)]
        fn new_remote_buffer_view_for_device(
            &self,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Option<Retained<ProtocolObject<dyn MTLBuffer>>>;

        /// Represents the GPU virtual address of a buffer resource
        #[unsafe(method(gpuAddress))]
        #[unsafe(method_family = none)]
        fn gpu_address(&self) -> u64;

        /// Query support tier for sparse buffers.
        #[unsafe(method(sparseBufferTier))]
        #[unsafe(method_family = none)]
        fn sparse_buffer_tier(&self) -> MTLBufferSparseTier;
    }
);

pub trait BufferExt: MTLBuffer + Message {
    /// Creates a single-plane tensor that shares storage with this buffer.
    ///
    /// Metal validates `descriptor`. `offset` must be zero for
    /// machine-learning tensors, 128-byte aligned for formatted tensor data,
    /// and otherwise aligned to the data type's size.
    fn new_tensor_with_descriptor_offset(
        &self,
        descriptor: &MTLTensorDescriptor,
        offset: usize,
    ) -> Result<Retained<ProtocolObject<dyn MTLTensor>>, MetalError>;

    /// Inform the device of the range of a buffer that the CPU has modified, allowing the implementation to invalidate
    /// its caches of the buffer's content.
    ///
    /// When the application writes to a buffer's sysmem copy via
    /// _contents,_that range of the buffer immediately
    /// becomes undefined for any accesses by the GPU (MTLDevice).  To restore coherency, the buffer modification must be followed
    /// by -didModifyRange:, and then followed by a commit of the MTLCommandBuffer that will access the buffer.
    /// -didModifyRange does not make the contents coherent for any previously committed command buffers.
    /// Note: This method is only required if buffer is created with a storage mode of MTLResourceStorageModeManaged.
    /// It is not valid to invoke this method on buffers of other storage modes.
    ///
    /// Parameter `range`: The range of bytes that have been modified.
    ///
    /// # Panics
    ///
    /// Panics if `range.start` exceeds `range.end`.
    #[cfg(any(target_os = "macos", target_abi = "macabi"))]
    #[deprecated(note = "managed storage has no effect on Apple silicon; use shared storage")]
    fn did_modify_range(
        &self,
        range: Range<usize>,
    );

    /// Adds a marker to a specific range in the buffer.
    /// When inspecting a buffer in the GPU debugging tools the marker will be shown.
    ///
    /// Parameter `marker`: A label used for the marker.
    ///
    /// Parameter `range`: The range of bytes the marker is using.
    ///
    /// # Panics
    ///
    /// Panics if `range.start` exceeds `range.end`.
    fn add_debug_marker(
        &self,
        marker: &str,
        range: Range<usize>,
    );
}

impl BufferExt for ProtocolObject<dyn MTLBuffer> {
    fn new_tensor_with_descriptor_offset(
        &self,
        descriptor: &MTLTensorDescriptor,
        offset: usize,
    ) -> Result<Retained<ProtocolObject<dyn MTLTensor>>, MetalError> {
        let mut error: *mut NSError = std::ptr::null_mut();
        let tensor = unsafe { msg_send![self, newTensorWithDescriptor: descriptor, offset: offset, error: &mut error] };
        unsafe { MetalError::result_from_nullable(tensor, error, "newTensorWithDescriptor:offset:error:") }
    }

    /// Inform the device of the range of a buffer that the CPU has modified, allowing the implementation to invalidate
    /// its caches of the buffer's content.
    ///
    /// When the application writes to a buffer's sysmem copy via `contents`, that range of the buffer immediately
    /// becomes undefined for any accesses by the GPU (MTLDevice). To restore coherency, the buffer modification must be followed
    /// by `didModifyRange:`, and then followed by a commit of the `MTLCommandBuffer` that will access the buffer.
    /// `didModifyRange:` does not make the contents coherent for any previously committed command buffers.
    ///
    /// Note: This method is only required if buffer is created with a storage mode of `MTLResourceStorageModeManaged`.
    /// It is not valid to invoke this method on buffers of other storage modes.
    ///
    /// Availability: macOS 10.11+, Mac Catalyst 13.0+ (unavailable on iOS)
    #[cfg(any(target_os = "macos", target_abi = "macabi"))]
    fn did_modify_range(
        &self,
        range: Range<usize>,
    ) {
        let range = NSRange::from(range);
        let _: () = unsafe {
            msg_send![
                self,
                didModifyRange: range,
            ]
        };
    }

    /// Adds a marker to a specific range in the buffer. When inspecting a buffer in GPU debugging tools, the marker will be shown.
    ///
    /// Parameter `marker`: A label used for the marker.
    /// Parameter `range`: The range of bytes the marker is using.
    ///
    /// Availability: macOS 10.12+, iOS 10.0+
    fn add_debug_marker(
        &self,
        marker: &str,
        range: Range<usize>,
    ) {
        let range = NSRange::from(range);
        let _: () = unsafe {
            msg_send![
                self,
                addDebugMarker: &*NSString::from_str(marker),
                range: range,
            ]
        };
    }
}
