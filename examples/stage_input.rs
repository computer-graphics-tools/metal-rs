//! Example demonstrating the use of MTLStageInputOutputDescriptor for
//! configuring the stage inputs and outputs for compute pipelines.

use metal_rs::metal::{
    MTLAttributeFormat, MTLStepFunction,
    MTLStageInputOutputDescriptor,
    MTLCreateSystemDefaultDevice,
    MTLPipelineOption,
    MTLIndexType
};

fn main() {
    // Get the default Metal device
    let device = MTLCreateSystemDefaultDevice();
    println!("Using device: {}", device.name());

    // Create a Metal library from source code
    let shader_src = r#"
    #include <metal_stdlib>
    using namespace metal;

    // Structure to pass per-vertex data to the compute kernel
    struct Vertex {
        float3 position [[attribute(0)]];
        float2 texcoord [[attribute(1)]];
        float3 normal [[attribute(2)]];
    };

    kernel void processVertices(
        device Vertex *input [[buffer(0)]],
        device Vertex *output [[buffer(1)]],
        uint id [[thread_position_in_grid]]
    ) {
        // Just copy the vertex data from input to output
        output[id] = input[id];
        
        // You could do more processing here, like:
        // - Transform vertices
        // - Calculate lighting
        // - Apply effects
    }
    "#;

    let library = device.new_library_with_source(shader_src, &Default::default())
        .expect("Failed to create Metal library");

    // Get the compute kernel function
    let kernel_function = library.get_function("processVertices")
        .expect("Failed to find kernel function");

    // Create a compute pipeline descriptor
    let compute_pipeline_descriptor = metal_rs::metal::MTLComputePipelineDescriptor::new();
    compute_pipeline_descriptor.set_compute_function(&kernel_function);
    
    // Create a stage input output descriptor to define the vertex layout
    let stage_input = MTLStageInputOutputDescriptor::stage_input_output_descriptor();
    
    // Configure buffer layouts
    let layouts = stage_input.as_ref().layouts();
    
    // Configure vertex buffer layout
    let vertex_buffer_layout = layouts.as_ref().object(0).unwrap(); // Buffer at index 0
    vertex_buffer_layout.as_ref().set_stride(32); // 3 floats + 2 floats + 3 floats = 8 floats * 4 bytes = 32 bytes per vertex
    vertex_buffer_layout.as_ref().set_step_function(MTLStepFunction::PerVertex);
    vertex_buffer_layout.as_ref().set_step_rate(1);
    
    // Configure attributes
    let attributes = stage_input.as_ref().attributes();
    
    // Position attribute
    let position_attr = attributes.as_ref().object(0).unwrap();
    position_attr.as_ref().set_format(MTLAttributeFormat::Float3);
    position_attr.as_ref().set_offset(0);
    position_attr.as_ref().set_buffer_index(0);
    
    // Texcoord attribute
    let texcoord_attr = attributes.as_ref().object(1).unwrap();
    texcoord_attr.as_ref().set_format(MTLAttributeFormat::Float2);
    texcoord_attr.as_ref().set_offset(12); // After 3 floats (12 bytes)
    texcoord_attr.as_ref().set_buffer_index(0);
    
    // Normal attribute
    let normal_attr = attributes.as_ref().object(2).unwrap();
    normal_attr.as_ref().set_format(MTLAttributeFormat::Float3);
    normal_attr.as_ref().set_offset(20); // After 3 floats + 2 floats (20 bytes)
    normal_attr.as_ref().set_buffer_index(0);
    
    // Configure indexing (if needed)
    stage_input.as_ref().set_index_type(MTLIndexType::UInt16);
    stage_input.as_ref().set_index_buffer_index(1);

    // Create the compute pipeline state with the stage input descriptor
    let pipeline_state = device.new_compute_pipeline_state_with_stage_input(
        &compute_pipeline_descriptor,
        &stage_input,
        MTLPipelineOption::None
    ).expect("Failed to create compute pipeline state");

    println!("Successfully created compute pipeline state with stage input");
    println!("Max total threads per threadgroup: {}", pipeline_state.max_total_threads_per_threadgroup());
    println!("Threadgroup size: {}", pipeline_state.thread_execution_width());
}