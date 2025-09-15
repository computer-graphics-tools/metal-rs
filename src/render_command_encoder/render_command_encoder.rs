use core::ptr::NonNull;
use objc2::{extern_protocol, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSObjectProtocol, NSRange};

use super::{
    CullMode, DepthClipMode, PrimitiveType, ScissorRect, TriangleFillMode,
    VertexAmplificationViewMapping, Viewport, VisibilityResultMode,
};
use crate::command_encoder::{BarrierScope as MTLBarrierScope, Stages as MTLRenderStages};
use crate::render_pipeline::{
    LogicalToPhysicalColorAttachmentMap as MTLLogicalToPhysicalColorAttachmentMap,
    RenderPipelineState,
};
use crate::texture::Texture as MTLTexture;
use crate::types::{IndexType as MTLIndexType, ResourceID, Size as MTLSize};
use crate::{
    AccelerationStructure, Buffer, CommandEncoder, CounterSampleBuffer, Fence,
    IntersectionFunctionTable, RasterizationRateMap, Resource, ResourceUsage, SamplerState,
    VisibleFunctionTable,
};

extern_protocol!(
    /// Render command encoder interface.
    #[name = "MTLRenderCommandEncoder"]
    pub unsafe trait RenderCommandEncoder: CommandEncoder {
        #[unsafe(method(setRenderPipelineState:))]
        #[unsafe(method_family = none)]
        fn set_render_pipeline_state(
            &self,
            pipeline_state: &ProtocolObject<dyn RenderPipelineState>,
        );

        #[unsafe(method(setViewport:))]
        #[unsafe(method_family = none)]
        fn set_viewport(&self, viewport: Viewport);

        #[unsafe(method(setViewports:count:))]
        #[unsafe(method_family = none)]
        unsafe fn set_viewports(&self, viewports: NonNull<Viewport>, count: usize);

        #[unsafe(method(setFrontFacingWinding:))]
        #[unsafe(method_family = none)]
        fn set_front_facing_winding(&self, winding: super::types::Winding);

        #[unsafe(method(setCullMode:))]
        #[unsafe(method_family = none)]
        fn set_cull_mode(&self, cull_mode: CullMode);

        #[unsafe(method(setDepthClipMode:))]
        #[unsafe(method_family = none)]
        fn set_depth_clip_mode(&self, mode: DepthClipMode);

        #[unsafe(method(setTriangleFillMode:))]
        #[unsafe(method_family = none)]
        fn set_triangle_fill_mode(&self, mode: TriangleFillMode);

        #[unsafe(method(setScissorRect:))]
        #[unsafe(method_family = none)]
        fn set_scissor_rect(&self, rect: ScissorRect);

        #[unsafe(method(setScissorRects:count:))]
        #[unsafe(method_family = none)]
        unsafe fn set_scissor_rects(&self, rects: NonNull<ScissorRect>, count: usize);

        #[unsafe(method(setBlendColorRed:green:blue:alpha:))]
        #[unsafe(method_family = none)]
        fn set_blend_color(&self, red: f32, green: f32, blue: f32, alpha: f32);

        #[unsafe(method(setStencilReferenceValue:))]
        #[unsafe(method_family = none)]
        fn set_stencil_reference_value(&self, reference_value: u32);

        #[unsafe(method(setStencilFrontReferenceValue:backReferenceValue:))]
        #[unsafe(method_family = none)]
        fn set_stencil_front_back_reference_value(&self, front: u32, back: u32);

        #[unsafe(method(setVisibilityResultMode:offset:))]
        #[unsafe(method_family = none)]
        fn set_visibility_result_mode(&self, mode: VisibilityResultMode, offset: usize);

        #[unsafe(method(setColorAttachmentMap:))]
        #[unsafe(method_family = none)]
        unsafe fn set_color_attachment_map(
            &self,
            mapping: Option<&MTLLogicalToPhysicalColorAttachmentMap>,
        );
    }
);
