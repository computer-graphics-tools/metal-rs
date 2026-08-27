use core::{ffi::c_void, ops::Range};

use objc2::{Message, extern_protocol, msg_send, runtime::ProtocolObject};
use objc2_foundation::NSRange;

use super::{
    MTLCullMode, MTLDepthClipMode, MTLScissorRect, MTLTriangleFillMode, MTLVertexAmplificationViewMapping, MTLViewport,
    MTLVisibilityResultMode, MTLWinding,
};
use crate::{
    MTLAccelerationStructure, MTLBarrierScope, MTLBuffer, MTLCommandEncoder, MTLCounterSampleBuffer,
    MTLDepthStencilState, MTLFence, MTLHeap, MTLIndexType, MTLIndirectCommandBuffer, MTLIntersectionFunctionTable,
    MTLLogicalToPhysicalColorAttachmentMap, MTLPrimitiveType, MTLRenderPipelineState, MTLRenderStages, MTLResource,
    MTLResourceUsage, MTLSamplerState, MTLSize, MTLStoreAction, MTLStoreActionOptions, MTLTexture,
    MTLVisibleFunctionTable,
    util::{opt_ref_slice_as_ptr, ref_slice_as_ptr},
};

extern_protocol!(
    /// Encodes graphics rendering state and draw commands.
    ///
    /// # Safety
    ///
    /// Implementors must be Objective-C objects that conform to Apple's
    /// `MTLRenderCommandEncoder` protocol.
    #[allow(clippy::missing_safety_doc, clippy::too_many_arguments)]
    pub unsafe trait MTLRenderCommandEncoder: MTLCommandEncoder {
        #[unsafe(method(setRenderPipelineState:))]
        #[unsafe(method_family = none)]
        fn set_render_pipeline_state(
            &self,
            pipeline_state: &ProtocolObject<dyn MTLRenderPipelineState>,
        );

        #[unsafe(method(setVertexBuffer:offset:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_vertex_buffer(
            &self,
            buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
            offset: usize,
            index: usize,
        );

        #[unsafe(method(setVertexBufferOffset:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_vertex_buffer_offset(
            &self,
            offset: usize,
            index: usize,
        );

        #[unsafe(method(setVertexBuffer:offset:attributeStride:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_vertex_buffer_with_attribute_stride(
            &self,
            buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
            offset: usize,
            attribute_stride: usize,
            index: usize,
        );

        #[unsafe(method(setVertexBufferOffset:attributeStride:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_vertex_buffer_offset_with_attribute_stride(
            &self,
            offset: usize,
            attribute_stride: usize,
            index: usize,
        );

        #[unsafe(method(setVertexTexture:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_vertex_texture(
            &self,
            texture: Option<&ProtocolObject<dyn MTLTexture>>,
            index: usize,
        );

        #[unsafe(method(setVertexSamplerState:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_vertex_sampler_state(
            &self,
            sampler: Option<&ProtocolObject<dyn MTLSamplerState>>,
            index: usize,
        );

        #[unsafe(method(setVertexSamplerState:lodMinClamp:lodMaxClamp:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_vertex_sampler_state_with_lod(
            &self,
            sampler: Option<&ProtocolObject<dyn MTLSamplerState>>,
            lod_min_clamp: f32,
            lod_max_clamp: f32,
            index: usize,
        );

        #[unsafe(method(setVertexVisibleFunctionTable:atBufferIndex:))]
        #[unsafe(method_family = none)]
        fn set_vertex_visible_function_table(
            &self,
            table: Option<&ProtocolObject<dyn MTLVisibleFunctionTable>>,
            buffer_index: usize,
        );

        #[unsafe(method(setVertexIntersectionFunctionTable:atBufferIndex:))]
        #[unsafe(method_family = none)]
        fn set_vertex_intersection_function_table(
            &self,
            table: Option<&ProtocolObject<dyn MTLIntersectionFunctionTable>>,
            buffer_index: usize,
        );

        #[unsafe(method(setVertexAccelerationStructure:atBufferIndex:))]
        #[unsafe(method_family = none)]
        fn set_vertex_acceleration_structure(
            &self,
            acceleration_structure: Option<&ProtocolObject<dyn MTLAccelerationStructure>>,
            buffer_index: usize,
        );

        #[unsafe(method(setViewport:))]
        #[unsafe(method_family = none)]
        fn set_viewport(
            &self,
            viewport: MTLViewport,
        );

        #[unsafe(method(setFrontFacingWinding:))]
        #[unsafe(method_family = none)]
        fn set_front_facing_winding(
            &self,
            winding: MTLWinding,
        );

        #[unsafe(method(setCullMode:))]
        #[unsafe(method_family = none)]
        fn set_cull_mode(
            &self,
            cull_mode: MTLCullMode,
        );

        #[unsafe(method(setDepthClipMode:))]
        #[unsafe(method_family = none)]
        fn set_depth_clip_mode(
            &self,
            mode: MTLDepthClipMode,
        );

        #[unsafe(method(setDepthBias:slopeScale:clamp:))]
        #[unsafe(method_family = none)]
        fn set_depth_bias(
            &self,
            depth_bias: f32,
            slope_scale: f32,
            clamp: f32,
        );

        #[unsafe(method(setDepthTestMinBound:maxBound:))]
        #[unsafe(method_family = none)]
        fn set_depth_test_bounds(
            &self,
            min_bound: f32,
            max_bound: f32,
        );

        #[unsafe(method(setScissorRect:))]
        #[unsafe(method_family = none)]
        fn set_scissor_rect(
            &self,
            rect: MTLScissorRect,
        );

        #[unsafe(method(setTriangleFillMode:))]
        #[unsafe(method_family = none)]
        fn set_triangle_fill_mode(
            &self,
            mode: MTLTriangleFillMode,
        );

        #[unsafe(method(setFragmentBuffer:offset:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_fragment_buffer(
            &self,
            buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
            offset: usize,
            index: usize,
        );

        #[unsafe(method(setFragmentBufferOffset:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_fragment_buffer_offset(
            &self,
            offset: usize,
            index: usize,
        );

        #[unsafe(method(setFragmentTexture:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_fragment_texture(
            &self,
            texture: Option<&ProtocolObject<dyn MTLTexture>>,
            index: usize,
        );

        #[unsafe(method(setFragmentSamplerState:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_fragment_sampler_state(
            &self,
            sampler: Option<&ProtocolObject<dyn MTLSamplerState>>,
            index: usize,
        );

        #[unsafe(method(setFragmentSamplerState:lodMinClamp:lodMaxClamp:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_fragment_sampler_state_with_lod(
            &self,
            sampler: Option<&ProtocolObject<dyn MTLSamplerState>>,
            lod_min_clamp: f32,
            lod_max_clamp: f32,
            index: usize,
        );

        #[unsafe(method(setFragmentVisibleFunctionTable:atBufferIndex:))]
        #[unsafe(method_family = none)]
        fn set_fragment_visible_function_table(
            &self,
            table: Option<&ProtocolObject<dyn MTLVisibleFunctionTable>>,
            buffer_index: usize,
        );

        #[unsafe(method(setFragmentIntersectionFunctionTable:atBufferIndex:))]
        #[unsafe(method_family = none)]
        fn set_fragment_intersection_function_table(
            &self,
            table: Option<&ProtocolObject<dyn MTLIntersectionFunctionTable>>,
            buffer_index: usize,
        );

        #[unsafe(method(setFragmentAccelerationStructure:atBufferIndex:))]
        #[unsafe(method_family = none)]
        fn set_fragment_acceleration_structure(
            &self,
            acceleration_structure: Option<&ProtocolObject<dyn MTLAccelerationStructure>>,
            buffer_index: usize,
        );

        #[unsafe(method(setBlendColorRed:green:blue:alpha:))]
        #[unsafe(method_family = none)]
        fn set_blend_color(
            &self,
            red: f32,
            green: f32,
            blue: f32,
            alpha: f32,
        );

        #[unsafe(method(setDepthStencilState:))]
        #[unsafe(method_family = none)]
        fn set_depth_stencil_state(
            &self,
            depth_stencil_state: Option<&ProtocolObject<dyn MTLDepthStencilState>>,
        );

        #[unsafe(method(setStencilReferenceValue:))]
        #[unsafe(method_family = none)]
        fn set_stencil_reference_value(
            &self,
            reference_value: u32,
        );

        #[unsafe(method(setStencilFrontReferenceValue:backReferenceValue:))]
        #[unsafe(method_family = none)]
        fn set_stencil_front_back_reference_value(
            &self,
            front: u32,
            back: u32,
        );

        #[unsafe(method(setVisibilityResultMode:offset:))]
        #[unsafe(method_family = none)]
        fn set_visibility_result_mode(
            &self,
            mode: MTLVisibilityResultMode,
            offset: usize,
        );

        #[unsafe(method(setColorStoreAction:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_color_store_action(
            &self,
            store_action: MTLStoreAction,
            color_attachment_index: usize,
        );

        #[unsafe(method(setDepthStoreAction:))]
        #[unsafe(method_family = none)]
        fn set_depth_store_action(
            &self,
            store_action: MTLStoreAction,
        );

        #[unsafe(method(setStencilStoreAction:))]
        #[unsafe(method_family = none)]
        fn set_stencil_store_action(
            &self,
            store_action: MTLStoreAction,
        );

        #[deprecated(note = "store action options have no effect on Apple Silicon")]
        #[unsafe(method(setColorStoreActionOptions:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_color_store_action_options(
            &self,
            store_action_options: MTLStoreActionOptions,
            color_attachment_index: usize,
        );

        #[deprecated(note = "store action options have no effect on Apple Silicon")]
        #[unsafe(method(setDepthStoreActionOptions:))]
        #[unsafe(method_family = none)]
        fn set_depth_store_action_options(
            &self,
            store_action_options: MTLStoreActionOptions,
        );

        #[deprecated(note = "store action options have no effect on Apple Silicon")]
        #[unsafe(method(setStencilStoreActionOptions:))]
        #[unsafe(method_family = none)]
        fn set_stencil_store_action_options(
            &self,
            store_action_options: MTLStoreActionOptions,
        );

        #[unsafe(method(setObjectBuffer:offset:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_object_buffer(
            &self,
            buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
            offset: usize,
            index: usize,
        );

        #[unsafe(method(setObjectBufferOffset:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_object_buffer_offset(
            &self,
            offset: usize,
            index: usize,
        );

        #[unsafe(method(setObjectTexture:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_object_texture(
            &self,
            texture: Option<&ProtocolObject<dyn MTLTexture>>,
            index: usize,
        );

        #[unsafe(method(setObjectSamplerState:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_object_sampler_state(
            &self,
            sampler: Option<&ProtocolObject<dyn MTLSamplerState>>,
            index: usize,
        );

        #[unsafe(method(setObjectSamplerState:lodMinClamp:lodMaxClamp:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_object_sampler_state_with_lod(
            &self,
            sampler: Option<&ProtocolObject<dyn MTLSamplerState>>,
            lod_min_clamp: f32,
            lod_max_clamp: f32,
            index: usize,
        );

        #[unsafe(method(setObjectThreadgroupMemoryLength:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_object_threadgroup_memory_length(
            &self,
            length: usize,
            index: usize,
        );

        #[unsafe(method(setMeshBuffer:offset:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_mesh_buffer(
            &self,
            buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
            offset: usize,
            index: usize,
        );

        #[unsafe(method(setMeshBufferOffset:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_mesh_buffer_offset(
            &self,
            offset: usize,
            index: usize,
        );

        #[unsafe(method(setMeshTexture:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_mesh_texture(
            &self,
            texture: Option<&ProtocolObject<dyn MTLTexture>>,
            index: usize,
        );

        #[unsafe(method(setMeshSamplerState:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_mesh_sampler_state(
            &self,
            sampler: Option<&ProtocolObject<dyn MTLSamplerState>>,
            index: usize,
        );

        #[unsafe(method(setMeshSamplerState:lodMinClamp:lodMaxClamp:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_mesh_sampler_state_with_lod(
            &self,
            sampler: Option<&ProtocolObject<dyn MTLSamplerState>>,
            lod_min_clamp: f32,
            lod_max_clamp: f32,
            index: usize,
        );

        #[unsafe(method(drawMeshThreadgroups:threadsPerObjectThreadgroup:threadsPerMeshThreadgroup:))]
        #[unsafe(method_family = none)]
        fn draw_mesh_threadgroups(
            &self,
            threadgroups_per_grid: MTLSize,
            threads_per_object_threadgroup: MTLSize,
            threads_per_mesh_threadgroup: MTLSize,
        );

        #[unsafe(method(drawMeshThreads:threadsPerObjectThreadgroup:threadsPerMeshThreadgroup:))]
        #[unsafe(method_family = none)]
        fn draw_mesh_threads(
            &self,
            threads_per_grid: MTLSize,
            threads_per_object_threadgroup: MTLSize,
            threads_per_mesh_threadgroup: MTLSize,
        );

        #[unsafe(method(drawMeshThreadgroupsWithIndirectBuffer:indirectBufferOffset:threadsPerObjectThreadgroup:threadsPerMeshThreadgroup:))]
        #[unsafe(method_family = none)]
        fn draw_mesh_threadgroups_indirect(
            &self,
            indirect_buffer: &ProtocolObject<dyn MTLBuffer>,
            indirect_buffer_offset: usize,
            threads_per_object_threadgroup: MTLSize,
            threads_per_mesh_threadgroup: MTLSize,
        );

        #[unsafe(method(drawPrimitives:vertexStart:vertexCount:))]
        #[unsafe(method_family = none)]
        fn draw_primitives(
            &self,
            primitive_type: MTLPrimitiveType,
            vertex_start: usize,
            vertex_count: usize,
        );

        #[unsafe(method(drawPrimitives:vertexStart:vertexCount:instanceCount:))]
        #[unsafe(method_family = none)]
        fn draw_primitives_with_instance_count(
            &self,
            primitive_type: MTLPrimitiveType,
            vertex_start: usize,
            vertex_count: usize,
            instance_count: usize,
        );

        #[unsafe(method(drawPrimitives:vertexStart:vertexCount:instanceCount:baseInstance:))]
        #[unsafe(method_family = none)]
        fn draw_primitives_instanced(
            &self,
            primitive_type: MTLPrimitiveType,
            vertex_start: usize,
            vertex_count: usize,
            instance_count: usize,
            base_instance: usize,
        );

        #[unsafe(method(drawIndexedPrimitives:indexCount:indexType:indexBuffer:indexBufferOffset:))]
        #[unsafe(method_family = none)]
        fn draw_indexed_primitives(
            &self,
            primitive_type: MTLPrimitiveType,
            index_count: usize,
            index_type: MTLIndexType,
            index_buffer: &ProtocolObject<dyn MTLBuffer>,
            index_buffer_offset: usize,
        );

        #[unsafe(method(drawIndexedPrimitives:indexCount:indexType:indexBuffer:indexBufferOffset:instanceCount:))]
        #[unsafe(method_family = none)]
        fn draw_indexed_primitives_instanced(
            &self,
            primitive_type: MTLPrimitiveType,
            index_count: usize,
            index_type: MTLIndexType,
            index_buffer: &ProtocolObject<dyn MTLBuffer>,
            index_buffer_offset: usize,
            instance_count: usize,
        );

        #[unsafe(method(drawIndexedPrimitives:indexCount:indexType:indexBuffer:indexBufferOffset:instanceCount:baseVertex:baseInstance:))]
        #[unsafe(method_family = none)]
        fn draw_indexed_primitives_instanced_with_base_vertex(
            &self,
            primitive_type: MTLPrimitiveType,
            index_count: usize,
            index_type: MTLIndexType,
            index_buffer: &ProtocolObject<dyn MTLBuffer>,
            index_buffer_offset: usize,
            instance_count: usize,
            base_vertex: isize,
            base_instance: usize,
        );

        #[unsafe(method(drawPrimitives:indirectBuffer:indirectBufferOffset:))]
        #[unsafe(method_family = none)]
        fn draw_primitives_indirect(
            &self,
            primitive_type: MTLPrimitiveType,
            indirect_buffer: &ProtocolObject<dyn MTLBuffer>,
            indirect_buffer_offset: usize,
        );

        #[unsafe(method(drawIndexedPrimitives:indexType:indexBuffer:indexBufferOffset:indirectBuffer:indirectBufferOffset:))]
        #[unsafe(method_family = none)]
        fn draw_indexed_primitives_indirect(
            &self,
            primitive_type: MTLPrimitiveType,
            index_type: MTLIndexType,
            index_buffer: &ProtocolObject<dyn MTLBuffer>,
            index_buffer_offset: usize,
            indirect_buffer: &ProtocolObject<dyn MTLBuffer>,
            indirect_buffer_offset: usize,
        );

        #[deprecated(note = "use memory_barrier_with_scope with MTLBarrierScope::RenderTargets instead")]
        #[unsafe(method(textureBarrier))]
        #[unsafe(method_family = none)]
        fn texture_barrier(&self);

        #[unsafe(method(updateFence:afterStages:))]
        #[unsafe(method_family = none)]
        fn update_fence_after_stages(
            &self,
            fence: &ProtocolObject<dyn MTLFence>,
            stages: MTLRenderStages,
        );

        #[unsafe(method(waitForFence:beforeStages:))]
        #[unsafe(method_family = none)]
        fn wait_for_fence_before_stages(
            &self,
            fence: &ProtocolObject<dyn MTLFence>,
            stages: MTLRenderStages,
        );

        #[unsafe(method(setTessellationFactorBuffer:offset:instanceStride:))]
        #[unsafe(method_family = none)]
        fn set_tessellation_factor_buffer(
            &self,
            buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
            offset: usize,
            instance_stride: usize,
        );

        #[unsafe(method(setTessellationFactorScale:))]
        #[unsafe(method_family = none)]
        fn set_tessellation_factor_scale(
            &self,
            scale: f32,
        );

        #[unsafe(method(drawPatches:patchStart:patchCount:patchIndexBuffer:patchIndexBufferOffset:instanceCount:baseInstance:))]
        #[unsafe(method_family = none)]
        fn draw_patches(
            &self,
            number_of_patch_control_points: usize,
            patch_start: usize,
            patch_count: usize,
            patch_index_buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
            patch_index_buffer_offset: usize,
            instance_count: usize,
            base_instance: usize,
        );

        #[unsafe(method(drawPatches:patchIndexBuffer:patchIndexBufferOffset:indirectBuffer:indirectBufferOffset:))]
        #[unsafe(method_family = none)]
        fn draw_patches_indirect(
            &self,
            number_of_patch_control_points: usize,
            patch_index_buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
            patch_index_buffer_offset: usize,
            indirect_buffer: &ProtocolObject<dyn MTLBuffer>,
            indirect_buffer_offset: usize,
        );

        #[unsafe(method(drawIndexedPatches:patchStart:patchCount:patchIndexBuffer:patchIndexBufferOffset:controlPointIndexBuffer:controlPointIndexBufferOffset:instanceCount:baseInstance:))]
        #[unsafe(method_family = none)]
        fn draw_indexed_patches(
            &self,
            number_of_patch_control_points: usize,
            patch_start: usize,
            patch_count: usize,
            patch_index_buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
            patch_index_buffer_offset: usize,
            control_point_index_buffer: &ProtocolObject<dyn MTLBuffer>,
            control_point_index_buffer_offset: usize,
            instance_count: usize,
            base_instance: usize,
        );

        #[unsafe(method(drawIndexedPatches:patchIndexBuffer:patchIndexBufferOffset:controlPointIndexBuffer:controlPointIndexBufferOffset:indirectBuffer:indirectBufferOffset:))]
        #[unsafe(method_family = none)]
        fn draw_indexed_patches_indirect(
            &self,
            number_of_patch_control_points: usize,
            patch_index_buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
            patch_index_buffer_offset: usize,
            control_point_index_buffer: &ProtocolObject<dyn MTLBuffer>,
            control_point_index_buffer_offset: usize,
            indirect_buffer: &ProtocolObject<dyn MTLBuffer>,
            indirect_buffer_offset: usize,
        );

        #[unsafe(method(tileWidth))]
        #[unsafe(method_family = none)]
        fn tile_width(&self) -> usize;

        #[unsafe(method(tileHeight))]
        #[unsafe(method_family = none)]
        fn tile_height(&self) -> usize;

        #[unsafe(method(setTileBuffer:offset:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_tile_buffer(
            &self,
            buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
            offset: usize,
            index: usize,
        );

        #[unsafe(method(setTileBufferOffset:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_tile_buffer_offset(
            &self,
            offset: usize,
            index: usize,
        );

        #[unsafe(method(setTileTexture:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_tile_texture(
            &self,
            texture: Option<&ProtocolObject<dyn MTLTexture>>,
            index: usize,
        );

        #[unsafe(method(setTileSamplerState:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_tile_sampler_state(
            &self,
            sampler: Option<&ProtocolObject<dyn MTLSamplerState>>,
            index: usize,
        );

        #[unsafe(method(setTileSamplerState:lodMinClamp:lodMaxClamp:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_tile_sampler_state_with_lod(
            &self,
            sampler: Option<&ProtocolObject<dyn MTLSamplerState>>,
            lod_min_clamp: f32,
            lod_max_clamp: f32,
            index: usize,
        );

        #[unsafe(method(setTileVisibleFunctionTable:atBufferIndex:))]
        #[unsafe(method_family = none)]
        fn set_tile_visible_function_table(
            &self,
            table: Option<&ProtocolObject<dyn MTLVisibleFunctionTable>>,
            buffer_index: usize,
        );

        #[unsafe(method(setTileIntersectionFunctionTable:atBufferIndex:))]
        #[unsafe(method_family = none)]
        fn set_tile_intersection_function_table(
            &self,
            table: Option<&ProtocolObject<dyn MTLIntersectionFunctionTable>>,
            buffer_index: usize,
        );

        #[unsafe(method(setTileAccelerationStructure:atBufferIndex:))]
        #[unsafe(method_family = none)]
        fn set_tile_acceleration_structure(
            &self,
            acceleration_structure: Option<&ProtocolObject<dyn MTLAccelerationStructure>>,
            buffer_index: usize,
        );

        #[unsafe(method(dispatchThreadsPerTile:))]
        #[unsafe(method_family = none)]
        fn dispatch_threads_per_tile(
            &self,
            threads_per_tile: MTLSize,
        );

        #[unsafe(method(setThreadgroupMemoryLength:offset:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_threadgroup_memory_length(
            &self,
            length: usize,
            offset: usize,
            index: usize,
        );

        #[deprecated(note = "use use_resource_at_stages instead")]
        #[unsafe(method(useResource:usage:))]
        #[unsafe(method_family = none)]
        fn use_resource(
            &self,
            resource: &ProtocolObject<dyn MTLResource>,
            usage: MTLResourceUsage,
        );

        #[unsafe(method(useResource:usage:stages:))]
        #[unsafe(method_family = none)]
        fn use_resource_at_stages(
            &self,
            resource: &ProtocolObject<dyn MTLResource>,
            usage: MTLResourceUsage,
            stages: MTLRenderStages,
        );

        #[deprecated(note = "use use_heap_at_stages instead")]
        #[unsafe(method(useHeap:))]
        #[unsafe(method_family = none)]
        fn use_heap(
            &self,
            heap: &ProtocolObject<dyn MTLHeap>,
        );

        #[unsafe(method(useHeap:stages:))]
        #[unsafe(method_family = none)]
        fn use_heap_at_stages(
            &self,
            heap: &ProtocolObject<dyn MTLHeap>,
            stages: MTLRenderStages,
        );

        #[unsafe(method(executeCommandsInBuffer:indirectBuffer:indirectBufferOffset:))]
        #[unsafe(method_family = none)]
        fn execute_commands_in_buffer_indirect(
            &self,
            indirect_command_buffer: &ProtocolObject<dyn MTLIndirectCommandBuffer>,
            indirect_range_buffer: &ProtocolObject<dyn MTLBuffer>,
            indirect_buffer_offset: usize,
        );

        #[unsafe(method(memoryBarrierWithScope:afterStages:beforeStages:))]
        #[unsafe(method_family = none)]
        fn memory_barrier_with_scope(
            &self,
            scope: MTLBarrierScope,
            after_stages: MTLRenderStages,
            before_stages: MTLRenderStages,
        );

        #[unsafe(method(sampleCountersInBuffer:atSampleIndex:withBarrier:))]
        #[unsafe(method_family = none)]
        fn sample_counters_in_buffer(
            &self,
            sample_buffer: &ProtocolObject<dyn MTLCounterSampleBuffer>,
            sample_index: usize,
            barrier: bool,
        );

        #[unsafe(method(setColorAttachmentMap:))]
        #[unsafe(method_family = none)]
        fn set_color_attachment_map(
            &self,
            mapping: Option<&MTLLogicalToPhysicalColorAttachmentMap>,
        );
    }
);

