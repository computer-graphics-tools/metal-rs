use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::{NSObject, ProtocolObject},
};
use objc2_foundation::{CopyingHelper, NSArray, NSCopying, NSObjectProtocol, NSString, NSUInteger};

use crate::{
    MTLBinaryArchive as BinaryArchive, MTLFunction, MTLLinkedFunctions,
    MTLPipelineBufferDescriptorArray, MTLShaderValidation, MTLSize,
    MTLTileRenderPipelineColorAttachmentDescriptorArray,
    dynamic_library::MTLDynamicLibrary as DynamicLibrary,
};

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtltilerenderpipelinedescriptor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLTileRenderPipelineDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLTileRenderPipelineDescriptor {}
);

unsafe impl CopyingHelper for MTLTileRenderPipelineDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLTileRenderPipelineDescriptor {}
);

impl MTLTileRenderPipelineDescriptor {
    extern_methods!(
        /// The descriptor label.
        #[unsafe(method(label))]
        #[unsafe(method_family = none)]
        pub unsafe fn label(&self) -> Option<Retained<NSString>>;

        /// Setter for [`label`][Self::label].
        ///
        /// This is [copied][objc2_foundation::NSCopying::copy] when set.
        #[unsafe(method(setLabel:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_label(&self, label: Option<&NSString>);

        /// The kernel or fragment function that serves as the tile shader for this pipeline.
        ///
        /// Both kernel-based and fragment-based tile pipelines dispatches will barrier against previous
        /// draws and other dispatches. Kernel-based pipelines will wait until all prior access to the tile completes.
        /// Fragment-based pipelines will only wait until all prior access to the fragment's location completes.
        #[unsafe(method(tileFunction))]
        #[unsafe(method_family = none)]
        pub unsafe fn tile_function(&self) -> Retained<ProtocolObject<dyn MTLFunction>>;

        /// Setter for [`tileFunction`][Self::tileFunction].
        #[unsafe(method(setTileFunction:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_tile_function(&self, tile_function: &ProtocolObject<dyn MTLFunction>);

        #[unsafe(method(rasterSampleCount))]
        #[unsafe(method_family = none)]
        pub unsafe fn raster_sample_count(&self) -> NSUInteger;

        /// Setter for [`rasterSampleCount`][Self::rasterSampleCount].
        #[unsafe(method(setRasterSampleCount:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_raster_sample_count(&self, raster_sample_count: NSUInteger);

        #[unsafe(method(colorAttachments))]
        #[unsafe(method_family = none)]
        pub unsafe fn color_attachments(
            &self,
        ) -> Retained<MTLTileRenderPipelineColorAttachmentDescriptorArray>;

        /// Whether all threadgroups associated with this pipeline will cover tiles entirely.
        ///
        /// Metal can optimize code generation for this case.
        #[unsafe(method(threadgroupSizeMatchesTileSize))]
        #[unsafe(method_family = none)]
        pub unsafe fn threadgroup_size_matches_tile_size(&self) -> bool;

        /// Setter for [`threadgroupSizeMatchesTileSize`][Self::threadgroupSizeMatchesTileSize].
        #[unsafe(method(setThreadgroupSizeMatchesTileSize:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_threadgroup_size_matches_tile_size(
            &self,
            threadgroup_size_matches_tile_size: bool,
        );

        #[unsafe(method(tileBuffers))]
        #[unsafe(method_family = none)]
        pub unsafe fn tile_buffers(&self) -> Retained<MTLPipelineBufferDescriptorArray>;

        /// Optional property. Set the maxTotalThreadsPerThreadgroup. If it is not set, returns zero.
        #[unsafe(method(maxTotalThreadsPerThreadgroup))]
        #[unsafe(method_family = none)]
        pub unsafe fn max_total_threads_per_threadgroup(&self) -> NSUInteger;

