# metal-rs Todo List

This document outlines the comprehensive todo list for developing a fully-featured Rust wrapper over the Metal API, comparable to metal-cpp.

## Core Architecture Improvements

- [ ] Implement proper error handling throughout the API
  - [ ] Create MTLError type that wraps NSError
  - [ ] Use Result return types for fallible operations
  - [ ] Add proper error propagation in all functions that can fail

- [ ] Improve memory management
  - [ ] Review and audit all Drop implementations
  - [ ] Ensure correct retain/release semantics for all types
  - [ ] Add runtime assertions for invalid object use

- [ ] Add strong type safety
  - [ ] Use newtypes for integer values where appropriate
  - [ ] Enforce API contracts through the type system
  - [ ] Add validation for API parameter ranges

- [ ] Enhance API ergonomics
  - [ ] Add builder patterns for complex descriptor types
  - [ ] Implement From/Into traits for compatible types
  - [ ] Add convenience methods for common operations

## Unimplemented Types

### Command Encoding

- [x] MTLComputeCommandEncoder
  - [x] Basic encoding functionality
  - [x] Thread group dispatching
  - [x] Buffer/texture binding
  - [x] State setting methods

- [x] MTLParallelRenderCommandEncoder
  - [x] Multicore rendering support
  - [x] Child encoder creation/management

- [x] MTLResourceStateCommandEncoder
  - [x] Resource state tracking
  - [x] Barrier operations
  - [x] Sparse texture mapping

- [x] MTLComputePass
  - [x] Pass descriptor handling
  - [x] Compute workload management

- [x] MTLRenderPass
  - [x] Pass descriptor handling
  - [x] Attachment configuration

- [x] MTLBlitPass
  - [x] Pass descriptor handling
  - [x] Blit operation configuration

### Pipeline States

- [x] MTLPipeline
  - [x] Base pipeline functionality
  - [x] Common properties and methods

- [x] Complete MTLRenderPipeline
  - [x] Full descriptor implementation
  - [x] Pipeline reflection
  - [x] Pipeline state creation with validation

- [x] MTLComputePipeline
  - [x] Pipeline state creation
  - [x] Pipeline reflection
  - [x] Kernel function binding
  - [x] Full descriptor implementation with buffers

### Resource Management

- [x] Complete MTLResource
  - [x] Resource common functionality
  - [x] Resource options and state management
  - [x] Proper inheritance for specific resources

- [x] MTLDrawable
  - [x] Drawable presentation
  - [x] Frame timing
  - [x] Display synchronization

- [x] MTLHeap
  - [x] Resource allocation from heaps
  - [x] Heap sizing and configuration
  - [x] Placement resource creation

- [x] MTLFence
  - [x] Cross-encoder synchronization
  - [x] Fence creation/updating/waiting

- [x] MTLEvent
  - [x] GPU timeline synchronization
  - [x] Event creation/signaling/waiting

- [x] MTLResidencySet
  - [x] Residency group management
  - [x] Explicit residency tracking

- [x] MTLAllocation
  - [x] Memory allocation management
  - [x] Manual resource allocation

### Descriptors and Data Structures

- [x] MTLVertexDescriptor
  - [x] Vertex layout configuration
  - [x] Vertex attribute formats
  - [x] Buffer bindings

- [x] MTLStageInputOutputDescriptor
  - [x] Compute stage configuration
  - [x] IO buffer bindings

- [x] MTLSampler
  - [x] Filtering options
  - [x] Address modes
  - [x] LOD settings

- [x] MTLFunctionDescriptor
  - [x] Function creation with specialized settings
  - [x] Function linkage options

### Argument Handling

- [x] MTLArgument
  - [x] Shader reflection
  - [x] Argument metadata

- [x] MTLArgumentEncoder
  - [x] Resource binding for indirect access
  - [x] Argument buffer creation

### Function Features

- [x] Complete MTLFunctionConstantValues
  - [x] More methods for setting constant values
  - [x] Type-safe constant value management

- [~] MTLFunctionHandle
  - [~] Function reference management (placeholder implementation)
  - [~] Indirect function calls (placeholder implementation)

- [x] MTLFunctionLog
  - [x] Function log handling
  - [x] Compiler feedback

- [x] MTLFunctionStitching
  - [x] Function stitching configuration
  - [x] Dynamic shader composition

