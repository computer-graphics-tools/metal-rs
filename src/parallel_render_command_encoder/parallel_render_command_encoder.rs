use objc2::{extern_protocol, rc::Retained, runtime::ProtocolObject};

use crate::{
    MTLCommandEncoder,
    render_command_encoder::MTLRenderCommandEncoder,
    render_pass::{MTLStoreAction, MTLStoreActionOptions},
};

extern_protocol!(
    /// Parallel render command encoder interface.
    ///
    /// Availability: macOS 10.11+, iOS 8.0+
    ///
    /// # Safety
    ///
    /// Implementors must be valid Objective-C objects that conform to the
    /// `MTLParallelRenderCommandEncoder` protocol.
    #[expect(
        clippy::missing_safety_doc,
        reason = "extern_protocol does not attach this safety section to its generated unsafe trait"
    )]
    pub unsafe trait MTLParallelRenderCommandEncoder: MTLCommandEncoder {
        /// Returns a render command encoder that may encode on another thread, or `None` if Metal cannot create one.
        #[unsafe(method(renderCommandEncoder))]
        #[unsafe(method_family = none)]
        fn render_command_encoder(&self) -> Option<Retained<ProtocolObject<dyn MTLRenderCommandEncoder>>>;

        /// Finalize color store action for a given color attachment.
        ///
        /// Availability: macOS 10.12+, iOS 10.0+
        #[unsafe(method(setColorStoreAction:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_color_store_action_at_index(
            &self,
            store_action: MTLStoreAction,
            color_attachment_index: usize,
        );

        /// Finalize depth store action.
        ///
        /// Availability: macOS 10.12+, iOS 10.0+
        #[unsafe(method(setDepthStoreAction:))]
        #[unsafe(method_family = none)]
        fn set_depth_store_action(
            &self,
            store_action: MTLStoreAction,
        );

        /// Finalize stencil store action.
        ///
        /// Availability: macOS 10.12+, iOS 10.0+
        #[unsafe(method(setStencilStoreAction:))]
        #[unsafe(method_family = none)]
        fn set_stencil_store_action(
            &self,
            store_action: MTLStoreAction,
        );

        /// Finalize color store action options for a given color attachment.
        ///
        /// Availability: macOS 10.13+, iOS 11.0+; deprecated in macOS and iOS 27.0.
        #[deprecated(note = "store action options have no effect on Apple Silicon")]
        #[unsafe(method(setColorStoreActionOptions:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_color_store_action_options_at_index(
            &self,
            store_action_options: MTLStoreActionOptions,
            color_attachment_index: usize,
        );

        /// Finalize depth store action options.
        ///
        /// Availability: macOS 10.13+, iOS 11.0+; deprecated in macOS and iOS 27.0.
        #[deprecated(note = "store action options have no effect on Apple Silicon")]
        #[unsafe(method(setDepthStoreActionOptions:))]
        #[unsafe(method_family = none)]
        fn set_depth_store_action_options(
            &self,
            store_action_options: MTLStoreActionOptions,
        );

        /// Finalize stencil store action options.
        ///
        /// Availability: macOS 10.13+, iOS 11.0+; deprecated in macOS and iOS 27.0.
        #[deprecated(note = "store action options have no effect on Apple Silicon")]
        #[unsafe(method(setStencilStoreActionOptions:))]
        #[unsafe(method_family = none)]
        fn set_stencil_store_action_options(
            &self,
            store_action_options: MTLStoreActionOptions,
        );
    }
);
