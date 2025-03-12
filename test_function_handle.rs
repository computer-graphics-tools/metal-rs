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