        /// Setter for [`maxTotalThreadsPerThreadgroup`][Self::maxTotalThreadsPerThreadgroup].
        #[unsafe(method(setMaxTotalThreadsPerThreadgroup:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_max_total_threads_per_threadgroup(
            &self,
            max_total_threads_per_threadgroup: NSUInteger,
        );

        /// The set of MTLBinaryArchive to search for compiled code when creating the pipeline state.
        ///
        /// Accelerate pipeline state creation by providing archives of compiled code such that no compilation needs to happen on the fast path.
        ///
        /// See: MTLBinaryArchive
        #[unsafe(method(binaryArchives))]
        #[unsafe(method_family = none)]
        pub unsafe fn binary_archives(
            &self,
        ) -> Option<Retained<NSArray<ProtocolObject<dyn BinaryArchive>>>>;

        /// Setter for [`binaryArchives`][Self::binaryArchives].
        ///
        /// This is [copied][objc2_foundation::NSCopying::copy] when set.
        #[unsafe(method(setBinaryArchives:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_binary_archives(
            &self,
            binary_archives: Option<&NSArray<ProtocolObject<dyn BinaryArchive>>>,
        );

        /// The set of MTLDynamicLibrary to use to resolve external symbols before considering symbols from dependent MTLDynamicLibrary.
        ///
        /// Typical workflows use the libraries property of MTLCompileOptions to record dependent libraries at compile time without having to use preloadedLibraries.
        /// This property can be used to override symbols from dependent libraries for experimentation or evaluating alternative implementations.
        /// It can also be used to provide dynamic libraries that are dynamically created (for example, from source) that have no stable installName that can be used to automatically load from the file system.
        ///
        /// See: MTLDynamicLibrary
        #[unsafe(method(preloadedLibraries))]
        #[unsafe(method_family = none)]
        pub unsafe fn preloaded_libraries(
            &self,
        ) -> Retained<NSArray<ProtocolObject<dyn DynamicLibrary>>>;

        /// Setter for [`preloadedLibraries`][Self::preloadedLibraries].
        ///
        /// This is [copied][objc2_foundation::NSCopying::copy] when set.
        #[unsafe(method(setPreloadedLibraries:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_preloaded_libraries(
            &self,
            preloaded_libraries: &NSArray<ProtocolObject<dyn DynamicLibrary>>,
        );

        /// The set of functions to be linked with the pipeline state and accessed from the tile function.
        ///
        /// See: MTLLinkedFunctions
        #[unsafe(method(linkedFunctions))]
        #[unsafe(method_family = none)]
        pub unsafe fn linked_functions(&self) -> Retained<MTLLinkedFunctions>;

        /// Setter for [`linkedFunctions`][Self::linkedFunctions].
        ///
        /// This is [copied][objc2_foundation::NSCopying::copy] when set.
        #[unsafe(method(setLinkedFunctions:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_linked_functions(&self, linked_functions: Option<&MTLLinkedFunctions>);

        /// This flag makes this pipeline support creating a new pipeline by adding binary functions.
        #[unsafe(method(supportAddingBinaryFunctions))]
        #[unsafe(method_family = none)]
        pub unsafe fn support_adding_binary_functions(&self) -> bool;

        /// Setter for [`supportAddingBinaryFunctions`][Self::supportAddingBinaryFunctions].
        #[unsafe(method(setSupportAddingBinaryFunctions:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_support_adding_binary_functions(&self, support_adding_binary_functions: bool);

        /// The maximum depth of the call stack in stack frames from the tile function. Defaults to 1 additional stack frame.
        #[unsafe(method(maxCallStackDepth))]
        #[unsafe(method_family = none)]
        pub unsafe fn max_call_stack_depth(&self) -> NSUInteger;

        /// Setter for [`maxCallStackDepth`][Self::maxCallStackDepth].
        #[unsafe(method(setMaxCallStackDepth:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_max_call_stack_depth(&self, max_call_stack_depth: NSUInteger);

        #[unsafe(method(reset))]
        #[unsafe(method_family = none)]
        pub unsafe fn reset(&self);

        /// Toggle that determines whether Metal Shader Validation should be enabled or disabled for the pipeline.
        ///
        /// The value can be overridden using `MTL_SHADER_VALIDATION_ENABLE_PIPELINES` or `MTL_SHADER_VALIDATION_DISABLE_PIPELINES` Environment Variables.
        #[unsafe(method(shaderValidation))]
        #[unsafe(method_family = none)]
        pub unsafe fn shader_validation(&self) -> MTLShaderValidation;

        /// Setter for [`shaderValidation`][Self::shaderValidation].
        #[unsafe(method(setShaderValidation:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_shader_validation(&self, shader_validation: MTLShaderValidation);

        /// Sets the required threads-per-threadgroup during tile dispatches. The `threadsPerTile` argument of any tile dispatch must match to this value if it is set.
        /// Optional, unless the pipeline is going to use CooperativeTensors in which case this must be set.
        /// Setting this to a size of 0 in every dimension disables this property
        #[unsafe(method(requiredThreadsPerThreadgroup))]
        #[unsafe(method_family = none)]
        pub unsafe fn required_threads_per_threadgroup(&self) -> MTLSize;

        /// Setter for [`requiredThreadsPerThreadgroup`][Self::requiredThreadsPerThreadgroup].
        #[unsafe(method(setRequiredThreadsPerThreadgroup:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_required_threads_per_threadgroup(
            &self,
            required_threads_per_threadgroup: MTLSize,
        );
    );
}

/// Methods declared on superclass `NSObject`.
impl MTLTileRenderPipelineDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
