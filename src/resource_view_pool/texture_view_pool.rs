use objc2::{extern_protocol, runtime::ProtocolObject};

use super::ResourceViewPool;
use crate::{Buffer, ResourceID, Texture, TextureDescriptor};

extern_protocol!(
    /// A pool of lightweight texture views.
    #[name = "MTLTextureViewPool"]
    pub unsafe trait TextureViewPool: ResourceViewPool {
        /// Copies a default texture view to a slot in this texture view pool at an index provided.
        #[unsafe(method(setTextureView:atIndex:))]
        #[unsafe(method_family = none)]
        unsafe fn set_texture_view_at_index(
            &self,
            texture: &ProtocolObject<dyn Texture>,
            index: usize,
        ) -> ResourceID;

        /// Creates a new lightweight texture view.
        #[unsafe(method(setTextureView:descriptor:atIndex:))]
        #[unsafe(method_family = none)]
        unsafe fn set_texture_view_with_descriptor_at_index(
            &self,
            texture: &ProtocolObject<dyn Texture>,
            descriptor: &TextureDescriptor,
            index: usize,
        ) -> ResourceID;

        /// Creates a new lightweight texture view of a buffer.
        #[unsafe(method(setTextureViewFromBuffer:descriptor:offset:bytesPerRow:atIndex:))]
        #[unsafe(method_family = none)]
        unsafe fn set_texture_view_from_buffer(
            &self,
            buffer: &ProtocolObject<dyn Buffer>,
            descriptor: &TextureDescriptor,
            offset: usize,
            bytes_per_row: usize,
            index: usize,
        ) -> ResourceID;
    }
);