/// Safe slice-based wrappers for render-encoder selectors with array inputs.
///
/// Bulk binding methods panic unless every input slice length equals the
/// binding range length, preventing Metal from reading beyond a slice passed
/// across FFI. Methods with an explicit count derive it from the corresponding
/// slice whenever Metal's nullability contract permits that.
pub trait MTLRenderCommandEncoderExt: MTLRenderCommandEncoder + Message + Sized {
    fn set_vertex_bytes(
        &self,
        bytes: &[u8],
        index: usize,
    ) {
        unsafe {
            let _: () = msg_send![
                self,
                setVertexBytes: bytes.as_ptr().cast::<c_void>(),
                length: bytes.len(),
                atIndex: index
            ];
        }
    }

    fn set_vertex_bytes_with_attribute_stride(
        &self,
        bytes: &[u8],
        attribute_stride: usize,
        index: usize,
    ) {
        unsafe {
            let _: () = msg_send![
                self,
                setVertexBytes: bytes.as_ptr().cast::<c_void>(),
                length: bytes.len(),
                attributeStride: attribute_stride,
                atIndex: index
            ];
        }
    }

    fn set_vertex_buffers(
        &self,
        buffers: &[Option<&ProtocolObject<dyn MTLBuffer>>],
        offsets: &[usize],
        range: Range<usize>,
    ) {
        assert_eq!(buffers.len(), offsets.len(), "buffers and offsets must have equal lengths");
        let range = checked_range(range, buffers.len());
        unsafe {
            let _: () = msg_send![
                self,
                setVertexBuffers: opt_ref_slice_as_ptr(buffers),
                offsets: offsets.as_ptr(),
                withRange: range
            ];
        }
    }

