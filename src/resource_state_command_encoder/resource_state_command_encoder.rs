use core::ptr::NonNull;
use objc2::{extern_protocol, runtime::ProtocolObject};

use crate::types::{Origin, Region, Size};
use crate::{Buffer, Fence, Texture};
use crate::{CommandEncoder, MapIndirectArguments, SparseTextureMappingMode};

extern_protocol!(
    /// Resource state command encoder
    #[name = "MTLResourceStateCommandEncoder"]
    pub unsafe trait ResourceStateCommandEncoder: CommandEncoder {
        /// Updates multiple regions within a sparse texture.
        /// Safety: pointers must be valid.
        #[optional]
        #[unsafe(method(updateTextureMappings:mode:regions:mipLevels:slices:numRegions:))]
        #[unsafe(method_family = none)]
        unsafe fn update_texture_mappings(
            &self,
            texture: &ProtocolObject<dyn Texture>,
            mode: SparseTextureMappingMode,
            regions: NonNull<Region>,
            mip_levels: NonNull<usize>,
            slices: NonNull<usize>,
            num_regions: usize,
        );

        /// Updates mapping for given sparse texture
        #[optional]
        #[unsafe(method(updateTextureMapping:mode:region:mipLevel:slice:))]
        #[unsafe(method_family = none)]
        unsafe fn update_texture_mapping(
            &self,
            texture: &ProtocolObject<dyn Texture>,
            mode: SparseTextureMappingMode,
            region: Region,
            mip_level: usize,
            slice: usize,
        );

        /// Updates mapping via an indirect buffer using `MapIndirectArguments`.
        #[optional]
        #[unsafe(method(updateTextureMapping:mode:indirectBuffer:indirectBufferOffset:))]
        #[unsafe(method_family = none)]
        unsafe fn update_texture_mapping_indirect(
            &self,
            texture: &ProtocolObject<dyn Texture>,
            mode: SparseTextureMappingMode,
            indirect_buffer: &ProtocolObject<dyn Buffer>,
            indirect_buffer_offset: usize,
        );

        /// Update the fence to capture all GPU work so far enqueued by this encoder.
        #[optional]
        #[unsafe(method(updateFence:))]
        #[unsafe(method_family = none)]
        unsafe fn update_fence(&self, fence: &ProtocolObject<dyn Fence>);

        /// Prevent further GPU work until the fence is reached.
        #[optional]
        #[unsafe(method(waitForFence:))]
        #[unsafe(method_family = none)]
        unsafe fn wait_for_fence(&self, fence: &ProtocolObject<dyn Fence>);

        /// Move sparse page mappings between textures from the same heap.
        #[optional]
        #[unsafe(method(moveTextureMappingsFromTexture:sourceSlice:sourceLevel:sourceOrigin:sourceSize:toTexture:destinationSlice:destinationLevel:destinationOrigin:))]
        #[unsafe(method_family = none)]
        unsafe fn move_texture_mappings(
            &self,
            source_texture: &ProtocolObject<dyn Texture>,
            source_slice: usize,
            source_level: usize,
            source_origin: Origin,
            source_size: Size,
            destination_texture: &ProtocolObject<dyn Texture>,
            destination_slice: usize,
            destination_level: usize,
            destination_origin: Origin,
        );
    }
);
