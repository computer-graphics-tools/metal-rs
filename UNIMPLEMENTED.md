# Unimplemented Metal Types

This document tracks Metal types that are available in metal-cpp but not yet implemented in metal-rs.

## Core Metal Types

| Category | Unimplemented Types |
|----------|---------------------|
| **Command Encoding** | |
| **Pipeline States** | |
| **Pipeline Descriptors** | |
| **Resource Management** | |
| **Descriptors** | |
| **Advanced Rendering** |  |
| **Function Features** | MTLFunctionHandle (partially implemented) |
| **Indirect Commands** | *Implemented* |
| **Binary Archives** | *Implemented* |
| **Ray Tracing** | *Implemented* |
| **Function Tables** | *Implemented* |
| **Arguments** | *Implemented* |
| **Debugging & Profiling** | *Implemented* |
| **I/O Operations** | *Implemented* |

## Completed Types

| Category | Implemented Types |
|----------|-------------------|
| **Command Management** | MTLDevice, MTLCommandQueue, MTLCommandBuffer, MTLCommandEncoder |
| **Resources** | MTLResource, MTLBuffer, MTLTexture, MTLTextureDescriptor |
| **Rendering** | MTLRenderCommandEncoder (partially), MTLDepthStencilState, MTLRenderPass, MTLParallelRenderCommandEncoder |
| **Synchronization** | MTLFence, MTLEvent |
| **Pass Descriptors** | MTLRenderPass, MTLComputePass, MTLBlitPass |
| **Memory Operations** | MTLBlitCommandEncoder, MTLResourceStateCommandEncoder |
| **Shader Code** | MTLLibrary, MTLFunction, MTLFunctionConstantValues |
| **Compute** | MTLComputeCommandEncoder, MTLComputePipelineState, MTLComputePipelineDescriptor |
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
   - ✅ MTLEvent
   - ✅ MTLComputePass
   - ✅ MTLBlitPass

5. **Specialized Features**
   - Ray tracing types (MTLAccelerationStructure, etc.)
   - ✅ Indirect command buffers
   - ✅ Binary archives
   - I/O operations

## Notes

Metal types should be implemented following the existing dual-type pattern:

- An owned type (e.g., `MTLSampler`) with ownership semantics
- A borrowed reference type (e.g., `MTLSamplerRef`) for borrowed references

Each implementation should include proper memory management with `Drop`/`Clone` implementations and follow Rust naming conventions while maintaining compatibility with the Objective-C Metal API.