    fn set_vertex_buffers_with_attribute_strides(
        &self,
        buffers: &[Option<&ProtocolObject<dyn MTLBuffer>>],
        offsets: &[usize],
        attribute_strides: &[usize],
        range: Range<usize>,
    ) {
        assert_eq!(buffers.len(), offsets.len(), "buffers and offsets must have equal lengths");
        assert_eq!(buffers.len(), attribute_strides.len(), "buffers and attribute strides must have equal lengths");
        let range = checked_range(range, buffers.len());
        unsafe {
            let _: () = msg_send![
                self,
                setVertexBuffers: opt_ref_slice_as_ptr(buffers),
                offsets: offsets.as_ptr(),
                attributeStrides: attribute_strides.as_ptr(),
                withRange: range
            ];
        }
    }

    fn set_vertex_textures(
        &self,
        textures: &[Option<&ProtocolObject<dyn MTLTexture>>],
        range: Range<usize>,
    ) {
        let range = checked_range(range, textures.len());
        unsafe {
            let _: () = msg_send![
                self,
                setVertexTextures: opt_ref_slice_as_ptr(textures),
                withRange: range
            ];
        }
    }

    fn set_vertex_sampler_states(
        &self,
        samplers: &[Option<&ProtocolObject<dyn MTLSamplerState>>],
        range: Range<usize>,
    ) {
        let range = checked_range(range, samplers.len());
        unsafe {
            let _: () = msg_send![
                self,
                setVertexSamplerStates: opt_ref_slice_as_ptr(samplers),
                withRange: range
            ];
        }
    }

