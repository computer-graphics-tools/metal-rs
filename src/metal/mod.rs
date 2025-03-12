//! Rust bindings for Apple's Metal framework.
//!
//! This module provides safe Rust wrappers around core Metal types needed for
//! graphics and compute programming on Apple platforms.
//!
//! # Design
//!
//! The implementation follows a dual-type pattern for each Objective-C class:
//! - An owned type (e.g., `MTLDevice`) with ownership semantics
//! - A borrowed reference type (e.g., `MTLDeviceRef`) for borrowed references
//!
//! # Example
//!
//! ```no_run
//! use metal_rs::metal::{MTLCreateSystemDefaultDevice, MTLClearColor};
//! 
//! // Get the default Metal device
//! let device = MTLCreateSystemDefaultDevice();
//! 
//! // Create a command queue
//! let queue = device.new_command_queue();
//! 
//! // Create a command buffer
//! let command_buffer = queue.new_command_buffer();
//! 
//! // Red clear color
//! let clear_color = MTLClearColor::new(1.0, 0.0, 0.0, 1.0);
//! ```

pub mod device;
pub mod command_queue;
pub mod command_buffer;
pub mod command_encoder;
pub mod types;
pub mod resource;
pub mod buffer;
pub mod texture;
pub mod library;
pub mod depth_stencil;
pub mod blit_command_encoder;
pub mod compute_command_encoder;
pub mod render_command_encoder;
pub mod parallel_render_command_encoder;
pub mod sampler;
pub mod render_pipeline;
pub mod vertex_descriptor;
pub mod fence;
pub mod drawable;
pub mod heap;
pub mod argument;
pub mod argument_encoder;
pub mod render_pass;
pub mod compute_pass;
pub mod blit_pass;
pub mod event;
pub mod resource_state_command_encoder;
pub mod pipeline;
pub mod compute_pipeline;
pub mod stage_input_output_descriptor;
pub mod function_descriptor;
pub mod allocation;
pub mod residency_set;
pub mod rasterization_rate;
pub mod linked_functions;
pub mod counters;
pub mod function_handle;
pub mod function_log;
pub mod function_stitching;
pub mod indirect_command_buffer;
pub mod indirect_command_encoder;
pub mod binary_archive;
pub mod dynamic_library;
pub mod log;
pub mod visible_function_table;
pub mod intersection_function_table;
pub mod acceleration_structure;
pub mod acceleration_structure_command_encoder;
pub mod acceleration_structure_types;
pub mod capture_manager;
pub mod log_state;
pub mod io_command_buffer;
pub mod io_command_queue;
pub mod io_compressor;

