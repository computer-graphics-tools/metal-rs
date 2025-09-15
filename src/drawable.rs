use core::ptr::NonNull;
use objc2::{extern_protocol, runtime::ProtocolObject};
use objc2_foundation::NSObjectProtocol;

/// The presented callback function protocol.
///
/// Be careful when you use delta between this `presented_time` and previous
/// frame's `presented_time` to animate next frame. If the frame was presented
/// using `present_after_minimum_duration` or `present_at_time`, the
/// `presented_time` might include delays to meet your specified present time.
/// If you want to measure how much frame you can achieve, use GPUStartTime in
/// the first command buffer of your frame rendering and GPUEndTime of your last
/// frame rendering to calculate the frame interval.
pub type DrawablePresentedHandler =
    *mut block2::DynBlock<dyn Fn(NonNull<ProtocolObject<dyn Drawable>>) >;

extern_protocol!(
    /// All "drawable" objects (such as those coming from CAMetalLayer) are expected to conform to this protocol.
    ///
    /// Mirrors `MTLDrawable`.
    #[name = "MTLDrawable"]
    pub unsafe trait Drawable: NSObjectProtocol {
        /// Present this drawable immediately.
        #[unsafe(method(present))]
        #[unsafe(method_family = none)]
        fn present(&self);

        /// Present this drawable at a specific host time.
        ///
        /// # Safety
        ///
        /// The `presentation_time` must be a valid host time value.
        #[unsafe(method(presentAtTime:))]
        #[unsafe(method_family = none)]
        unsafe fn present_at_time(&self, presentation_time: f64);

        /// Present this drawable while setting a minimum duration in seconds
        /// before allowing this drawable to appear on the display.
        ///
        /// # Safety
        ///
        /// The `duration` must be a non-negative duration in seconds.
        #[unsafe(method(presentAfterMinimumDuration:))]
        #[unsafe(method_family = none)]
        unsafe fn present_after_minimum_duration(&self, duration: f64);

        /// Add a block to be called when this drawable is presented on screen.
        ///
        /// # Safety
        ///
        /// `block` must be a valid pointer.
        #[unsafe(method(addPresentedHandler:))]
        #[unsafe(method_family = none)]
        unsafe fn add_presented_handler(&self, block: DrawablePresentedHandler);

        /// The host time that this drawable was presented on screen.
        /// Returns 0 if a frame has not been presented or has been skipped.
        #[unsafe(method(presentedTime))]
        #[unsafe(method_family = none)]
        unsafe fn presented_time(&self) -> f64;

        /// The monotonically incremented ID for all drawable objects created
        /// from the same CAMetalLayer object. The value starts from 0.
        #[unsafe(method(drawableID))]
        #[unsafe(method_family = none)]
        fn drawable_id(&self) -> usize;
    }
);