    fn set_vertex_sampler_states_with_lods(
        &self,
        samplers: &[Option<&ProtocolObject<dyn MTLSamplerState>>],
        lod_min_clamps: &[f32],
        lod_max_clamps: &[f32],
        range: Range<usize>,
    ) {
        assert_eq!(samplers.len(), lod_min_clamps.len(), "samplers and minimum LODs must have equal lengths");
        assert_eq!(samplers.len(), lod_max_clamps.len(), "samplers and maximum LODs must have equal lengths");
        let range = checked_range(range, samplers.len());
        unsafe {
            let _: () = msg_send![
                self,
                setVertexSamplerStates: opt_ref_slice_as_ptr(samplers),
                lodMinClamps: lod_min_clamps.as_ptr(),
                lodMaxClamps: lod_max_clamps.as_ptr(),
                withRange: range
            ];
        }
    }

    fn set_vertex_visible_function_tables(
        &self,
        tables: &[Option<&ProtocolObject<dyn MTLVisibleFunctionTable>>],
        range: Range<usize>,
    ) {
        let range = checked_range(range, tables.len());
        unsafe {
            let _: () = msg_send![
                self,
                setVertexVisibleFunctionTables: opt_ref_slice_as_ptr(tables),
                withBufferRange: range
            ];
        }
    }

