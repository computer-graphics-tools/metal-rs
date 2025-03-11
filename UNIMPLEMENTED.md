# Unimplemented Metal Types

This document tracks Metal types that are available in metal-cpp but not yet implemented in metal-rs.

## Core Metal Types

| Category | Unimplemented Types |
|----------|---------------------|
| **Command Encoding** | MTLResourceStateCommandEncoder, MTLComputePass, MTLBlitPass |
| **Pipeline States** | MTLPipeline |
| **Resource Management** | MTLEvent, MTLAllocation, MTLResidencySet |
| **Descriptors** | MTLStageInputOutputDescriptor, MTLFunctionDescriptor |
| **Advanced Rendering** | MTLRasterizationRate, MTLLinkedFunctions, MTLCounters |
| **Function Features** | MTLFunctionHandle, MTLFunctionLog, MTLFunctionStitching |
| **Indirect Commands** | MTLIndirectCommandBuffer, MTLIndirectCommandEncoder |
| **Binary Archives** | MTLBinaryArchive, MTLDynamicLibrary |
| **Ray Tracing** | MTLAccelerationStructure, MTLAccelerationStructureCommandEncoder, MTLAccelerationStructureTypes |
| **Function Tables** | MTLVisibleFunctionTable, MTLIntersectionFunctionTable |
| **Arguments** | MTLArgument |
| **Debugging & Profiling** | MTLCaptureManager, MTLCaptureScope, MTLLogState |
| **I/O Operations** | MTLIOCommandBuffer, MTLIOCommandQueue, MTLIOCompressor |

## Completed Types

| Category | Implemented Types |
|----------|-------------------|
| **Command Management** | MTLDevice, MTLCommandQueue, MTLCommandBuffer, MTLCommandEncoder |
| **Resources** | MTLResource, MTLBuffer, MTLTexture, MTLTextureDescriptor |
| **Rendering** | MTLRenderCommandEncoder (partially), MTLDepthStencilState, MTLRenderPass, MTLParallelRenderCommandEncoder |
| **Memory Operations** | MTLBlitCommandEncoder |
| **Shader Code** | MTLLibrary, MTLFunction, MTLFunctionConstantValues |
| **Compute** | MTLComputeCommandEncoder, MTLComputePipelineState |
| **Types and Constants** | MTLPixelFormat, MTLTypes |
| **Sampling** | MTLSampler |
| **Vertex Data** | MTLVertexDescriptor |
| **Synchronization** | MTLFence |
| **Presentation** | MTLDrawable |
| **Memory Management** | MTLHeap |
| **Argument Buffers** | MTLArgumentEncoder |

## Priority Implementation Order

Based on the typical Metal application requirements and existing implementation, here's a suggested implementation order:

1. **Essential for Rendering**
   - ✅ MTLSampler
   - ✅ MTLVertexDescriptor
   - ✅ Complete MTLRenderPipeline
   - ✅ Complete MTLResource

2. **Essential for Resource Management**
   - ✅ MTLFence (synchronization)
   - ✅ MTLDrawable
   - ✅ MTLHeap

3. **Essential for Advanced Rendering**
   - ✅ MTLArgumentEncoder
   - ✅ Complete MTLFunctionConstantValues
   - ✅ MTLRenderPass

4. **Advanced Features**
   - ✅ MTLParallelRenderCommandEncoder
   - MTLEvent
   - MTLComputePass
   - MTLBlitPass

5. **Specialized Features**
   - Ray tracing types (MTLAccelerationStructure, etc.)
   - Indirect command buffers
   - Binary archives
   - I/O operations

## Notes

Metal types should be implemented following the existing dual-type pattern:

- An owned type (e.g., `MTLSampler`) with ownership semantics
- A borrowed reference type (e.g., `MTLSamplerRef`) for borrowed references

Each implementation should include proper memory management with `Drop`/`Clone` implementations and follow Rust naming conventions while maintaining compatibility with the Objective-C Metal API.
