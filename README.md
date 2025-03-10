# metal-rs

Rust bindings for Apple's Metal API.

## Overview

`metal-rs` is a safe and idiomatic Rust wrapper around Apple's Metal API, allowing Rust developers to create high-performance graphics and compute applications for Apple platforms. This library provides a comprehensive interface to Metal's functionality while leveraging Rust's memory safety guarantees.

## Features

- Safe Rust wrappers for Metal API objects
- Type-safe enumerations and bitflags for Metal constants
- Seamless integration with other Rust libraries
- Support for both rendering and compute workflows
- Resource management with proper ownership semantics
- Comprehensive error handling

## Supported Metal Features

- Device discovery and information
- Command queues and buffers
- Buffer creation and management
- Texture creation and manipulation
- Shader compilation through Metal libraries
- Render pipelines and render passes
- Compute pipelines and compute dispatching
- Samplers and texture filtering
- Blitting operations
- Memory barriers and synchronization
- Debugging and profiling utilities
- Ray tracing and acceleration structures
- GPU-driven rendering with indirect command buffers
- Binary archives for pipeline caching
- Parallel command encoders for multi-threading
- Variable rate shading with rasterization rate maps
- Shader argument reflection

## Requirements

- macOS, iOS, or other Apple platform with Metal support
- Rust 1.56.0 or later

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
metal-rs = "0.1.0"
```

For most applications, you'll also want to include `bytemuck` for safe type casting:

```toml
[dependencies]
metal-rs = "0.1.0"
bytemuck = { version = "1.14.0", features = ["derive"] }
```

## Basic Usage

### Getting a Metal Device

```rust
use metal_rs::device::Device;

fn main() {
    // Get the default system device
    let device = Device::system_default().expect("No Metal device found");
    
    println!("Using device: {}", device.name());
    println!("Low power: {}", device.is_low_power());
    println!("Unified memory: {}", device.has_unified_memory());
}
```

### Creating and Using Buffers

```rust
use metal_rs::{
    constants::MTLResourceOptions,
    device::Device,
};

fn main() {
    let device = Device::system_default().expect("No Metal device found");
    
    // Create a buffer with some data
    let data = [1u32, 2, 3, 4, 5];
    let buffer = device.new_buffer_with_data(
        bytemuck::cast_slice(&data),
        MTLResourceOptions::CPUCacheModeDefaultCache | MTLResourceOptions::StorageModeShared,
    );
    
    println!("Buffer length: {} bytes", buffer.length());
    
    // Read back from the buffer
    let contents = buffer.contents_as_slice::<u32>(data.len());
    println!("Buffer contents: {:?}", contents);
}
```

### Basic Triangle Rendering

```rust
use metal_rs::{
    constants::MTLResourceOptions,
    device::Device,
    enums::{MTLPixelFormat, MTLLoadAction, MTLStoreAction, MTLCullMode, MTLPrimitiveType},
    texture::TextureDescriptor,
    render::{RenderPipelineDescriptor, RenderPassDescriptor},
    types::{MTLClearColor, MTLViewport, MTLScissorRect},
};

// Metal shader source code for a simple triangle
const SHADER_SOURCE: &str = r#"
#include <metal_stdlib>
using namespace metal;

struct VertexOut {
    float4 position [[position]];
    float4 color;
};

vertex VertexOut vertex_main(uint vertex_id [[vertex_id]]) {
    const float3 positions[3] = {
        float3(-0.5, -0.5, 0.0),
        float3( 0.0,  0.5, 0.0),
        float3( 0.5, -0.5, 0.0)
    };
    
    const float4 colors[3] = {
        float4(1.0, 0.0, 0.0, 1.0),
        float4(0.0, 1.0, 0.0, 1.0),
        float4(0.0, 0.0, 1.0, 1.0)
    };
    
    VertexOut out;
    out.position = float4(positions[vertex_id], 1.0);
    out.color = colors[vertex_id];
    return out;
}

fragment float4 fragment_main(VertexOut in [[stage_in]]) {
    return in.color;
}
"#;