// Re-export types for public API
pub use device::{MTLDevice, MTLDeviceRef, MTLCreateSystemDefaultDevice, MTLFeatureSet};
pub use command_queue::{MTLCommandQueue, MTLCommandQueueRef};
pub use command_buffer::{MTLCommandBuffer, MTLCommandBufferRef, MTLCommandBufferStatus};
pub use command_encoder::{MTLCommandEncoder, MTLCommandEncoderRef, CommandEncoder};
pub use types::{MTLPixelFormat};
pub use texture::{MTLSize, MTLOrigin, MTLRegion};
pub use buffer::{MTLBuffer, MTLBufferRef};
pub use texture::{MTLTexture, MTLTextureRef, MTLTextureType, MTLTextureUsage, MTLTextureDescriptor, MTLTextureDescriptorRef};
pub use library::{
    MTLLibrary, MTLLibraryRef, 
    MTLFunction, MTLFunctionRef, 
    MTLFunctionType,
    MTLCompileOptions, 
    MTLFunctionConstantValues, MTLFunctionConstantValuesRef
};
pub use function_handle::{MTLFunctionHandle, MTLFunctionHandleRef};
pub use function_log::{
    MTLFunctionLog, MTLFunctionLogRef,
    MTLFunctionLogDebugLocation, MTLFunctionLogDebugLocationRef,
    MTLFunctionLogType
};
pub use log::{
    MTLLog, MTLLogRef,
    MTLLogContainer, MTLLogContainerRef,
    MTLLogType
};
pub use depth_stencil::{
    MTLCompareFunction, MTLStencilOperation,
    MTLStencilDescriptor, MTLStencilDescriptorRef,
    MTLDepthStencilDescriptor, MTLDepthStencilDescriptorRef,
    MTLDepthStencilState, MTLDepthStencilStateRef
};
pub use blit_command_encoder::{
    MTLBlitCommandEncoder, MTLBlitCommandEncoderRef, MTLBlitOption
};
pub use resource::{
    MTLResource, MTLResourceRef,
    MTLPurgeableState, MTLCPUCacheMode, MTLStorageMode, MTLHazardTrackingMode, MTLResourceOptions,
};
pub use compute_command_encoder::{
    MTLComputeCommandEncoder, MTLComputeCommandEncoderRef,
    MTLComputePipelineState, MTLComputePipelineStateRef,
    MTLDispatchType, MTLResourceUsage, MTLBarrierScope,
    MTLDispatchThreadgroupsIndirectArguments, MTLStageInRegionIndirectArguments
};
pub use sampler::{
    MTLSamplerState, MTLSamplerStateRef,
    MTLSamplerDescriptor, MTLSamplerDescriptorRef,
    MTLSamplerMinMagFilter, MTLSamplerMipFilter,
    MTLSamplerAddressMode, MTLSamplerBorderColor
};
pub use render_pipeline::{
    MTLColorWriteMask, MTLBlendFactor, MTLBlendOperation, MTLPrimitiveTopologyClass, MTLPipelineOption,
    MTLRenderPipelineColorAttachmentDescriptor, MTLRenderPipelineColorAttachmentDescriptorRef,
    MTLRenderPipelineColorAttachmentDescriptorArray, MTLRenderPipelineColorAttachmentDescriptorArrayRef,
    MTLRenderPipelineDescriptor, MTLRenderPipelineDescriptorRef,
    MTLRenderPipelineState, MTLRenderPipelineStateRef,
    MTLRenderPipelineReflection, MTLRenderPipelineReflectionRef
};
pub use vertex_descriptor::{
    MTLVertexFormat, MTLVertexStepFunction,
    MTLVertexBufferLayoutDescriptor, MTLVertexBufferLayoutDescriptorRef,
    MTLVertexBufferLayoutDescriptorArray, MTLVertexBufferLayoutDescriptorArrayRef,
    MTLVertexAttributeDescriptor, MTLVertexAttributeDescriptorRef,
    MTLVertexAttributeDescriptorArray, MTLVertexAttributeDescriptorArrayRef,
    MTLVertexDescriptor, MTLVertexDescriptorRef,
    BUFFER_LAYOUT_STRIDE_DYNAMIC
};
pub use fence::{
    MTLFence, MTLFenceRef
};
pub use drawable::{
    MTLDrawable, MTLDrawableRef
};
pub use heap::{
    MTLHeapType, MTLSparsePageSize,
    MTLHeapDescriptor, MTLHeapDescriptorRef,
    MTLHeap, MTLHeapRef
};
pub use argument::{
    MTLDataType, MTLBindingType, MTLArgumentType, MTLBindingAccess,
    MTLArgumentDescriptor, MTLArgumentDescriptorRef,
    MTLType, MTLTypeRef,
    MTLStructMember, MTLStructMemberRef,
    MTLStructType, MTLStructTypeRef,
    MTLArrayType, MTLArrayTypeRef,
    MTLPointerType, MTLPointerTypeRef,
    MTLTextureReferenceType, MTLTextureReferenceTypeRef,
    MTLArgument, MTLArgumentRef,
    MTLBinding, MTLBindingRef,
    MTLBufferBinding, MTLBufferBindingRef,
    MTLThreadgroupBinding, MTLThreadgroupBindingRef,
    MTLTextureBinding, MTLTextureBindingRef,
    MTLObjectPayloadBinding, MTLObjectPayloadBindingRef
};
pub use argument_encoder::{
    MTLArgumentEncoder, MTLArgumentEncoderRef,
    ATTRIBUTE_STRIDE_STATIC
};
pub use render_pass::{
    MTLLoadAction, MTLStoreAction, MTLStoreActionOptions, MTLClearColor,
    MTLMultisampleDepthResolveFilter, MTLMultisampleStencilResolveFilter,
    MTLRenderPassDescriptor, MTLRenderPassDescriptorRef,
    MTLRenderPassColorAttachmentDescriptor, MTLRenderPassColorAttachmentDescriptorRef,
    MTLRenderPassColorAttachmentDescriptorArray, MTLRenderPassColorAttachmentDescriptorArrayRef,
    MTLRenderPassDepthAttachmentDescriptor, MTLRenderPassDepthAttachmentDescriptorRef,
    MTLRenderPassStencilAttachmentDescriptor, MTLRenderPassStencilAttachmentDescriptorRef
};
pub use render_command_encoder::{
    MTLRenderCommandEncoder, MTLRenderCommandEncoderRef,
    MTLPrimitiveType, MTLWinding, MTLCullMode, MTLDepthClipMode,
    MTLTriangleFillMode, MTLViewport, MTLScissorRect, MTLIndexType
};
pub use parallel_render_command_encoder::{
    MTLParallelRenderCommandEncoder, MTLParallelRenderCommandEncoderRef
};
pub use event::{
    MTLEvent, MTLEventRef,
    MTLSharedEvent, MTLSharedEventRef,
    MTLSharedEventHandle, MTLSharedEventHandleRef,
    MTLSharedEventListener, MTLSharedEventListenerRef
};
pub use compute_pass::{
    MTLComputePassDescriptor, MTLComputePassDescriptorRef,
    MTLComputePassSampleBufferAttachmentDescriptor, MTLComputePassSampleBufferAttachmentDescriptorRef,
    MTLComputePassSampleBufferAttachmentDescriptorArray, MTLComputePassSampleBufferAttachmentDescriptorArrayRef
};
pub use blit_pass::{
    MTLBlitPassDescriptor, MTLBlitPassDescriptorRef,
    MTLBlitPassSampleBufferAttachmentDescriptor, MTLBlitPassSampleBufferAttachmentDescriptorRef,
    MTLBlitPassSampleBufferAttachmentDescriptorArray, MTLBlitPassSampleBufferAttachmentDescriptorArrayRef
};
pub use resource_state_command_encoder::{
    MTLResourceStateCommandEncoder, MTLResourceStateCommandEncoderRef,
    MTLSparseTextureMappingMode, MTLSparseTextureRegionUpdateIndirectArguments
};
pub use pipeline::{
    MTLMutability,
    MTLPipelineBufferDescriptor, MTLPipelineBufferDescriptorRef,
    MTLPipelineBufferDescriptorArray, MTLPipelineBufferDescriptorArrayRef
};
pub use compute_pipeline::{
    MTLComputePipelineDescriptor, MTLComputePipelineDescriptorRef
};
pub use stage_input_output_descriptor::{
    MTLAttributeFormat, MTLStepFunction,
    MTLBufferLayoutDescriptor, MTLBufferLayoutDescriptorRef,
    MTLBufferLayoutDescriptorArray, MTLBufferLayoutDescriptorArrayRef,
    MTLAttributeDescriptor, MTLAttributeDescriptorRef,
    MTLAttributeDescriptorArray, MTLAttributeDescriptorArrayRef,
    MTLStageInputOutputDescriptor, MTLStageInputOutputDescriptorRef
};
pub use function_descriptor::{
    MTLFunctionOptions,
    MTLFunctionDescriptor, MTLFunctionDescriptorRef,
    MTLIntersectionFunctionDescriptor, MTLIntersectionFunctionDescriptorRef
};
pub use allocation::{
    MTLAllocation, MTLAllocationRef
};
pub use residency_set::{
    MTLResidencySetDescriptor, MTLResidencySetDescriptorRef,
    MTLResidencySet, MTLResidencySetRef
};
pub use rasterization_rate::{
    MTLRasterizationRateSampleArray, MTLRasterizationRateSampleArrayRef,
    MTLRasterizationRateLayerDescriptor, MTLRasterizationRateLayerDescriptorRef,
    MTLRasterizationRateLayerArray, MTLRasterizationRateLayerArrayRef,
    MTLRasterizationRateMapDescriptor, MTLRasterizationRateMapDescriptorRef,
    MTLRasterizationRateMap, MTLRasterizationRateMapRef
};
pub use linked_functions::{
    MTLLinkedFunctions, MTLLinkedFunctionsRef
};
pub use counters::{
    MTLCounterSamplingPoint, MTLCounterSampleBufferError,
    MTLCounterResultTimestamp, MTLCounterResultStageUtilization, MTLCounterResultStatistic,
    MTLCounter, MTLCounterRef,
    MTLCounterSet, MTLCounterSetRef,
    MTLCounterSampleBufferDescriptor, MTLCounterSampleBufferDescriptorRef,
    MTLCounterSampleBuffer, MTLCounterSampleBufferRef,
    counter_sampling
};
pub use function_stitching::{
    MTLStitchedLibraryOptions, MTLStitchedLibraryOptionFlags,
    MTLFunctionStitchingAttribute, MTLFunctionStitchingAttributeRef,
    MTLFunctionStitchingAttributeAlwaysInline, MTLFunctionStitchingAttributeAlwaysInlineRef,
    MTLFunctionStitchingNode, MTLFunctionStitchingNodeRef,
    MTLFunctionStitchingInputNode, MTLFunctionStitchingInputNodeRef,
    MTLFunctionStitchingFunctionNode, MTLFunctionStitchingFunctionNodeRef,
    MTLFunctionStitchingGraph, MTLFunctionStitchingGraphRef,
    MTLStitchedLibraryDescriptor, MTLStitchedLibraryDescriptorRef
};
pub use indirect_command_buffer::{
    MTLIndirectCommandType,
    MTLIndirectCommandBufferExecutionRange,
    MTLIndirectCommandBufferDescriptor, MTLIndirectCommandBufferDescriptorRef,
    MTLIndirectCommandBuffer, MTLIndirectCommandBufferRef
};
pub use indirect_command_encoder::{
    MTLIndirectRenderCommand, MTLIndirectRenderCommandRef,
    MTLIndirectComputeCommand, MTLIndirectComputeCommandRef
};
pub use binary_archive::{
    MTLBinaryArchiveError,
    MTLBinaryArchiveDescriptor, MTLBinaryArchiveDescriptorRef,
    MTLBinaryArchive, MTLBinaryArchiveRef
};
pub use dynamic_library::{
    MTLDynamicLibraryError,
    MTLDynamicLibrary, MTLDynamicLibraryRef
};

