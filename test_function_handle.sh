#!/bin/bash
cd /Users/eugenebokhan/Developer/computer-graphics-tools/metal-rs

# Build the project
echo "Building project..."
cargo build

# Run the example
echo "Running function_handle example..."
RUST_BACKTRACE=1 cargo run --example function_handle

# Compile a simple test to check for any potential issues
echo "Creating and running a simple test program..."
cat > test_function_handle.rs << 'EOF'
use metal_rs::metal::{MTLCreateSystemDefaultDevice, MTLFunctionType};

fn main() {
    let device = MTLCreateSystemDefaultDevice();
    println!("Device: {:?}", device);
    
    let source = r#"
        #include <metal_stdlib>
        using namespace metal;
        
        kernel void test(device float* buf [[buffer(0)]]) {
            buf[0] = 42.0;
        }
    "#;
    
    let library = device.new_library_with_source(source, &Default::default()).unwrap();
    println!("Library: {:?}", library);
    
    let function = library.get_function("test").unwrap();
    println!("Function: {:?}", function);
    println!("Function type: {:?}", function.function_type());
    
    let handle = function.new_function_handle();
    println!("Function handle: {:?}", handle);
    println!("Handle name: {}", handle.name());
    println!("Handle type: {:?}", handle.function_type());
}
EOF

rustc -L target/debug/deps -L target/debug --extern metal_rs=target/debug/libmetal_rs.rlib test_function_handle.rs -o test_function_handle
./test_function_handle