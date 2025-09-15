use objc2::{extern_protocol, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::NSObjectProtocol;

use crate::CommandEncoder;
use crate::render_command_encoder::RenderCommandEncoder;
use crate::render_pass::{StoreAction, StoreActionOptions};

extern_protocol!(
    /// Parallel render command encoder interface.
    #[name = "MTLParallelRenderCommandEncoder"]
    pub unsafe trait ParallelRenderCommandEncoder: CommandEncoder {
        /// Return a new autoreleased render command encoder to encode on a different thread.
        #[unsafe(method(renderCommandEncoder))]
        #[unsafe(method_family = none)]
        fn render_command_encoder(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn RenderCommandEncoder>>>;

        /// Finalize color store action for a given color attachment.
        #[unsafe(method(setColorStoreAction:atIndex:))]
        #[unsafe(method_family = none)]
        unsafe fn set_color_store_action_at_index(
            &self,
            store_action: StoreAction,
            color_attachment_index: usize,
        );

        /// Finalize depth store action.
        #[unsafe(method(setDepthStoreAction:))]
        #[unsafe(method_family = none)]
        unsafe fn set_depth_store_action(&self, store_action: StoreAction);

        /// Finalize stencil store action.
        #[unsafe(method(setStencilStoreAction:))]
        #[unsafe(method_family = none)]
        unsafe fn set_stencil_store_action(&self, store_action: StoreAction);

        /// Finalize color store action options for a given color attachment.
        #[unsafe(method(setColorStoreActionOptions:atIndex:))]
        #[unsafe(method_family = none)]
        unsafe fn set_color_store_action_options_at_index(
            &self,
            store_action_options: StoreActionOptions,
            color_attachment_index: usize,
        );

        /// Finalize depth store action options.
        #[unsafe(method(setDepthStoreActionOptions:))]
        #[unsafe(method_family = none)]
        unsafe fn set_depth_store_action_options(&self, store_action_options: StoreActionOptions);

        /// Finalize stencil store action options.
        #[unsafe(method(setStencilStoreActionOptions:))]
        #[unsafe(method_family = none)]
        unsafe fn set_stencil_store_action_options(&self, store_action_options: StoreActionOptions);
    }
);