pub use visible_function_table::{
    MTLVisibleFunctionTable, MTLVisibleFunctionTableRef,
    MTLVisibleFunctionTableDescriptor, MTLVisibleFunctionTableDescriptorRef
};

pub use intersection_function_table::{
    MTLIntersectionFunctionSignature,
    MTLIntersectionFunctionTable, MTLIntersectionFunctionTableRef,
    MTLIntersectionFunctionTableDescriptor, MTLIntersectionFunctionTableDescriptorRef
};

pub use acceleration_structure::{
    MTLAccelerationStructureUsage,
    MTLAccelerationStructureSizes,
    MTLAccelerationStructure, MTLAccelerationStructureRef,
    MTLAccelerationStructureDescriptor, MTLAccelerationStructureDescriptorRef
};

pub use acceleration_structure_command_encoder::{
    MTLAccelerationStructureRefitOptions,
    MTLAccelerationStructureCommandEncoder, MTLAccelerationStructureCommandEncoderRef,
    MTLAccelerationStructurePassDescriptor, MTLAccelerationStructurePassDescriptorRef
};

pub use acceleration_structure_types::{
    PackedFloat3,
    PackedFloat4x3,
    PackedFloatQuaternion,
    AxisAlignedBoundingBox,
    ComponentTransform
};

