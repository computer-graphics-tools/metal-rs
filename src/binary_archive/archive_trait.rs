use objc2::{extern_protocol, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSError, NSErrorDomain, NSObjectProtocol, NSString, NSURL};

use crate::function_stitching::StitchedLibraryDescriptor;
use crate::{ComputePipelineDescriptor, FunctionDescriptor, Library, RenderPipelineDescriptor};

extern "C" {
    pub static MTLBinaryArchiveDomain: &'static NSErrorDomain;
}

extern_protocol!(
    /// A container of pipeline state descriptors and their associated compiled code.
    #[name = "MTLBinaryArchive"]
    pub unsafe trait BinaryArchive: NSObjectProtocol {
        /// A string to help identify this object.
        #[unsafe(method(label))]
        #[unsafe(method_family = none)]
        fn label(&self) -> Option<Retained<NSString>>;

        /// Setter for `label`.
        #[unsafe(method(setLabel:))]
        #[unsafe(method_family = none)]
        fn set_label(&self, label: Option<&NSString>);

        /// The device this resource was created against.
        #[unsafe(method(device))]
        #[unsafe(method_family = none)]
        fn device(&self) -> Retained<ProtocolObject<dyn crate::Device>>;

        /// Add compute pipeline functions to the archive.
        #[unsafe(method(addComputePipelineFunctionsWithDescriptor:error:_))]
        #[unsafe(method_family = none)]
        fn add_compute_pipeline_functions(
            &self,
            descriptor: &ComputePipelineDescriptor,
        ) -> Result<(), Retained<NSError>>;

        /// Add render pipeline functions to the archive.
        #[unsafe(method(addRenderPipelineFunctionsWithDescriptor:error:_))]
        #[unsafe(method_family = none)]
        fn add_render_pipeline_functions(
            &self,
            descriptor: &RenderPipelineDescriptor,
        ) -> Result<(), Retained<NSError>>;

        /// Add tile render pipeline functions to the archive.
        #[unsafe(method(addTileRenderPipelineFunctionsWithDescriptor:error:_))]
        #[unsafe(method_family = none)]
        unsafe fn add_tile_render_pipeline_functions(
            &self,
            descriptor: &crate::mtl4::MTL4TileRenderPipeline,
        ) -> Result<(), Retained<NSError>>;

        /// Add mesh render pipeline functions to the archive.
        #[unsafe(method(addMeshRenderPipelineFunctionsWithDescriptor:error:_))]
        #[unsafe(method_family = none)]
        unsafe fn add_mesh_render_pipeline_functions(
            &self,
            descriptor: &crate::mtl4::MTL4RenderPipeline,
        ) -> Result<(), Retained<NSError>>;

        /// Add the function(s) from a stitched library to the archive.
        #[unsafe(method(addLibraryWithDescriptor:error:_))]
        #[unsafe(method_family = none)]
        unsafe fn add_library_with_descriptor(
            &self,
            descriptor: &StitchedLibraryDescriptor,
        ) -> Result<(), Retained<NSError>>;

        /// Write the contents of a BinaryArchive to a file.
        #[unsafe(method(serializeToURL:error:_))]
        #[unsafe(method_family = none)]
        fn serialize_to_url(&self, url: &NSURL) -> Result<(), Retained<NSError>>;

        /// Add a `visible` or `intersection` function to the archive.
        #[unsafe(method(addFunctionWithDescriptor:library:error:_))]
        #[unsafe(method_family = none)]
        unsafe fn add_function_with_descriptor_library(
            &self,
            descriptor: &FunctionDescriptor,
            library: &ProtocolObject<dyn Library>,
        ) -> Result<(), Retained<NSError>>;
    }
);