fn main() {
    let device = Device::system_default().expect("No Metal device found");
    let command_queue = device.new_command_queue();
    
    // Create a library from the shader source
    let library = device.new_library_with_source(SHADER_SOURCE, None)
        .expect("Failed to create library from source");
    
    // Get the vertex and fragment functions
    let vertex_function = library.get_function("vertex_main", None)
        .expect("Failed to get vertex function");
    let fragment_function = library.get_function("fragment_main", None)
        .expect("Failed to get fragment function");
    
    // Create a render pipeline
    let pipeline_descriptor = RenderPipelineDescriptor::new();
    pipeline_descriptor.set_vertex_function(Some(&vertex_function));
    pipeline_descriptor.set_fragment_function(Some(&fragment_function));
    
    // Configure the color attachment format
    let color_attachment = pipeline_descriptor.color_attachment(0);
    color_attachment.set_pixel_format(MTLPixelFormat::BGRA8Unorm);
    
    // Create the pipeline state
    let pipeline_state = device.new_render_pipeline_state_with_descriptor(&pipeline_descriptor)
        .expect("Failed to create render pipeline state");
    
    // Create a texture to render to
    let texture_descriptor = TextureDescriptor::texture_2d_descriptor(
        MTLPixelFormat::BGRA8Unorm, 800, 600, false,
    );
    texture_descriptor.set_resource_options(
        MTLResourceOptions::CPUCacheModeDefaultCache | MTLResourceOptions::StorageModeShared
    );
    let texture = device.new_texture(&texture_descriptor);
    
    // Create a render pass descriptor
    let render_pass_descriptor = RenderPassDescriptor::new();
    let color_attachment = render_pass_descriptor.color_attachment(0);
    color_attachment.set_texture(Some(&texture));
    color_attachment.set_load_action(MTLLoadAction::Clear);
    color_attachment.set_store_action(MTLStoreAction::Store);
    color_attachment.set_clear_color(MTLClearColor::new(0.2, 0.2, 0.2, 1.0));
    
    // Create a command buffer and render command encoder
    let command_buffer = command_queue.new_command_buffer();
    let encoder = command_buffer.new_render_command_encoder(&render_pass_descriptor);
    
    // Set the render state and viewport
    encoder.set_render_pipeline_state(&pipeline_state);
    encoder.set_viewport(MTLViewport::new(0.0, 0.0, 800.0, 600.0, 0.0, 1.0));
    
    // Draw the triangle
    encoder.draw_primitives(MTLPrimitiveType::Triangle, 0, 3);
    encoder.end_encoding();
    
    // Commit the command buffer and wait for completion
    command_buffer.commit();
    command_buffer.wait_until_completed();
}
```

### Basic Compute

```rust
use metal_rs::{
    constants::MTLResourceOptions,
    device::Device,
    enums::MTLDataType,
    types::MTLSize,
};

// Metal compute shader source
const COMPUTE_SHADER_SOURCE: &str = r#"
#include <metal_stdlib>
using namespace metal;

kernel void add_arrays(device const float* inA,
                        device const float* inB,
                        device float* result,
                        uint index [[thread_position_in_grid]]) {
    result[index] = inA[index] + inB[index];
}
"#;

fn main() {
    let device = Device::system_default().expect("No Metal device found");
    let command_queue = device.new_command_queue();
    
    // Create input data
    let data_size = 1024;
    let a_data: Vec<f32> = (0..data_size).map(|i| i as f32).collect();
    let b_data: Vec<f32> = (0..data_size).map(|i| (i * 2) as f32).collect();
    
    // Create input and output buffers
    let buffer_a = device.new_buffer_with_data(
        bytemuck::cast_slice(&a_data),
        MTLResourceOptions::StorageModeShared,
    );
    let buffer_b = device.new_buffer_with_data(
        bytemuck::cast_slice(&b_data),
        MTLResourceOptions::StorageModeShared,
    );
    let buffer_result = device.new_buffer(
        data_size * std::mem::size_of::<f32>(),
        MTLResourceOptions::StorageModeShared,
    );
    
    // Create a compute pipeline
    let library = device.new_library_with_source(COMPUTE_SHADER_SOURCE, None)
        .expect("Failed to create library");
    let kernel_function = library.get_function("add_arrays", None)
        .expect("Failed to get kernel function");
    let compute_pipeline_state = device.new_compute_pipeline_state_with_function(&kernel_function)
        .expect("Failed to create compute pipeline state");
    
    // Create a command buffer and compute encoder
    let command_buffer = command_queue.new_command_buffer();
    let compute_encoder = command_buffer.new_compute_command_encoder();
    
    // Configure the compute encoder
    compute_encoder.set_compute_pipeline_state(&compute_pipeline_state);
    compute_encoder.set_buffer(Some(&buffer_a), 0, 0);
    compute_encoder.set_buffer(Some(&buffer_b), 0, 1);
    compute_encoder.set_buffer(Some(&buffer_result), 0, 2);
    
    // Dispatch the compute grid
    let grid_size = MTLSize::new(data_size, 1, 1);
    let thread_group_size = MTLSize::new(
        compute_pipeline_state.max_total_threads_per_threadgroup(),
        1, 
        1
    );
    compute_encoder.dispatch_threads(grid_size, thread_group_size);
    compute_encoder.end_encoding();
    
    // Execute and wait for completion
    command_buffer.commit();
    command_buffer.wait_until_completed();
    
    // Read back the results
    let result_slice = buffer_result.contents_as_slice::<f32>(data_size);
    println!("First few results: {:?}", &result_slice[0..10]);
}
```

### Ray Tracing

```rust
use metal_rs::*;
use std::mem::size_of;

