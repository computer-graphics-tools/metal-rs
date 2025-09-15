use objc2::{extern_protocol, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSObjectProtocol, NSRange};

use crate::IndirectComputeCommand;
use crate::IndirectRenderCommand;
use crate::types::ResourceID;

extern_protocol!(
    /// Bridged protocol for `MTLIndirectCommandBuffer`.
    #[name = "MTLIndirectCommandBuffer"]
    pub unsafe trait IndirectCommandBuffer: NSObjectProtocol {
        #[unsafe(method(size))]
        #[unsafe(method_family = none)]
        fn size(&self) -> usize;

        /// Handle of the GPU resource suitable for storing in an Argument Buffer.
        #[unsafe(method(gpuResourceID))]
        #[unsafe(method_family = none)]
        unsafe fn gpu_resource_id(&self) -> ResourceID;

        #[unsafe(method(resetWithRange:))]
        #[unsafe(method_family = none)]
        unsafe fn reset_with_range(&self, range: NSRange);

        #[unsafe(method(indirectRenderCommandAtIndex:))]
        #[unsafe(method_family = none)]
        unsafe fn indirect_render_command_at_index(
            &self,
            command_index: usize,
        ) -> Retained<ProtocolObject<dyn IndirectRenderCommand>>;

        #[unsafe(method(indirectComputeCommandAtIndex:))]
        #[unsafe(method_family = none)]
        unsafe fn indirect_compute_command_at_index(
            &self,
            command_index: usize,
        ) -> Retained<ProtocolObject<dyn IndirectComputeCommand>>;
    }
);
