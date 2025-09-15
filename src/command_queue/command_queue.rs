use objc2::{extern_protocol, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSObjectProtocol, NSString};

use crate::{MTLCommandBuffer, MTLCommandBufferDescriptor, MTLDevice};

extern_protocol!(
    /// A serial queue of command buffers to be executed by the device.
    pub unsafe trait MTLCommandQueue: NSObjectProtocol + Send + Sync {
        /// A string to help identify this object.
        #[unsafe(method(label))]
        #[unsafe(method_family = none)]
        fn label(&self) -> Option<Retained<NSString>>;

        /// Setter for `label`.
        #[unsafe(method(setLabel:))]
        #[unsafe(method_family = none)]
        fn set_label(&self, label: Option<&NSString>);

        /// The device this queue will submit to.
        #[unsafe(method(device))]
        #[unsafe(method_family = none)]
        fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;

        /// Returns a new command buffer used to encode work into this queue that
        /// maintains strong references to resources used within the command buffer.
        #[unsafe(method(commandBuffer))]
        #[unsafe(method_family = none)]
        fn command_buffer(&self) -> Option<Retained<ProtocolObject<dyn MTLCommandBuffer>>>;

        /// Returns a new command buffer used to encode work into this queue.
        #[unsafe(method(commandBufferWithDescriptor:))]
        #[unsafe(method_family = none)]
        unsafe fn command_buffer_with_descriptor(
            &self,
            descriptor: &MTLCommandBufferDescriptor,
        ) -> Option<Retained<ProtocolObject<dyn MTLCommandBuffer>>>;

        /// Returns a new command buffer that does not maintain strong references to resources used within it.
        #[unsafe(method(commandBufferWithUnretainedReferences))]
        #[unsafe(method_family = none)]
        unsafe fn command_buffer_with_unretained_references(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn MTLCommandBuffer>>>;

        /// Inform Xcode about when debug capture should start and stop.
        #[deprecated]
        #[unsafe(method(insertDebugCaptureBoundary))]
        #[unsafe(method_family = none)]
        unsafe fn insert_debug_capture_boundary(&self);
    }
);