// Vertex struct with position
#[repr(C)]
struct Vertex {
    position: [f32; 3],
}

// Ray struct matching Metal's ray data layout
#[repr(C)]
struct Ray {
    origin: [f32; 3],
    min_distance: f32,
    direction: [f32; 3],
    max_distance: f32,
}

// Intersection result struct
#[repr(C)]
struct Intersection {
    distance: f32,
    primitive_id: u32,
    geometry_id: u32,
    instance_id: u32,
}

fn main() {
    // Get a Metal device that supports ray tracing
    let device = MTLCreateSystemDefaultDevice();
    if !device.supportsRaytracing() {
        println!("This device does not support ray tracing. Exiting.");
        return;
    }
    
    // Create a simple triangle geometry
    let vertices = [
        Vertex { position: [-1.0, -1.0, 0.0] },
        Vertex { position: [ 1.0, -1.0, 0.0] },
        Vertex { position: [ 0.0,  1.0, 0.0] },
    ];
    
    // Create a vertex buffer
    let vertex_buffer = device.newBuffer(
        vertices.as_ptr() as *const _,
        (vertices.len() * size_of::<Vertex>()) as u64,
        MTLResourceOptions::StorageModeShared,
    );
    
    // Create a triangle geometry descriptor
    let geometry_descriptor = MTLAccelerationStructureTriangleGeometryDescriptor::new();
    geometry_descriptor.setVertexBuffer(vertex_buffer);
    geometry_descriptor.setVertexFormat(MTLAttributeFormat::Float3);
    geometry_descriptor.setVertexStride(size_of::<Vertex>() as NSUInteger);
    geometry_descriptor.setTriangleCount(1);
    
    // Create a primitive acceleration structure descriptor
    let primitive_descriptor = MTLPrimitiveAccelerationStructureDescriptor::new();
    let geometries = unsafe {
        // Create an array to hold the geometry descriptors
        let cls = class!(NSMutableArray);
        let array: *mut Object = msg_send![cls, array];
        let _: () = msg_send![array, addObject: geometry_descriptor.as_ptr()];
        Id::from_ptr(array as *mut NSArray<MTLAccelerationStructureGeometryDescriptor>)
    };
    primitive_descriptor.setGeometryDescriptors(geometries);
    
    // Get memory requirements for the acceleration structure
    let sizes = device.accelerationStructureSizes(
        primitive_descriptor as Id<MTLAccelerationStructureDescriptor>
    );
    
    // Create the acceleration structure and scratch buffer
    let acceleration_structure = device.makeAccelerationStructure(
        primitive_descriptor as Id<MTLAccelerationStructureDescriptor>
    );
    let scratch_buffer = device.newBuffer(
        sizes.buildScratchBufferSize as u64,
        MTLResourceOptions::StorageModePrivate,
    );
    
    // Build the acceleration structure
    let command_queue = device.newCommandQueue();
    let command_buffer = command_queue.commandBuffer();
    let compute_encoder = command_buffer.computeCommandEncoder();
    compute_encoder.buildAccelerationStructure(
        acceleration_structure,
        primitive_descriptor as Id<MTLAccelerationStructureDescriptor>,
        scratch_buffer,
        0,
    );
    compute_encoder.endEncoding();
    
    // Create a ray for intersection testing
    let rays = [
        Ray {
            origin: [0.0, 0.0, -1.0],
            min_distance: 0.0,
            direction: [0.0, 0.0, 1.0],
            max_distance: 10000.0,
        }
    ];
    
    // Create buffers for ray data and intersection results
    let ray_buffer = device.newBuffer(
        rays.as_ptr() as *const _,
        (rays.len() * size_of::<Ray>()) as u64,
        MTLResourceOptions::StorageModeShared,
    );
    let intersection_buffer = device.newBuffer(
        size_of::<Intersection>() as u64,
        MTLResourceOptions::StorageModeShared,
    );
    
    // Create the intersection function table
    let function_table_descriptor = MTLIntersectionFunctionTableDescriptor::new();
    function_table_descriptor.setFunctionCount(1);
    let intersection_table = device.makeIntersectionFunctionTable(function_table_descriptor);
    intersection_table.setOpaqueTriangleIntersectionFunction(
        MTLIntersectionFunctionSignature::None,
        0,
    );
    
    // Create ray intersector and perform the intersection test
    let ray_intersector = MTLRayIntersector::new(device);
    let compute_encoder = command_buffer.computeCommandEncoder();
    compute_encoder.encodeIntersection(
        ray_intersector,
        ray_buffer,
        0,
        intersection_buffer,
        0,
        1, // 1 ray
        acceleration_structure,
        intersection_table,
    );
    compute_encoder.endEncoding();
    
    // Execute and wait for completion
    command_buffer.commit();
    command_buffer.waitUntilCompleted();
    
    // Check the intersection result
    let intersection_ptr = unsafe {
        intersection_buffer.contents() as *const Intersection
    };
    let intersection = unsafe { &*intersection_ptr };
    
    if intersection.distance < f32::INFINITY {
        println!("Ray hit the triangle at distance: {}", intersection.distance);
    } else {
        println!("Ray missed the triangle.");
    }
}
```

### GPU-Driven Rendering with Indirect Command Buffers

```rust
use metal_rs::*;