pub use capture_manager::{
    MTLCaptureError,
    MTLCaptureDestination,
    MTLCaptureDescriptor, MTLCaptureDescriptorRef,
    MTLCaptureScope, MTLCaptureScopeRef,
    MTLCaptureManager, MTLCaptureManagerRef
};

pub use log_state::{
    MTLLogLevel,
    MTLLogStateError,
    MTLLogHandler,
    MTLLogStateDescriptor, MTLLogStateDescriptorRef,
    MTLLogState, MTLLogStateRef
};

pub use io_command_buffer::{
    MTLIOStatus,
    MTLIOError,
    MTLIOFileHandle, MTLIOFileHandleRef,
    MTLIOScratchBuffer, MTLIOScratchBufferRef,
    MTLIOScratchBufferAllocator, MTLIOScratchBufferAllocatorRef,
    MTLIOCommandBuffer, MTLIOCommandBufferRef
};

pub use io_command_queue::{
    MTLIOPriority,
    MTLIOCommandQueueType,
    MTLIOCommandQueueDescriptor, MTLIOCommandQueueDescriptorRef,
    MTLIOCommandQueue, MTLIOCommandQueueRef
};

pub use io_compressor::{
    MTLIOCompressionMethod,
    MTLIOCompressionStatus,
    MTLIOCompressionContext
};