    fn set_vertex_intersection_function_tables(
        &self,
        tables: &[Option<&ProtocolObject<dyn MTLIntersectionFunctionTable>>],
        range: Range<usize>,
    ) {
        let range = checked_range(range, tables.len());
        unsafe {
            let _: () = msg_send![
                self,
                setVertexIntersectionFunctionTables: opt_ref_slice_as_ptr(tables),
                withBufferRange: range
            ];
        }
    }

    fn set_viewports(
        &self,
        viewports: &[MTLViewport],
    ) {
        unsafe {
            let _: () = msg_send![self, setViewports: viewports.as_ptr(), count: viewports.len()];
        }
    }

    fn set_vertex_amplification_count(
        &self,
        count: usize,
        mappings: Option<&[MTLVertexAmplificationViewMapping]>,
    ) {
        assert!(count <= 2, "Metal supports at most two vertex amplifications");
        if let Some(mappings) = mappings {
            assert_eq!(mappings.len(), count, "mapping count must match amplification count");
        }
        let mappings = mappings.map_or(core::ptr::null(), <[MTLVertexAmplificationViewMapping]>::as_ptr);
        unsafe {
            let _: () = msg_send![
                self,
                setVertexAmplificationCount: count,
                viewMappings: mappings
            ];
        }
    }

