pub use objc2::{rc::Retained, runtime::ProtocolObject};

#[allow(deprecated)]
pub use crate::MTLFeatureSet;
pub use crate::{
    BufferExt, MTL4CommandQueue, MTL4CommandQueueExt, MTL4CopySparseBufferMappingOperation,
    MTL4CopySparseTextureMappingOperation, MTL4UpdateSparseBufferMappingOperation,
    MTL4UpdateSparseTextureMappingOperation, MTLBindingExt, MTLBlitCommandEncoder, MTLBlitCommandEncoderExt, MTLBuffer,
    MTLCaptureDescriptor, MTLCaptureDestination, MTLCaptureManager, MTLCaptureScopeExt, MTLCommandBuffer,
    MTLCommandBufferExt, MTLCommandBufferHandler, MTLCommandBufferStatus, MTLCommandEncoder, MTLCommandEncoderExt,
    MTLCommandQueue, MTLCommandQueueExt, MTLCompareFunction, MTLComputeCommandEncoder, MTLComputeCommandEncoderExt,
    MTLComputePipelineDescriptor, MTLComputePipelineState, MTLComputePipelineStateExt, MTLCounterExt,
    MTLCounterSampleBufferExt, MTLCounterSetExt, MTLDataType, MTLDepthStencilDescriptor, MTLDevice, MTLDeviceExt,
    MTLDynamicLibraryExt, MTLEvent, MTLEventExt, MTLFunction, MTLFunctionConstantValues, MTLFunctionExt,
    MTLFunctionHandleExt, MTLGPUFamily, MTLHeap, MTLHeapDescriptor, MTLHeapExt, MTLHeapType,
    MTLIndirectCommandBufferExt, MTLLibrary, MTLLibraryExt, MTLPipelineOption, MTLPixelFormat, MTLReadWriteTextureTier,
    MTLRenderCommandEncoder, MTLRenderCommandEncoderExt, MTLRenderPassDescriptor, MTLRenderPipelineDescriptor,
    MTLRenderPipelineState, MTLResidencySetExt, MTLResource, MTLResourceExt, MTLResourceOptions, MTLSamplerDescriptor,
    MTLSize, MTLSparsePageSize, MTLSparseTextureMappingMode, MTLStorageMode, MTLTexture, MTLTextureDescriptor,
    MTLTextureUsage, MTLVertexDescriptor, MetalError, TextureExt,
};
