use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::{NSObject, ProtocolObject},
};
use objc2_foundation::{CopyingHelper, NSArray, NSCopying, NSObjectProtocol, NSString};

use super::{PrimitiveTopologyClass, RenderPipelineColorAttachmentDescriptorArray};
use crate::{PipelineBufferDescriptorArray, PixelFormat, ShaderValidation, VertexDescriptor};

extern_class!(
    /// Descriptor for creating a `RenderPipelineState`.
    #[unsafe(super(NSObject))]
    #[name = "MTLRenderPipelineDescriptor"]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct RenderPipelineDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for RenderPipelineDescriptor {}
);

unsafe impl CopyingHelper for RenderPipelineDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for RenderPipelineDescriptor {}
);

impl RenderPipelineDescriptor {
    extern_methods!(
        #[unsafe(method(label))]
        #[unsafe(method_family = none)]
        pub fn label(&self) -> Option<Retained<NSString>>;

        #[unsafe(method(setLabel:))]
        #[unsafe(method_family = none)]
        pub fn set_label(&self, label: Option<&NSString>);

        #[unsafe(method(rasterSampleCount))]
        #[unsafe(method_family = none)]
        pub fn raster_sample_count(&self) -> usize;

        #[unsafe(method(setRasterSampleCount:))]
        #[unsafe(method_family = none)]
        pub fn set_raster_sample_count(&self, raster_sample_count: usize);

        #[unsafe(method(isAlphaToCoverageEnabled))]
        #[unsafe(method_family = none)]
        pub fn is_alpha_to_coverage_enabled(&self) -> bool;

        #[unsafe(method(setAlphaToCoverageEnabled:))]
        #[unsafe(method_family = none)]
        pub fn set_alpha_to_coverage_enabled(&self, enabled: bool);

        #[unsafe(method(isAlphaToOneEnabled))]
        #[unsafe(method_family = none)]
        pub fn is_alpha_to_one_enabled(&self) -> bool;

        #[unsafe(method(setAlphaToOneEnabled:))]
        #[unsafe(method_family = none)]
        pub fn set_alpha_to_one_enabled(&self, enabled: bool);

        #[unsafe(method(isRasterizationEnabled))]
        #[unsafe(method_family = none)]
        pub fn is_rasterization_enabled(&self) -> bool;

        #[unsafe(method(setRasterizationEnabled:))]
        #[unsafe(method_family = none)]
        pub fn set_rasterization_enabled(&self, enabled: bool);

        #[unsafe(method(maxVertexAmplificationCount))]
        #[unsafe(method_family = none)]
        pub fn max_vertex_amplification_count(&self) -> usize;

        #[unsafe(method(setMaxVertexAmplificationCount:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_max_vertex_amplification_count(&self, value: usize);

        #[unsafe(method(colorAttachments))]
        #[unsafe(method_family = none)]
        pub fn color_attachments(&self) -> Retained<RenderPipelineColorAttachmentDescriptorArray>;

        #[unsafe(method(depthAttachmentPixelFormat))]
        #[unsafe(method_family = none)]
        pub fn depth_attachment_pixel_format(&self) -> PixelFormat;

        #[unsafe(method(setDepthAttachmentPixelFormat:))]
        #[unsafe(method_family = none)]
        pub fn set_depth_attachment_pixel_format(&self, fmt: PixelFormat);

        #[unsafe(method(stencilAttachmentPixelFormat))]
        #[unsafe(method_family = none)]
        pub fn stencil_attachment_pixel_format(&self) -> PixelFormat;

        #[unsafe(method(setStencilAttachmentPixelFormat:))]
        #[unsafe(method_family = none)]
        pub fn set_stencil_attachment_pixel_format(&self, fmt: PixelFormat);

        #[unsafe(method(inputPrimitiveTopology))]
        #[unsafe(method_family = none)]
        pub fn input_primitive_topology(&self) -> PrimitiveTopologyClass;

        #[unsafe(method(setInputPrimitiveTopology:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_input_primitive_topology(&self, topo: PrimitiveTopologyClass);

        #[unsafe(method(vertexDescriptor))]
        #[unsafe(method_family = none)]
        pub fn vertex_descriptor(&self) -> Option<Retained<VertexDescriptor>>;

        #[unsafe(method(setVertexDescriptor:))]
        #[unsafe(method_family = none)]
        pub fn set_vertex_descriptor(&self, vertex_descriptor: Option<&VertexDescriptor>);

        #[unsafe(method(supportIndirectCommandBuffers))]
        #[unsafe(method_family = none)]
        pub fn support_indirect_command_buffers(&self) -> bool;

        #[unsafe(method(setSupportIndirectCommandBuffers:))]
        #[unsafe(method_family = none)]
        pub fn set_support_indirect_command_buffers(&self, enabled: bool);

        #[unsafe(method(reset))]
        #[unsafe(method_family = none)]
        pub fn reset(&self);

        #[unsafe(method(shaderValidation))]
        #[unsafe(method_family = none)]
        pub unsafe fn shader_validation(&self) -> ShaderValidation;

        #[unsafe(method(setShaderValidation:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_shader_validation(&self, v: ShaderValidation);
    );
}

impl RenderPipelineDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
