use core::ops::Range;

use objc2::{Message, extern_protocol, msg_send, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::NSRange;

use crate::{MTLIndirectComputeCommand, MTLIndirectRenderCommand, MTLResource, types::MTLResourceID};

extern_protocol!(
    /// Bridged protocol for `MTLIndirectCommandBuffer`.
    ///
    /// Availability: macOS 10.14+, iOS 12.0+
    pub unsafe trait MTLIndirectCommandBuffer: MTLResource {
        #[unsafe(method(size))]
        #[unsafe(method_family = none)]
        fn size(&self) -> usize;

        /// Handle of the GPU resource suitable for storing in an Argument Buffer.
        ///
        /// Availability: macOS 13.0+, iOS 16.0+
        #[unsafe(method(gpuResourceID))]
        #[unsafe(method_family = none)]
        fn gpu_resource_id(&self) -> MTLResourceID;

        /// Returns the render command at `command_index`.
        ///
        /// `command_index` needs to be less than [`size`](Self::size).
        #[unsafe(method(indirectRenderCommandAtIndex:))]
        #[unsafe(method_family = none)]
        fn indirect_render_command_at_index(
            &self,
            command_index: usize,
        ) -> Retained<ProtocolObject<dyn MTLIndirectRenderCommand>>;

        /// Returns the compute command at `command_index`.
        ///
        /// `command_index` needs to be less than [`size`](Self::size).
        #[unsafe(method(indirectComputeCommandAtIndex:))]
        #[unsafe(method_family = none)]
        fn indirect_compute_command_at_index(
            &self,
            command_index: usize,
        ) -> Retained<ProtocolObject<dyn MTLIndirectComputeCommand>>;
    }
);

pub trait MTLIndirectCommandBufferExt: MTLIndirectCommandBuffer + Message {
    /// Resets commands in `range`.
    ///
    /// `range` needs to lie within the command buffer's
    /// [`size`](MTLIndirectCommandBuffer::size).
    fn reset_with_range(
        &self,
        range: Range<usize>,
    ) where
        Self: Sized,
    {
        let range = NSRange::from(range);
        unsafe {
            let _: () = msg_send![self, resetWithRange: range];
        }
    }
}

impl<T: MTLIndirectCommandBuffer + Message> MTLIndirectCommandBufferExt for T {}