    fn set_vertex_amplification_view_mappings(
        &self,
        mappings: &[MTLVertexAmplificationViewMapping],
    ) {
        self.set_vertex_amplification_count(mappings.len(), Some(mappings));
    }

    fn set_scissor_rects(
        &self,
        rects: &[MTLScissorRect],
    ) {
        unsafe {
            let _: () = msg_send![self, setScissorRects: rects.as_ptr(), count: rects.len()];
        }
    }

    fn set_fragment_bytes(
        &self,
        bytes: &[u8],
        index: usize,
    ) {
        unsafe {
            let _: () = msg_send![
                self,
                setFragmentBytes: bytes.as_ptr().cast::<c_void>(),
                length: bytes.len(),
                atIndex: index
            ];
        }
    }

    fn set_fragment_buffers(
        &self,
        buffers: &[Option<&ProtocolObject<dyn MTLBuffer>>],
        offsets: &[usize],
        range: Range<usize>,
    ) {
        assert_eq!(buffers.len(), offsets.len(), "buffers and offsets must have equal lengths");
        let range = checked_range(range, buffers.len());
        unsafe {
            let _: () = msg_send![
                self,
                setFragmentBuffers: opt_ref_slice_as_ptr(buffers),
                offsets: offsets.as_ptr(),
                withRange: range
            ];
        }
    }

    fn set_fragment_textures(
        &self,
        textures: &[Option<&ProtocolObject<dyn MTLTexture>>],
        range: Range<usize>,
    ) {
        let range = checked_range(range, textures.len());
        unsafe {
            let _: () = msg_send![
                self,
                setFragmentTextures: opt_ref_slice_as_ptr(textures),
                withRange: range
            ];
        }
    }

    fn set_fragment_sampler_states(
        &self,
        samplers: &[Option<&ProtocolObject<dyn MTLSamplerState>>],
        range: Range<usize>,
    ) {
        let range = checked_range(range, samplers.len());
        unsafe {
            let _: () = msg_send![
                self,
                setFragmentSamplerStates: opt_ref_slice_as_ptr(samplers),
                withRange: range
            ];
        }
    }

    fn set_fragment_sampler_states_with_lods(
        &self,
        samplers: &[Option<&ProtocolObject<dyn MTLSamplerState>>],
        lod_min_clamps: &[f32],
        lod_max_clamps: &[f32],
        range: Range<usize>,
    ) {
        assert_eq!(samplers.len(), lod_min_clamps.len(), "samplers and minimum LODs must have equal lengths");
        assert_eq!(samplers.len(), lod_max_clamps.len(), "samplers and maximum LODs must have equal lengths");
        let range = checked_range(range, samplers.len());
        unsafe {
            let _: () = msg_send![
                self,
                setFragmentSamplerStates: opt_ref_slice_as_ptr(samplers),
                lodMinClamps: lod_min_clamps.as_ptr(),
                lodMaxClamps: lod_max_clamps.as_ptr(),
                withRange: range
            ];
        }
    }

    fn set_fragment_visible_function_tables(
        &self,
        tables: &[Option<&ProtocolObject<dyn MTLVisibleFunctionTable>>],
        range: Range<usize>,
    ) {
        let range = checked_range(range, tables.len());
        unsafe {
            let _: () = msg_send![
                self,
                setFragmentVisibleFunctionTables: opt_ref_slice_as_ptr(tables),
                withBufferRange: range
            ];
        }
    }

