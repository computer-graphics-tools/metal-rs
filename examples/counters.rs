// Example demonstrating the use of Metal performance counters

use metal_rs::metal::{
    MTLCreateSystemDefaultDevice, MTLCounterSamplingPoint, MTLCounterSampleBufferDescriptor,
    MTLStorageMode, MTLSize, MTLCounterResultStatistic, MTLResourceOptions,
    counters::common_counter_set
};
use metal_rs::foundation::NSRange;
use foreign_types::ForeignType;
use objc::{msg_send, sel, sel_impl};
use std::mem::size_of;

// A simple compute kernel that does some work
const COMPUTE_SHADER: &str = r#"
#include <metal_stdlib>
using namespace metal;

kernel void count_kernel(
    device float4 *input [[ buffer(0) ]],
    device float4 *output [[ buffer(1) ]],
    uint id [[ thread_position_in_grid ]]
) {
    // Do some simple computations to generate counters
    float4 value = input[id];
    
    // Some math operations to create work for the GPU
    for (int i = 0; i < 100; i++) {
        value = sin(value) + cos(value * 0.5);
    }
    
    output[id] = value;
}
"#;

fn main() {
    // Get the default system device
    let device = MTLCreateSystemDefaultDevice();
    println!("Using device: {}", device.name());
    
    // Try different sampling points
    println!("Checking support for different sampling points:");
    println!("  Stage boundary: {}", device.supports_counter_sampling(MTLCounterSamplingPoint::AtStageBoundary));
    println!("  Draw boundary: {}", device.supports_counter_sampling(MTLCounterSamplingPoint::AtDrawBoundary));
    println!("  Dispatch boundary: {}", device.supports_counter_sampling(MTLCounterSamplingPoint::AtDispatchBoundary));
    println!("  Tile dispatch boundary: {}", device.supports_counter_sampling(MTLCounterSamplingPoint::AtTileDispatchBoundary));
    println!("  Blit boundary: {}", device.supports_counter_sampling(MTLCounterSamplingPoint::AtBlitBoundary));
    
    // Use stage boundary as fallback if dispatch boundary is not supported
    let sampling_point = if device.supports_counter_sampling(MTLCounterSamplingPoint::AtDispatchBoundary) {
        MTLCounterSamplingPoint::AtDispatchBoundary
    } else if device.supports_counter_sampling(MTLCounterSamplingPoint::AtStageBoundary) {
        MTLCounterSamplingPoint::AtStageBoundary
    } else {
        println!("Device does not support any useful counter sampling points. Exiting.");
        return;
    };
    
    // Get available counter sets
    let counter_sets = match device.counter_sets() {
        Some(sets) => sets,
        None => {
            println!("No counter sets available. Exiting.");
            return;
        }
    };
    
    println!("Found {} counter sets:", counter_sets.count());
    for i in 0..counter_sets.count() {
        unsafe {
            let counter_set_ptr: *mut objc::runtime::Object = msg_send![counter_sets.as_ref(), objectAtIndex:i];
            let counter_set = metal_rs::metal::MTLCounterSet::from_ptr(counter_set_ptr);
            println!("  - {}", counter_set.name());
            
            if let Some(counters) = counter_set.counters() {
                println!("    Contains {} counters", counters.count());
                for j in 0..counters.count().min(3) { // Show only first 3 counters
                    let counter_ptr: *mut objc::runtime::Object = msg_send![counters.as_ref(), objectAtIndex:j];
                    let counter = metal_rs::metal::MTLCounter::from_ptr(counter_ptr);
                    println!("      - {}", counter.name());
                }
                if counters.count() > 3 {
                    println!("      ... and {} more", counters.count() - 3);
                }
            }
        }
    }
    
    // Look for the statistic counter set
    let mut statistic_set = None;
    for i in 0..counter_sets.count() {
        unsafe {
            let counter_set_ptr: *mut objc::runtime::Object = msg_send![counter_sets.as_ref(), objectAtIndex:i];
            let counter_set = metal_rs::metal::MTLCounterSet::from_ptr(counter_set_ptr);
            if counter_set.name() == common_counter_set::STATISTIC {
                statistic_set = Some(counter_set);
                break;
            }
        }
    }
    
    let statistic_set = match statistic_set {
        Some(set) => set,
        None => {
            println!("Statistic counter set not available. Exiting.");
            return;
        }
    };
    
    // Create a sample buffer descriptor
    let buffer_descriptor = MTLCounterSampleBufferDescriptor::new();
    buffer_descriptor.set_label("Compute Statistics");
    buffer_descriptor.set_storage_mode(MTLStorageMode::Shared);
    buffer_descriptor.set_sample_count(2); // Before and after our compute work
    buffer_descriptor.set_counter_set(&statistic_set);
    
    // Create a sample buffer
    let sample_buffer = match device.new_counter_sample_buffer(&buffer_descriptor) {
        Some(buffer) => buffer,
        None => {
            println!("Failed to create counter sample buffer. Exiting.");
            return;
        }
    };
    
    // Create input and output buffers for compute work
    let buffer_length = 1024;
    let buffer_size = buffer_length * size_of::<[f32; 4]>();
    
    let mut input_data = vec![[0.5f32, 0.6f32, 0.7f32, 0.8f32]; buffer_length];
    for i in 0..buffer_length {
        input_data[i] = [i as f32 * 0.01, 0.6, 0.7, 0.8];
    }
    
    let input_buffer = device.new_buffer_with_data(
        input_data.as_ptr() as *const _,
        buffer_size,
        MTLResourceOptions::CPUCacheModeDefaultCache
    );
    
    let output_buffer = device.new_buffer(
        buffer_size,
        MTLResourceOptions::CPUCacheModeDefaultCache
    );
    
    // Create command queue and buffer
    let command_queue = device.new_command_queue();
    let command_buffer = command_queue.new_command_buffer();
    
    // Set up compute encoder
    let compute_encoder = unsafe {
        use objc::runtime::Object;
        let encoder_ptr: *mut Object = msg_send![command_buffer.as_ref(), computeCommandEncoder];
        metal_rs::metal::MTLComputeCommandEncoder::from_ptr(encoder_ptr)
    };
    
    // Create and compile shader
    let compile_options = Default::default();
    let library = device.new_library_with_source(COMPUTE_SHADER, &compile_options).unwrap();
    let compute_function = library.get_function("count_kernel").unwrap();
    let compute_pipeline_state = device.new_compute_pipeline_state_with_function(&compute_function).unwrap();
    
    // Take first sample before computation
    compute_encoder.sample_counters_in_buffer(&sample_buffer, 0, true);
    
    // Set up computation
    compute_encoder.set_compute_pipeline_state(&compute_pipeline_state);
    compute_encoder.set_buffer(Some(input_buffer.as_ref()), 0, 0);
    compute_encoder.set_buffer(Some(output_buffer.as_ref()), 0, 1);
    
    // Dispatch threads
    let threads_per_grid = MTLSize::new(buffer_length, 1, 1);
    let threads_per_threadgroup = MTLSize::new(64, 1, 1);
    compute_encoder.dispatch_threads(threads_per_grid, threads_per_threadgroup);
    
    // Take second sample after computation
    compute_encoder.sample_counters_in_buffer(&sample_buffer, 1, true);
    
    // End encoding and commit
    compute_encoder.end_encoding();
    command_buffer.commit();
    
    // Wait for the command buffer to complete using objc calls
    unsafe {
        let _: () = msg_send![command_buffer.as_ref(), waitUntilCompleted];
    }
    
    // Resolve sample data
    let range = NSRange::new(0, 2); // Both samples
    let resolve_data_ptr = sample_buffer.resolve_counter_range(range).unwrap();
    
    // Get statistics by parsing the data
    let data_ptr = resolve_data_ptr as *const MTLCounterResultStatistic;
    
    println!("\nCounter results:");
    
    // Read the before and after samples
    let before_stats: MTLCounterResultStatistic;
    let after_stats: MTLCounterResultStatistic;
    
    unsafe {
        before_stats = *data_ptr.offset(0);
        after_stats = *data_ptr.offset(1);
    }
    
    println!("\nBefore computation:");
    println!("  - Compute kernel invocations: {}", before_stats.compute_kernel_invocations);
    
    println!("\nAfter computation:");
    println!("  - Compute kernel invocations: {}", after_stats.compute_kernel_invocations);
    
    // Calculate the difference to see what our computation did
    println!("\nStatistics from computation:");
    println!("  - Compute kernel invocations: {}", after_stats.compute_kernel_invocations - before_stats.compute_kernel_invocations);
    
    if after_stats.compute_kernel_invocations - before_stats.compute_kernel_invocations > 0 {
        println!("\nSuccessfully recorded and measured kernel invocations!");
    } else {
        println!("\nNo computation was recorded, something went wrong.");
    }
}