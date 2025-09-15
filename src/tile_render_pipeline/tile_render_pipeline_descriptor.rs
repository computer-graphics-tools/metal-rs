use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::{NSObject, ProtocolObject},
};
use objc2_foundation::{CopyingHelper, NSArray, NSCopying, NSObjectProtocol, NSString};

use super::TileRenderPipelineColorAttachmentDescriptorArray;
use crate::{
    BinaryArchive,
    DynamicLibrary,
    LinkedFunctions,
    PipelineBufferDescriptorArray,
    types::Size as MTLSize,
};
use crate::library::Function;

extern_class!(
    /// Descriptor for creating a `RenderPipelineState` for tile shaders.
    #[unsafe(super(NSObject))]
    #[name = "MTLTileRenderPipelineDescriptor"]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct TileRenderPipelineDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for TileRenderPipelineDescriptor {}
);

unsafe impl CopyingHelper for TileRenderPipelineDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for TileRenderPipelineDescriptor {}
);

impl TileRenderPipelineDescriptor {
    extern_methods!(
        #[unsafe(method(label))]
        #[unsafe(method_family = none)]
        pub fn label(&self) -> Option<Retained<NSString>>;

        #[unsafe(method(setLabel:))]
        #[unsafe(method_family = none)]
        pub fn set_label(&self, label: Option<&NSString>);

        /// Kernel or fragment function that serves as the tile shader.
        #[unsafe(method(tileFunction))]
        #[unsafe(method_family = none)]
        pub fn tile_function(&self) -> Option<Retained<ProtocolObject<dyn Function>>>;

        #[unsafe(method(setTileFunction:))]
        #[unsafe(method_family = none)]
        pub fn set_tile_function(&self, func: Option<&ProtocolObject<dyn Function>>);

        /// Raster sample count.
        #[unsafe(method(rasterSampleCount))]
        #[unsafe(method_family = none)]
        pub fn raster_sample_count(&self) -> usize;

        #[unsafe(method(setRasterSampleCount:))]
        #[unsafe(method_family = none)]
        pub fn set_raster_sample_count(&self, value: usize);

        /// Color attachments array.
        #[unsafe(method(colorAttachments))]
        #[unsafe(method_family = none)]
        pub fn color_attachments(&self) -> Retained<TileRenderPipelineColorAttachmentDescriptorArray>;

        /// Whether threadgroup size matches tile size.
        #[unsafe(method(threadgroupSizeMatchesTileSize))]
        #[unsafe(method_family = none)]
        pub fn threadgroup_size_matches_tile_size(&self) -> bool;

        #[unsafe(method(setThreadgroupSizeMatchesTileSize:))]
        #[unsafe(method_family = none)]
        pub fn set_threadgroup_size_matches_tile_size(&self, v: bool);

        /// Optional: pipeline tile buffers.
        #[unsafe(method(tileBuffers))]
        #[unsafe(method_family = none)]
        pub fn tile_buffers(&self) -> Retained<PipelineBufferDescriptorArray>;

        /// Optional property. If not set, returns zero.
        #[unsafe(method(maxTotalThreadsPerThreadgroup))]
        #[unsafe(method_family = none)]
        pub fn max_total_threads_per_threadgroup(&self) -> usize;

        #[unsafe(method(setMaxTotalThreadsPerThreadgroup:))]
        #[unsafe(method_family = none)]
        pub fn set_max_total_threads_per_threadgroup(&self, value: usize);

        /// Archives to accelerate pipeline creation.
        #[unsafe(method(binaryArchives))]
        #[unsafe(method_family = none)]
        pub fn binary_archives(&self) -> Option<Retained<NSArray<ProtocolObject<dyn BinaryArchive>>>>;

        /// Copied when set.
        #[unsafe(method(setBinaryArchives:))]
        #[unsafe(method_family = none)]
        pub fn set_binary_archives(
            &self,
            archives: Option<&NSArray<ProtocolObject<dyn BinaryArchive>>>,
        );

        /// Preloaded dynamic libraries.
        #[unsafe(method(preloadedLibraries))]
        #[unsafe(method_family = none)]
        pub fn preloaded_libraries(&self) -> Retained<NSArray<ProtocolObject<dyn DynamicLibrary>>>;

        /// Copied when set.
        #[unsafe(method(setPreloadedLibraries:))]
        #[unsafe(method_family = none)]
        pub fn set_preloaded_libraries(
            &self,
            libs: &NSArray<ProtocolObject<dyn DynamicLibrary>>,
        );

        /// Functions to be linked with the pipeline.
        #[unsafe(method(linkedFunctions))]
        #[unsafe(method_family = none)]
        pub fn linked_functions(&self) -> Retained<LinkedFunctions>;

        /// Copied when set.
        #[unsafe(method(setLinkedFunctions:))]
        #[unsafe(method_family = none)]
        pub fn set_linked_functions(&self, funcs: Option<&LinkedFunctions>);

        /// Support adding binary functions.
        #[unsafe(method(supportAddingBinaryFunctions))]
        #[unsafe(method_family = none)]
        pub fn support_adding_binary_functions(&self) -> bool;

        #[unsafe(method(setSupportAddingBinaryFunctions:))]
        #[unsafe(method_family = none)]
        pub fn set_support_adding_binary_functions(&self, v: bool);

        /// Max call stack depth.
        #[unsafe(method(maxCallStackDepth))]
        #[unsafe(method_family = none)]
        pub fn max_call_stack_depth(&self) -> usize;

        #[unsafe(method(setMaxCallStackDepth:))]
        #[unsafe(method_family = none)]
        pub fn set_max_call_stack_depth(&self, v: usize);

        /// Required threads per threadgroup for tile dispatches.
        #[unsafe(method(requiredThreadsPerThreadgroup))]
        #[unsafe(method_family = none)]
        pub unsafe fn required_threads_per_threadgroup(&self) -> MTLSize;

        #[unsafe(method(setRequiredThreadsPerThreadgroup:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_required_threads_per_threadgroup(&self, v: MTLSize);

        /// Reset to default state.
        #[unsafe(method(reset))]
        #[unsafe(method_family = none)]
        pub fn reset(&self);
    );
}

impl TileRenderPipelineDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}


