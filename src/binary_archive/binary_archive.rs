use objc2::{extern_protocol, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSError, NSErrorDomain, NSObjectProtocol, NSString, NSURL};

use crate::{
    MTLComputePipelineDescriptor, MTLFunctionDescriptor, MTLLibrary, MTLRenderPipelineDescriptor,
    function_stitching::MTLStitchedLibraryDescriptor,
};

#[allow(unused)]
unsafe extern "C" {
    pub static MTLBinaryArchiveDomain: &'static NSErrorDomain;
}

extern_protocol!(
    /// A container of pipeline state descriptors and their associated compiled code.
    pub unsafe trait MTLBinaryArchive: NSObjectProtocol {
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
        fn device(&self) -> Retained<ProtocolObject<dyn crate::MTLDevice>>;

        /// Add compute pipeline functions to the archive.
        #[unsafe(method(addComputePipelineFunctionsWithDescriptor:error:_))]
        #[unsafe(method_family = none)]
        fn add_compute_pipeline_functions(
            &self,
            descriptor: &MTLComputePipelineDescriptor,
        ) -> Result<(), Retained<NSError>>;

        /// Add render pipeline functions to the archive.
        #[unsafe(method(addRenderPipelineFunctionsWithDescriptor:error:_))]
        #[unsafe(method_family = none)]
        fn add_render_pipeline_functions(
            &self,
            descriptor: &MTLRenderPipelineDescriptor,
        ) -> Result<(), Retained<NSError>>;

        // Tile and mesh render pipeline variants omitted for now.

        /// Add the function(s) from a stitched library to the archive.
        #[unsafe(method(addLibraryWithDescriptor:error:_))]
        #[unsafe(method_family = none)]
        unsafe fn add_library_with_descriptor(
            &self,
            descriptor: &MTLStitchedLibraryDescriptor,
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
            descriptor: &MTLFunctionDescriptor,
            library: &ProtocolObject<dyn MTLLibrary>,
        ) -> Result<(), Retained<NSError>>;
    }
);