    fn set_fragment_intersection_function_tables(
        &self,
        tables: &[Option<&ProtocolObject<dyn MTLIntersectionFunctionTable>>],
        range: Range<usize>,
    ) {
        let range = checked_range(range, tables.len());
        unsafe {
            let _: () = msg_send![
                self,
                setFragmentIntersectionFunctionTables: opt_ref_slice_as_ptr(tables),
                withBufferRange: range
            ];
        }
    }

    fn set_object_bytes(
        &self,
        bytes: &[u8],
        index: usize,
    ) {
        unsafe {
            let _: () = msg_send![
                self,
                setObjectBytes: bytes.as_ptr().cast::<c_void>(),
                length: bytes.len(),
                atIndex: index
            ];
        }
    }

    fn set_object_buffers(
        &self,
        buffers: &[Option<&ProtocolObject<dyn MTLBuffer>>],
        offsets: &[usize],
        range: Range<usize>,
    ) {
        assert_eq!(buffers.len(), offsets.len(), "buffers and offsets must have equal lengths");
        let range = checked_range(range, buffers.len());
        unsafe {
            let _: () = msg_send![
                self,
                setObjectBuffers: opt_ref_slice_as_ptr(buffers),
                offsets: offsets.as_ptr(),
                withRange: range
            ];
        }
    }

    fn set_object_textures(
        &self,
        textures: &[Option<&ProtocolObject<dyn MTLTexture>>],
        range: Range<usize>,
    ) {
        let range = checked_range(range, textures.len());
        unsafe {
            let _: () = msg_send![
                self,
                setObjectTextures: opt_ref_slice_as_ptr(textures),
                withRange: range
            ];
        }
    }

    fn set_object_sampler_states(
        &self,
        samplers: &[Option<&ProtocolObject<dyn MTLSamplerState>>],
        range: Range<usize>,
    ) {
        let range = checked_range(range, samplers.len());
        unsafe {
            let _: () = msg_send![
                self,
                setObjectSamplerStates: opt_ref_slice_as_ptr(samplers),
                withRange: range
            ];
        }
    }

    fn set_object_sampler_states_with_lods(
        &self,
        samplers: &[Option<&ProtocolObject<dyn MTLSamplerState>>],
        lod_min_clamps: &[f32],
        lod_max_clamps: &[f32],
        range: Range<usize>,
    ) {
        assert_eq!(samplers.len(), lod_min_clamps.len(), "samplers and minimum LODs must have equal lengths");
        assert_eq!(samplers.len(), lod_max_clamps.len(), "samplers and maximum LODs must have equal lengths");
        let range = checked_range(range, samplers.len());
        unsafe {
            let _: () = msg_send![
                self,
                setObjectSamplerStates: opt_ref_slice_as_ptr(samplers),
                lodMinClamps: lod_min_clamps.as_ptr(),
                lodMaxClamps: lod_max_clamps.as_ptr(),
                withRange: range
            ];
        }
    }

    fn set_mesh_bytes(
        &self,
        bytes: &[u8],
        index: usize,
    ) {
        unsafe {
            let _: () = msg_send![
                self,
                setMeshBytes: bytes.as_ptr().cast::<c_void>(),
                length: bytes.len(),
                atIndex: index
            ];
        }
    }

    fn set_mesh_buffers(
        &self,
        buffers: &[Option<&ProtocolObject<dyn MTLBuffer>>],
        offsets: &[usize],
        range: Range<usize>,
    ) {
        assert_eq!(buffers.len(), offsets.len(), "buffers and offsets must have equal lengths");
        let range = checked_range(range, buffers.len());
        unsafe {
            let _: () = msg_send![
                self,
                setMeshBuffers: opt_ref_slice_as_ptr(buffers),
                offsets: offsets.as_ptr(),
                withRange: range
            ];
        }
    }

    fn set_mesh_textures(
        &self,
        textures: &[Option<&ProtocolObject<dyn MTLTexture>>],
        range: Range<usize>,
    ) {
        let range = checked_range(range, textures.len());
        unsafe {
            let _: () = msg_send![
                self,
                setMeshTextures: opt_ref_slice_as_ptr(textures),
                withRange: range
            ];
        }
    }

    fn set_mesh_sampler_states(
        &self,
        samplers: &[Option<&ProtocolObject<dyn MTLSamplerState>>],
        range: Range<usize>,
    ) {
        let range = checked_range(range, samplers.len());
        unsafe {
            let _: () = msg_send![
                self,
                setMeshSamplerStates: opt_ref_slice_as_ptr(samplers),
                withRange: range
            ];
        }
    }

    fn set_mesh_sampler_states_with_lods(
        &self,
        samplers: &[Option<&ProtocolObject<dyn MTLSamplerState>>],
        lod_min_clamps: &[f32],
        lod_max_clamps: &[f32],
        range: Range<usize>,
    ) {
        assert_eq!(samplers.len(), lod_min_clamps.len(), "samplers and minimum LODs must have equal lengths");
        assert_eq!(samplers.len(), lod_max_clamps.len(), "samplers and maximum LODs must have equal lengths");
        let range = checked_range(range, samplers.len());
        unsafe {
            let _: () = msg_send![
                self,
                setMeshSamplerStates: opt_ref_slice_as_ptr(samplers),
                lodMinClamps: lod_min_clamps.as_ptr(),
                lodMaxClamps: lod_max_clamps.as_ptr(),
                withRange: range
            ];
        }
    }

    fn set_tile_bytes(
        &self,
        bytes: &[u8],
        index: usize,
    ) {
        unsafe {
            let _: () = msg_send![
                self,
                setTileBytes: bytes.as_ptr().cast::<c_void>(),
                length: bytes.len(),
                atIndex: index
            ];
        }
    }

    fn set_tile_buffers(
        &self,
        buffers: &[Option<&ProtocolObject<dyn MTLBuffer>>],
        offsets: &[usize],
        range: Range<usize>,
    ) {
        assert_eq!(buffers.len(), offsets.len(), "buffers and offsets must have equal lengths");
        let range = checked_range(range, buffers.len());
        unsafe {
            let _: () = msg_send![
                self,
                setTileBuffers: opt_ref_slice_as_ptr(buffers),
                offsets: offsets.as_ptr(),
                withRange: range
            ];
        }
    }

