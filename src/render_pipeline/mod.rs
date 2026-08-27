mod blend_factor;
mod blend_operation;
mod color_write_mask;
mod logical_to_physical_color_attachment_map;
mod mesh_render_pipeline_descriptor;
mod primitive_topology_class;
mod render_pipeline_color_attachment_descriptor;
mod render_pipeline_color_attachment_descriptor_array;
mod render_pipeline_descriptor;
mod render_pipeline_functions_descriptor;
mod render_pipeline_reflection;
mod render_pipeline_state;
mod tessellation_control_point_index_type;
mod tessellation_factor_format;
mod tessellation_factor_step_function;
mod tessellation_partition_mode;
mod tile_render_pipeline_color_attachment_descriptor;
mod tile_render_pipeline_color_attachment_descriptor_array;
mod tile_render_pipeline_descriptor;

pub use blend_factor::*;
pub use blend_operation::*;
pub use color_write_mask::*;
pub use logical_to_physical_color_attachment_map::*;
pub use mesh_render_pipeline_descriptor::*;
pub use primitive_topology_class::*;
pub use render_pipeline_color_attachment_descriptor::*;
pub use render_pipeline_color_attachment_descriptor_array::*;
pub use render_pipeline_descriptor::*;
pub use render_pipeline_functions_descriptor::*;
pub use render_pipeline_reflection::*;
pub use render_pipeline_state::*;
pub use tessellation_control_point_index_type::*;
pub use tessellation_factor_format::*;
pub use tessellation_factor_step_function::*;
pub use tessellation_partition_mode::*;
pub use tile_render_pipeline_color_attachment_descriptor::*;
pub use tile_render_pipeline_color_attachment_descriptor_array::*;
pub use tile_render_pipeline_descriptor::*;

#[cfg(test)]
mod tests {
    use core::mem::size_of;

    use objc2_foundation::NSCopying;

    use super::*;
    use crate::MTLAllocation;

    fn assert_allocation<T: MTLAllocation + ?Sized>() {}
    fn assert_copying<T: NSCopying + ?Sized>() {}
    fn assert_send_sync<T: Send + Sync + ?Sized>() {}

    #[test]
    fn xcode_27_unspecialized_values_match_the_header() {
        assert_eq!(MTLBlendFactor::Unspecialized as u64, 19);
        assert_eq!(MTLBlendOperation::Unspecialized as u64, 5);
        assert_eq!(MTLColorWriteMask::Unspecialized.bits(), 0x10);

        assert_eq!(size_of::<MTLBlendFactor>(), size_of::<usize>());
        assert_eq!(size_of::<MTLBlendOperation>(), size_of::<usize>());
        assert_eq!(size_of::<MTLColorWriteMask>(), size_of::<usize>());
        assert_eq!(size_of::<MTLPrimitiveTopologyClass>(), size_of::<usize>());
        assert_eq!(size_of::<MTLTessellationPartitionMode>(), size_of::<usize>());
        assert_eq!(size_of::<MTLTessellationFactorStepFunction>(), size_of::<usize>());
        assert_eq!(size_of::<MTLTessellationFactorFormat>(), size_of::<usize>());
        assert_eq!(size_of::<MTLTessellationControlPointIndexType>(), size_of::<usize>());
    }

    #[test]
    fn pipeline_types_expose_header_protocol_conformances() {
        assert_allocation::<dyn MTLRenderPipelineState>();
        assert_copying::<MTLLogicalToPhysicalColorAttachmentMap>();
        assert_copying::<MTLMeshRenderPipelineDescriptor>();
        assert_copying::<MTLRenderPipelineColorAttachmentDescriptor>();
        assert_copying::<MTLRenderPipelineDescriptor>();
        assert_copying::<MTLRenderPipelineFunctionsDescriptor>();
        assert_copying::<MTLTileRenderPipelineDescriptor>();
        assert_send_sync::<dyn MTLRenderPipelineState>();
        assert_send_sync::<MTLRenderPipelineReflection>();
    }
}
