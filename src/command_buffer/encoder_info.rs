use objc2::{Message, extern_protocol, msg_send, rc::Retained};
use objc2_foundation::{NSArray, NSObjectProtocol, NSString};

use super::MTLCommandEncoderErrorState;

extern_protocol!(
    /// Execution status information for a Metal command encoder.
    ///
    /// Availability: macOS 11.0+, iOS 14.0+
    ///
    /// # Safety
    ///
    /// Implementers must satisfy the Objective-C `MTLCommandBufferEncoderInfo`
    /// protocol contract.
    #[expect(
        clippy::missing_safety_doc,
        reason = "extern_protocol does not attach this safety section to its generated unsafe trait"
    )]
    pub unsafe trait MTLCommandBufferEncoderInfo: NSObjectProtocol {
        /// The encoder's execution state.
        #[unsafe(method(errorState))]
        #[unsafe(method_family = none)]
        fn error_state(&self) -> MTLCommandEncoderErrorState;
    }
);

/// Rust-native accessors for command-encoder diagnostic information.
pub trait MTLCommandBufferEncoderInfoExt: MTLCommandBufferEncoderInfo + Message {
    /// The debug label at command-buffer submission time.
    fn label(&self) -> String
    where
        Self: Sized,
    {
        let label: Retained<NSString> = unsafe { msg_send![self, label] };
        label.to_string()
    }

    /// The debug signposts inserted into the encoder.
    fn debug_signposts(&self) -> Box<[String]>
    where
        Self: Sized,
    {
        let signposts: Retained<NSArray<NSString>> = unsafe { msg_send![self, debugSignposts] };
        signposts.iter().map(|signpost| signpost.to_string()).collect::<Vec<_>>().into_boxed_slice()
    }
}

impl<T: MTLCommandBufferEncoderInfo + Message> MTLCommandBufferEncoderInfoExt for T {}
