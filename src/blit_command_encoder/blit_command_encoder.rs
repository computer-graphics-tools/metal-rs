use objc2::{extern_protocol, runtime::ProtocolObject};
use objc2_foundation::NSRange;

use crate::types::{Origin, Region, Size};
use crate::{
    Buffer, CommandEncoder, CounterSampleBuffer, Fence, IndirectCommandBuffer, Resource, Tensor,
    Texture,
};

use super::BlitOption;

extern_protocol!(
    /// A command encoder that performs basic copies and blits between buffers and textures.
    #[name = "MTLBlitCommandEncoder"]
    pub unsafe trait BlitCommandEncoder: CommandEncoder {
        /// Flush any copy of this resource from the device's caches, and invalidate any CPU caches if needed.
        #[unsafe(method(synchronizeResource:))]
        #[unsafe(method_family = none)]
        fn synchronize_resource(&self, resource: &ProtocolObject<dyn Resource>);

        /// Flush any copy of this image from the device's caches, and invalidate CPU caches if needed.
        #[unsafe(method(synchronizeTexture:slice:level:))]
        #[unsafe(method_family = none)]
        unsafe fn synchronize_texture_slice_level(
            &self,
            texture: &ProtocolObject<dyn Texture>,
            slice: usize,
            level: usize,
        );

        /// Copy a rectangle of pixels between textures.
        #[unsafe(method(copyFromTexture:sourceSlice:sourceLevel:sourceOrigin:sourceSize:toTexture:destinationSlice:destinationLevel:destinationOrigin:))]
        #[unsafe(method_family = none)]
        unsafe fn copy_from_texture_to_texture(
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

        /// Copy an image from a buffer into a texture.
        #[unsafe(method(copyFromBuffer:sourceOffset:sourceBytesPerRow:sourceBytesPerImage:sourceSize:toTexture:destinationSlice:destinationLevel:destinationOrigin:))]
        #[unsafe(method_family = none)]
        unsafe fn copy_from_buffer_to_texture(
            &self,
            source_buffer: &ProtocolObject<dyn Buffer>,
            source_offset: usize,
            source_bytes_per_row: usize,
            source_bytes_per_image: usize,
            source_size: Size,
            destination_texture: &ProtocolObject<dyn Texture>,
            destination_slice: usize,
            destination_level: usize,
            destination_origin: Origin,
        );

        /// Copy an image from a buffer into a texture with options.
        #[unsafe(method(copyFromBuffer:sourceOffset:sourceBytesPerRow:sourceBytesPerImage:sourceSize:toTexture:destinationSlice:destinationLevel:destinationOrigin:options:))]
        #[unsafe(method_family = none)]
        unsafe fn copy_from_buffer_to_texture_with_options(
            &self,
            source_buffer: &ProtocolObject<dyn Buffer>,
            source_offset: usize,
            source_bytes_per_row: usize,
            source_bytes_per_image: usize,
            source_size: Size,
            destination_texture: &ProtocolObject<dyn Texture>,
            destination_slice: usize,
            destination_level: usize,
            destination_origin: Origin,
            options: BlitOption,
        );

        /// Copy an image from a texture into a buffer.
        #[unsafe(method(copyFromTexture:sourceSlice:sourceLevel:sourceOrigin:sourceSize:toBuffer:destinationOffset:destinationBytesPerRow:destinationBytesPerImage:))]
        #[unsafe(method_family = none)]
        unsafe fn copy_from_texture_to_buffer(
            &self,
            source_texture: &ProtocolObject<dyn Texture>,
            source_slice: usize,
            source_level: usize,
            source_origin: Origin,
            source_size: Size,
            destination_buffer: &ProtocolObject<dyn Buffer>,
            destination_offset: usize,
            destination_bytes_per_row: usize,
            destination_bytes_per_image: usize,
        );

        /// Copy an image from a texture into a buffer with options.
        #[unsafe(method(copyFromTexture:sourceSlice:sourceLevel:sourceOrigin:sourceSize:toBuffer:destinationOffset:destinationBytesPerRow:destinationBytesPerImage:options:))]
        #[unsafe(method_family = none)]
        unsafe fn copy_from_texture_to_buffer_with_options(
            &self,
            source_texture: &ProtocolObject<dyn Texture>,
            source_slice: usize,
            source_level: usize,
            source_origin: Origin,
            source_size: Size,
            destination_buffer: &ProtocolObject<dyn Buffer>,
            destination_offset: usize,
            destination_bytes_per_row: usize,
            destination_bytes_per_image: usize,
            options: BlitOption,
        );

        /// Generate mipmaps for a texture from the base level up to the max level.
        #[unsafe(method(generateMipmapsForTexture:))]
        #[unsafe(method_family = none)]
        fn generate_mipmaps_for_texture(&self, texture: &ProtocolObject<dyn Texture>);

        /// Fill a buffer with a fixed value in each byte.
        #[unsafe(method(fillBuffer:range:value:))]
        #[unsafe(method_family = none)]
        fn fill_buffer_range_value(
            &self,
            buffer: &ProtocolObject<dyn Buffer>,
            range: NSRange,
            value: u8,
        );

        /// Copy whole surfaces between textures.
        #[unsafe(method(copyFromTexture:sourceSlice:sourceLevel:toTexture:destinationSlice:destinationLevel:sliceCount:levelCount:))]
        #[unsafe(method_family = none)]
        unsafe fn copy_surfaces_between_textures(
            &self,
            source_texture: &ProtocolObject<dyn Texture>,
            source_slice: usize,
            source_level: usize,
            destination_texture: &ProtocolObject<dyn Texture>,
            destination_slice: usize,
            destination_level: usize,
            slice_count: usize,
            level_count: usize,
        );

        /// Copy as many whole surfaces as possible between textures.
        #[unsafe(method(copyFromTexture:toTexture:))]
        #[unsafe(method_family = none)]
        unsafe fn copy_from_texture_to_texture_simple(
            &self,
            source_texture: &ProtocolObject<dyn Texture>,
            destination_texture: &ProtocolObject<dyn Texture>,
        );

        /// Basic memory copy between buffers.
        #[unsafe(method(copyFromBuffer:sourceOffset:toBuffer:destinationOffset:size:))]
        #[unsafe(method_family = none)]
        unsafe fn copy_buffer_to_buffer(
            &self,
            source_buffer: &ProtocolObject<dyn Buffer>,
            source_offset: usize,
            destination_buffer: &ProtocolObject<dyn Buffer>,
            destination_offset: usize,
            size: usize,
        );

        /// Update the fence to capture all GPU work so far enqueued by this encoder.
        #[unsafe(method(updateFence:))]
        #[unsafe(method_family = none)]
        fn update_fence(&self, fence: &ProtocolObject<dyn Fence>);

        /// Prevent further GPU work until the fence is reached.
        #[unsafe(method(waitForFence:))]
        #[unsafe(method_family = none)]
        fn wait_for_fence(&self, fence: &ProtocolObject<dyn Fence>);

        /// Copies tile access counters within specified region into provided buffer.
        #[optional]
        #[unsafe(method(getTextureAccessCounters:region:mipLevel:slice:resetCounters:countersBuffer:countersBufferOffset:))]
        #[unsafe(method_family = none)]
        unsafe fn get_texture_access_counters(
            &self,
            texture: &ProtocolObject<dyn Texture>,
            region: Region,
            mip_level: usize,
            slice: usize,
            reset_counters: bool,
            counters_buffer: &ProtocolObject<dyn Buffer>,
            counters_buffer_offset: usize,
        );

        /// Resets tile access counters within specified region.
        #[optional]
        #[unsafe(method(resetTextureAccessCounters:region:mipLevel:slice:))]
        #[unsafe(method_family = none)]
        unsafe fn reset_texture_access_counters(
            &self,
            texture: &ProtocolObject<dyn Texture>,
            region: Region,
            mip_level: usize,
            slice: usize,
        );

        /// Optimizes the texture data for GPU access.
        #[unsafe(method(optimizeContentsForGPUAccess:))]
        #[unsafe(method_family = none)]
        fn optimize_contents_for_gpu_access(&self, texture: &ProtocolObject<dyn Texture>);

        /// Optimizes a subset of the texture data for GPU access.
        #[unsafe(method(optimizeContentsForGPUAccess:slice:level:))]
        #[unsafe(method_family = none)]
        unsafe fn optimize_contents_for_gpu_access_slice_level(
            &self,
            texture: &ProtocolObject<dyn Texture>,
            slice: usize,
            level: usize,
        );

        /// Optimizes the texture data for CPU access.
        #[unsafe(method(optimizeContentsForCPUAccess:))]
        #[unsafe(method_family = none)]
        unsafe fn optimize_contents_for_cpu_access(&self, texture: &ProtocolObject<dyn Texture>);

        /// Optimizes a subset of the texture data for CPU access.
        #[unsafe(method(optimizeContentsForCPUAccess:slice:level:))]
        #[unsafe(method_family = none)]
        unsafe fn optimize_contents_for_cpu_access_slice_level(
            &self,
            texture: &ProtocolObject<dyn Texture>,
            slice: usize,
            level: usize,
        );

        /// Reset commands in an indirect command buffer using the GPU.
        #[unsafe(method(resetCommandsInBuffer:withRange:))]
        #[unsafe(method_family = none)]
        unsafe fn reset_commands_in_buffer(
            &self,
            buffer: &ProtocolObject<dyn IndirectCommandBuffer>,
            range: NSRange,
        );

        /// Copy a region of an indirect command buffer into a destination buffer starting at index using the GPU.
        #[unsafe(method(copyIndirectCommandBuffer:sourceRange:destination:destinationIndex:))]
        #[unsafe(method_family = none)]
        unsafe fn copy_indirect_command_buffer(
            &self,
            source: &ProtocolObject<dyn IndirectCommandBuffer>,
            source_range: NSRange,
            destination: &ProtocolObject<dyn IndirectCommandBuffer>,
            destination_index: usize,
        );

        /// Optimize a range of commands within an indirect command buffer.
        #[unsafe(method(optimizeIndirectCommandBuffer:withRange:))]
        #[unsafe(method_family = none)]
        unsafe fn optimize_indirect_command_buffer(
            &self,
            indirect_command_buffer: &ProtocolObject<dyn IndirectCommandBuffer>,
            range: NSRange,
        );

        /// Sample hardware counters at this point in the blit encoder.
        #[unsafe(method(sampleCountersInBuffer:atSampleIndex:withBarrier:))]
        #[unsafe(method_family = none)]
        unsafe fn sample_counters_in_buffer(
            &self,
            sample_buffer: &ProtocolObject<dyn CounterSampleBuffer>,
            sample_index: usize,
            barrier: bool,
        );

        /// Resolve the counters from the raw buffer to a processed buffer.
        #[unsafe(method(resolveCounters:inRange:destinationBuffer:destinationOffset:))]
        #[unsafe(method_family = none)]
        unsafe fn resolve_counters(
            &self,
            sample_buffer: &ProtocolObject<dyn CounterSampleBuffer>,
            range: NSRange,
            destination_buffer: &ProtocolObject<dyn Buffer>,
            destination_offset: usize,
        );

        /// Copy data between tensor slices.
        #[unsafe(method(copyFromTensor:sourceOrigin:sourceDimensions:toTensor:destinationOrigin:destinationDimensions:))]
        #[unsafe(method_family = none)]
        unsafe fn copy_between_tensors(
            &self,
            source_tensor: &ProtocolObject<dyn Tensor>,
            source_origin: &crate::tensor::TensorExtents,
            source_dimensions: &crate::tensor::TensorExtents,
            destination_tensor: &ProtocolObject<dyn Tensor>,
            destination_origin: &crate::tensor::TensorExtents,
            destination_dimensions: &crate::tensor::TensorExtents,
        );
    }
);