### Advanced Rendering

- [x] MTLRasterizationRate
  - [x] Variable rate rasterization
  - [x] Rasterization map configuration

- [x] MTLLinkedFunctions
  - [x] Function linking
  - [x] Dynamic function selection

- [x] MTLCounters
  - [x] Performance counter access
  - [x] Statistics gathering

### Indirect Commands

- [x] MTLIndirectCommandBuffer
  - [x] Indirect command recording
  - [x] Command execution

- [x] MTLIndirectCommandEncoder
  - [x] Render command encoding
  - [x] Compute command encoding

### Ray Tracing and Acceleration Structures

- [x] MTLAccelerationStructure
  - [x] Acceleration structure creation
  - [x] Acceleration structure updates

- [x] MTLAccelerationStructureCommandEncoder
  - [x] Build commands
  - [x] Refitting commands

- [x] MTLAccelerationStructureTypes
  - [x] Data types for acceleration structure
  - [x] Geometry descriptors

- [x] MTLVisibleFunctionTable
  - [x] Function table configuration
  - [x] Shader binding

- [x] MTLIntersectionFunctionTable
  - [x] Intersection function binding
  - [x] Ray-geometry intersection definition

### Binary and Library Management

- [x] MTLBinaryArchive
  - [x] Binary storage for compiled pipelines
  - [x] Archive loading/saving

- [x] MTLDynamicLibrary
  - [x] Dynamic function loading
  - [x] Shared Metal code modules

### Debugging and Profiling

- [x] MTLCaptureManager
  - [x] GPU workload capture
  - [x] Debugging session management

- [x] MTLCaptureScope
  - [x] Capture scope definition
  - [x] Capture boundaries

- [x] MTLLogState
  - [x] GPU logging configuration
  - [x] Debugging feedback

### I/O Operations

- [x] MTLIOCommandBuffer
  - [x] I/O operations command buffer
  - [x] Asynchronous data transfer

- [x] MTLIOCommandQueue
  - [x] I/O command queue management
  - [x] Command scheduling

- [x] MTLIOCompressor
  - [x] Hardware-accelerated compression
  - [x] Efficient data transfer

## Testing and Examples

- [ ] Unit test suite
  - [ ] Test for each major type
  - [ ] Test for each major function
  - [ ] Edge case testing
  - [ ] Error case testing

- [ ] Integration tests
  - [ ] End-to-end rendering pipeline test
  - [ ] End-to-end compute pipeline test
  - [ ] Performance benchmarks

- [ ] Example applications
  - [ ] Basic triangle rendering
  - [ ] Texture mapping and sampling
  - [ ] Compute shader usage
  - [ ] Advanced rendering techniques
  - [ ] Interop with other native libraries

## Documentation

- [ ] API documentation
  - [ ] All public types fully documented
  - [ ] Example code for each major feature
  - [ ] Migration guide from metal-cpp/Objective-C
  - [ ] Best practices section

- [ ] Guides and tutorials
  - [ ] Getting started guide
  - [ ] Basic rendering tutorial
  - [ ] Basic compute tutorial
  - [ ] Memory management guide
  - [ ] Performance optimization guide

## Infrastructure and Tooling

- [ ] CI/CD pipeline
  - [ ] Automated tests on macOS
  - [ ] Documentation generation
  - [ ] Code coverage reporting

- [ ] Build system improvements
  - [ ] Feature flags for optional components
  - [ ] Conditional compilation for platform differences
  - [ ] Cargo features for version compatibility

## Implementation Approach

For each component, follow this general approach:

1. Study [metal-cpp](metal-cpp) implementation and Apple's Metal API documentation
2. Design idiomatic Rust wrappers that follow established patterns
3. Implement dual-type pattern (owned type + ref type)
4. Add proper memory management with Drop/Clone
5. Implement core functionality
6. Add safety wrappers and ergonomic APIs
7. Write unit tests and documentation
8. Create examples demonstrating the feature

## Priorities

1. Complete core types needed for basic rendering
2. Add compute pipeline support
3. Implement memory optimization features (heaps, fences)
4. Add advanced rendering features
5. Implement specialized features (ray tracing, etc.)

Remember that Metal API evolves with each macOS/iOS release, so this todo list will need regular updates to stay current with Apple's latest additions.
