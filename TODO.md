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

- [ ] MTLResourceStateCommandEncoder
  - [ ] Resource state tracking
  - [ ] Barrier operations

- [ ] MTLComputePass
  - [ ] Pass descriptor handling
  - [ ] Compute workload management

- [x] MTLRenderPass
  - [x] Pass descriptor handling
  - [x] Attachment configuration

- [ ] MTLBlitPass
  - [ ] Pass descriptor handling
  - [ ] Blit operation configuration

### Pipeline States

- [ ] MTLPipeline
  - [ ] Base pipeline functionality
  - [ ] Common properties and methods

- [x] Complete MTLRenderPipeline
  - [x] Full descriptor implementation
  - [x] Pipeline reflection
  - [x] Pipeline state creation with validation

- [x] MTLComputePipeline
  - [x] Pipeline state creation
  - [x] Pipeline reflection
  - [x] Kernel function binding

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

- [ ] MTLResidencySet
  - [ ] Residency group management
  - [ ] Explicit residency tracking

- [ ] MTLAllocation
  - [ ] Memory allocation management
  - [ ] Manual resource allocation

### Descriptors and Data Structures

- [x] MTLVertexDescriptor
  - [x] Vertex layout configuration
  - [x] Vertex attribute formats
  - [x] Buffer bindings

- [ ] MTLStageInputOutputDescriptor
  - [ ] Compute stage configuration
  - [ ] IO buffer bindings

- [x] MTLSampler
  - [x] Filtering options
  - [x] Address modes
  - [x] LOD settings

- [ ] MTLFunctionDescriptor
  - [ ] Function creation with specialized settings
  - [ ] Function linkage options

### Argument Handling

- [ ] MTLArgument
  - [ ] Shader reflection
  - [ ] Argument metadata

- [x] MTLArgumentEncoder
  - [x] Resource binding for indirect access
  - [x] Argument buffer creation

### Function Features

- [x] Complete MTLFunctionConstantValues
  - [x] More methods for setting constant values
  - [x] Type-safe constant value management

- [ ] MTLFunctionHandle
  - [ ] Function reference management
  - [ ] Indirect function calls

- [ ] MTLFunctionLog
  - [ ] Function log handling
  - [ ] Compiler feedback

- [ ] MTLFunctionStitching
  - [ ] Function stitching configuration
  - [ ] Dynamic shader composition

### Advanced Rendering

- [ ] MTLRasterizationRate
  - [ ] Variable rate rasterization
  - [ ] Rasterization map configuration

- [ ] MTLLinkedFunctions
  - [ ] Function linking
  - [ ] Dynamic function selection

- [ ] MTLCounters
  - [ ] Performance counter access
  - [ ] Statistics gathering

### Indirect Commands

- [ ] MTLIndirectCommandBuffer
  - [ ] Indirect command recording
  - [ ] Command execution

- [ ] MTLIndirectCommandEncoder
  - [ ] Render command encoding
  - [ ] Compute command encoding

### Ray Tracing and Acceleration Structures

- [ ] MTLAccelerationStructure
  - [ ] Acceleration structure creation
  - [ ] Acceleration structure updates

- [ ] MTLAccelerationStructureCommandEncoder
  - [ ] Build commands
  - [ ] Refitting commands

- [ ] MTLAccelerationStructureTypes
  - [ ] Data types for acceleration structure
  - [ ] Geometry descriptors

- [ ] MTLVisibleFunctionTable
  - [ ] Function table configuration
  - [ ] Shader binding

- [ ] MTLIntersectionFunctionTable
  - [ ] Intersection function binding
  - [ ] Ray-geometry intersection definition

### Binary and Library Management

- [ ] MTLBinaryArchive
  - [ ] Binary storage for compiled pipelines
  - [ ] Archive loading/saving

- [ ] MTLDynamicLibrary
  - [ ] Dynamic function loading
  - [ ] Shared Metal code modules

### Debugging and Profiling

- [ ] MTLCaptureManager
  - [ ] GPU workload capture
  - [ ] Debugging session management

- [ ] MTLCaptureScope
  - [ ] Capture scope definition
  - [ ] Capture boundaries

- [ ] MTLLogState
  - [ ] GPU logging configuration
  - [ ] Debugging feedback

### I/O Operations

- [ ] MTLIOCommandBuffer
  - [ ] I/O operations command buffer
  - [ ] Asynchronous data transfer

- [ ] MTLIOCommandQueue
  - [ ] I/O command queue management
  - [ ] Command scheduling

- [ ] MTLIOCompressor
  - [ ] Hardware-accelerated compression
  - [ ] Efficient data transfer

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