fn main() {
    // Get a Metal device that supports indirect command buffers
    let device = MTLCreateSystemDefaultDevice();
    if !device.supportsIndirectCommandBuffers() {
        println!("This device does not support indirect command buffers. Exiting.");
        return;
    }
    
    // Create an indirect command buffer descriptor
    let icb_descriptor = MTLIndirectCommandBufferDescriptor::new();
    icb_descriptor.setCommandTypes(MTLIndirectCommandType::Draw | MTLIndirectCommandType::DrawIndexed);
    icb_descriptor.setInheritPipelineState(false);
    
    // Create the indirect command buffer
    let indirect_command_buffer = device.makeIndirectCommandBuffer(
        icb_descriptor,
        10, // Number of commands to support
        MTLResourceOptions::StorageModeShared
    );
    
    // Get a render command
    let render_command = indirect_command_buffer.indirectRenderCommandAtIndex(0);
    
    // Configure the render command
    // - Set pipeline state
    // - Set vertex/fragment buffers
    // - Configure draw parameters
    // - etc.
    
    // Optimize the indirect command buffer
    let command_queue = device.newCommandQueue();
    let command_buffer = command_queue.commandBuffer();
    let blit_encoder = command_buffer.blitCommandEncoder();
    blit_encoder.optimizeIndirectCommandBuffer(
        indirect_command_buffer,
        0..1 // Range of commands to optimize
    );
    blit_encoder.endEncoding();
    
    // Execute the indirect command buffer
    let render_encoder = command_buffer.renderCommandEncoder(/* render pass descriptor */);
    render_encoder.executeIndirectCommandBuffer(
        indirect_command_buffer,
        0..1 // Range of commands to execute
    );
    render_encoder.endEncoding();
    
    // Execute and wait for completion
    command_buffer.commit();
    command_buffer.waitUntilCompleted();
}
```

## Documentation

For detailed API documentation, run:

```
cargo doc --open
```

## Project Structure

### Core Modules
- `src/foundation/` - Rust bindings for Apple's Foundation framework
  - `src/foundation/string.rs` - NSString implementation with safe wrappers
  - For more details, see [FOUNDATION.md](FOUNDATION.md)
- `src/buffer.rs` - Buffer creation and manipulation
- `src/command_buffer.rs` - Command buffer management
- `src/command_encoder.rs` - Base command encoder functionality
- `src/command_queue.rs` - Command queue operations
- `src/compute.rs` - Compute command encoders and pipelines
- `src/constants.rs` - Common constants and bit flags
- `src/device.rs` - Metal device discovery and properties
- `src/enums.rs` - Enumerations for Metal types
- `src/function.rs` - Shader function handling
- `src/library.rs` - Library loading and compilation
- `src/pipeline.rs` - Pipeline state management
- `src/render.rs` - Render command encoders and pipelines
- `src/sampler.rs` - Texture samplers
- `src/texture.rs` - Texture creation and manipulation
- `src/types.rs` - Common Metal types

### Advanced Features
- `src/acceleration_structure.rs` - Ray tracing acceleration structures
- `src/ray_tracing.rs` - Ray tracing intersection pipelines
- `src/indirect_command_buffer.rs` - GPU-driven rendering commands
- `src/parallel_render_command_encoder.rs` - Multi-threaded rendering
- `src/binary_archive.rs` - Pipeline state caching
- `src/rasterization_rate.rs` - Variable rate shading
- `src/argument.rs` - Shader reflection and argument inspection
- `src/capture_manager.rs` - Debugging and capture utilities

### Example Applications
- `examples/basic.rs` - Basic rendering example
- `examples/ray_tracing.rs` - Ray tracing with acceleration structures
- `examples/indirect_rendering.rs` - GPU-driven rendering with indirect commands

## Contributing

Contributions to metal-rs are welcome! Please feel free to submit pull requests, report issues, or suggest improvements.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

This project takes inspiration from the following Rust projects:
- [metal-rs](https://github.com/gfx-rs/metal-rs) by gfx-rs
- [core-graphics-types](https://github.com/servo/core-foundation-rs)
- [wgpu](https://github.com/gfx-rs/wgpu-rs)
- [objc-rs](https://github.com/SSheldon/rust-objc)