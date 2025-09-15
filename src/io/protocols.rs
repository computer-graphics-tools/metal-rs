use objc2::{extern_protocol, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSObjectProtocol, NSString, NSUInteger};

extern_protocol!(
    /// Represents a queue that schedules IO command buffers.
    #[name = "MTLIOCommandQueue"]
    pub unsafe trait IoCommandQueue: NSObjectProtocol + Send + Sync {
        /// Insert a barrier to order prior and subsequent command buffers.
        #[unsafe(method(enqueueBarrier))]
        #[unsafe(method_family = none)]
        unsafe fn enqueue_barrier(&self);

        // Note: Minimal surface; IOCommandBuffer not yet ported here.
        // Provide methods returning opaque ProtocolObject<dyn Any> later when ported.

        /// Optional label.
        #[unsafe(method(label))]
        #[unsafe(method_family = none)]
        unsafe fn label(&self) -> Option<Retained<NSString>>;

        /// Setter for label.
        #[unsafe(method(setLabel:))]
        #[unsafe(method_family = none)]
        unsafe fn set_label(&self, label: Option<&NSString>);
    }
);

extern_protocol!(
    /// Represents a file handle usable as a source for IO commands.
    #[name = "MTLIOFileHandle"]
    pub unsafe trait IoFileHandle: NSObjectProtocol + Send + Sync {
        /// Optional label.
        #[unsafe(method(label))]
        #[unsafe(method_family = none)]
        unsafe fn label(&self) -> Option<Retained<NSString>>;

        /// Setter for label.
        #[unsafe(method(setLabel:))]
        #[unsafe(method_family = none)]
        unsafe fn set_label(&self, label: Option<&NSString>);
    }
);

extern_protocol!(
    /// Extendible protocol used to wrap scratch buffers for IO commands.
    #[name = "MTLIOScratchBuffer"]
    pub unsafe trait IoScratchBuffer: NSObjectProtocol {}
);

extern_protocol!(
    /// Custom allocator for scratch buffers used by IO queues.
    #[name = "MTLIOScratchBufferAllocator"]
    pub unsafe trait IoScratchBufferAllocator: NSObjectProtocol {
        /// Called when additional scratch memory is required by a load command.
        #[unsafe(method(newScratchBufferWithMinimumSize:))]
        #[unsafe(method_family = new)]
        unsafe fn new_scratch_buffer_with_minimum_size(
            &self,
            minimum_size: NSUInteger,
        ) -> Option<Retained<ProtocolObject<dyn IoScratchBuffer>>>;
    }
);