    fn set_tile_textures(
        &self,
        textures: &[Option<&ProtocolObject<dyn MTLTexture>>],
        range: Range<usize>,
    ) {
        let range = checked_range(range, textures.len());
        unsafe {
            let _: () = msg_send![self, setTileTextures: opt_ref_slice_as_ptr(textures), withRange: range];
        }
    }

    fn set_tile_sampler_states(
        &self,
        samplers: &[Option<&ProtocolObject<dyn MTLSamplerState>>],
        range: Range<usize>,
    ) {
        let range = checked_range(range, samplers.len());
        unsafe {
            let _: () = msg_send![
                self,
                setTileSamplerStates: opt_ref_slice_as_ptr(samplers),
                withRange: range
            ];
        }
    }

    fn set_tile_sampler_states_with_lods(
        &self,
        samplers: &[Option<&ProtocolObject<dyn MTLSamplerState>>],
        lod_min_clamps: &[f32],
        lod_max_clamps: &[f32],
        range: Range<usize>,
    ) {
        assert_eq!(samplers.len(), lod_min_clamps.len(), "samplers and minimum LODs must have equal lengths");
        assert_eq!(samplers.len(), lod_max_clamps.len(), "samplers and maximum LODs must have equal lengths");
        let range = checked_range(range, samplers.len());
        unsafe {
            let _: () = msg_send![
                self,
                setTileSamplerStates: opt_ref_slice_as_ptr(samplers),
                lodMinClamps: lod_min_clamps.as_ptr(),
                lodMaxClamps: lod_max_clamps.as_ptr(),
                withRange: range
            ];
        }
    }

    fn set_tile_visible_function_tables(
        &self,
        tables: &[Option<&ProtocolObject<dyn MTLVisibleFunctionTable>>],
        range: Range<usize>,
    ) {
        let range = checked_range(range, tables.len());
        unsafe {
            let _: () = msg_send![
                self,
                setTileVisibleFunctionTables: opt_ref_slice_as_ptr(tables),
                withBufferRange: range
            ];
        }
    }

    fn set_tile_intersection_function_tables(
        &self,
        tables: &[Option<&ProtocolObject<dyn MTLIntersectionFunctionTable>>],
        range: Range<usize>,
    ) {
        let range = checked_range(range, tables.len());
        unsafe {
            let _: () = msg_send![
                self,
                setTileIntersectionFunctionTables: opt_ref_slice_as_ptr(tables),
                withBufferRange: range
            ];
        }
    }

    #[deprecated(note = "use use_resources_at_stages instead")]
    fn use_resources(
        &self,
        resources: &[&ProtocolObject<dyn MTLResource>],
        usage: MTLResourceUsage,
    ) {
        unsafe {
            let _: () = msg_send![
                self,
                useResources: ref_slice_as_ptr(resources),
                count: resources.len(),
                usage: usage
            ];
        }
    }

    fn use_resources_at_stages(
        &self,
        resources: &[&ProtocolObject<dyn MTLResource>],
        usage: MTLResourceUsage,
        stages: MTLRenderStages,
    ) {
        unsafe {
            let _: () = msg_send![
                self,
                useResources: ref_slice_as_ptr(resources),
                count: resources.len(),
                usage: usage,
                stages: stages
            ];
        }
    }

    #[deprecated(note = "use use_heaps_at_stages instead")]
    fn use_heaps(
        &self,
        heaps: &[&ProtocolObject<dyn MTLHeap>],
    ) {
        unsafe {
            let _: () = msg_send![self, useHeaps: ref_slice_as_ptr(heaps), count: heaps.len()];
        }
    }

    fn use_heaps_at_stages(
        &self,
        heaps: &[&ProtocolObject<dyn MTLHeap>],
        stages: MTLRenderStages,
    ) {
        unsafe {
            let _: () = msg_send![
                self,
                useHeaps: ref_slice_as_ptr(heaps),
                count: heaps.len(),
                stages: stages
            ];
        }
    }

    fn execute_commands_in_buffer(
        &self,
        indirect_command_buffer: &ProtocolObject<dyn MTLIndirectCommandBuffer>,
        execution_range: Range<usize>,
    ) {
        let range = NSRange::from(execution_range);
        unsafe {
            let _: () = msg_send![
                self,
                executeCommandsInBuffer: indirect_command_buffer,
                withRange: range
            ];
        }
    }

    fn memory_barrier_with_resources(
        &self,
        resources: &[&ProtocolObject<dyn MTLResource>],
        after_stages: MTLRenderStages,
        before_stages: MTLRenderStages,
    ) {
        unsafe {
            let _: () = msg_send![
                self,
                memoryBarrierWithResources: ref_slice_as_ptr(resources),
                count: resources.len(),
                afterStages: after_stages,
                beforeStages: before_stages
            ];
        }
    }
}

impl<T: MTLRenderCommandEncoder + Message> MTLRenderCommandEncoderExt for T {}

fn checked_range(
    range: Range<usize>,
    expected_len: usize,
) -> NSRange {
    let range = NSRange::from(range);
    assert_eq!(range.length, expected_len, "binding range length must match slice length");
    range
}

#[cfg(test)]
mod tests {
    use super::checked_range;

    #[test]
    fn checked_range_accepts_matching_slice_length() {
        assert_eq!(checked_range(3..6, 3), objc2_foundation::NSRange::new(3, 3));
    }

    #[test]
    #[should_panic(expected = "binding range length must match slice length")]
    fn checked_range_rejects_mismatched_slice_length() {
        let _ = checked_range(3..6, 2);
    }
}
