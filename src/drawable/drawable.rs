use objc2::{Message, extern_protocol, msg_send};
use objc2_foundation::NSObjectProtocol;

use crate::MTLDrawablePresentedHandler;

extern_protocol!(
    /// All "drawable" objects (such as those coming from CAMetalLayer) are expected to conform to this protocol.
    ///
    /// Availability: macOS 10.11+, iOS 8.0+
    ///
    /// # Safety
    ///
    /// Implementors must be valid Objective-C objects that conform to the
    /// `MTLDrawable` protocol.
    #[expect(
        clippy::missing_safety_doc,
        reason = "extern_protocol does not attach this safety section to its generated unsafe trait"
    )]
    pub unsafe trait MTLDrawable: NSObjectProtocol {
        /// Present this drawable immediately.
        #[unsafe(method(present))]
        #[unsafe(method_family = none)]
        fn present(&self);

        /// Present this drawable at a specific host time.
        #[unsafe(method(presentAtTime:))]
        #[unsafe(method_family = none)]
        fn present_at_time(
            &self,
            presentation_time: f64,
        );

        /// Present this drawable while setting a minimum duration in seconds
        /// before allowing this drawable to appear on the display.
        ///
        /// Availability: macOS 10.15.4+, iOS 10.3+, Mac Catalyst 13.4+
        #[unsafe(method(presentAfterMinimumDuration:))]
        #[unsafe(method_family = none)]
        fn present_after_minimum_duration(
            &self,
            duration: f64,
        );

        /// The host time that this drawable was presented on screen.
        /// Returns 0 if a frame has not been presented or has been skipped.
        ///
        /// Availability: macOS 10.15.4+, iOS 10.3+, Mac Catalyst 13.4+
        #[unsafe(method(presentedTime))]
        #[unsafe(method_family = none)]
        fn presented_time(&self) -> f64;

        /// The monotonically incremented ID for all drawable objects created
        /// from the same CAMetalLayer object. The value starts from 0.
        ///
        /// Availability: macOS 10.15.4+, iOS 10.3+, Mac Catalyst 13.4+
        #[unsafe(method(drawableID))]
        #[unsafe(method_family = none)]
        fn drawable_id(&self) -> usize;
    }
);

/// Safe convenience methods for [`MTLDrawable`].
pub trait MTLDrawableExt: MTLDrawable + Message {
    /// Registers a sendable block that Metal invokes after presenting this
    /// drawable on screen.
    ///
    /// Availability: macOS 10.15.4+, iOS 10.3+, Mac Catalyst 13.4+
    fn add_presented_handler(
        &self,
        handler: &MTLDrawablePresentedHandler,
    );
}

impl<T> MTLDrawableExt for T
where
    T: MTLDrawable + Message,
{
    fn add_presented_handler(
        &self,
        handler: &MTLDrawablePresentedHandler,
    ) {
        unsafe {
            let _: () = msg_send![self, addPresentedHandler: handler.as_block()];
        }
    }
}
