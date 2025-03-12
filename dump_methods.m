#import <Foundation/Foundation.h>
#import <Metal/Metal.h>
#include <objc/runtime.h>

void dumpClassMethods(Class cls, NSString *className) {
    unsigned int methodCount = 0;
    Method *methods = class_copyMethodList(cls, &methodCount);
    
    NSLog(@"Methods for class %@:", className);
    for (unsigned int i = 0; i < methodCount; i++) {
        Method method = methods[i];
        SEL selector = method_getName(method);
        NSLog(@"  %@", NSStringFromSelector(selector));
    }
    
    free(methods);
}

int main() {
    id<MTLDevice> device = MTLCreateSystemDefaultDevice();
    NSString *source = @"kernel void test() { }";
    
    NSError *error = nil;
    id<MTLLibrary> library = [device newLibraryWithSource:source options:nil error:&error];
    
    if (!library) {
        NSLog(@"Failed to create library: %@", error);
        return 1;
    }
    
    id<MTLFunction> function = [library newFunctionWithName:@"test"];
    
    if (!function) {
        NSLog(@"Failed to get function");
        return 1;
    }
    
    NSLog(@"Function class: %@", NSStringFromClass([function class]));
    dumpClassMethods([function class], NSStringFromClass([function class]));
    
    // Try to create a function handle
    @try {
        id handle = [function newFunctionHandle];
        if (handle) {
            NSLog(@"Successfully created function handle: %@", handle);
            NSLog(@"Function handle class: %@", NSStringFromClass([handle class]));
            dumpClassMethods([handle class], NSStringFromClass([handle class]));
        } else {
            NSLog(@"Failed to create function handle (returned nil)");
        }
    } @catch (NSException *exception) {
        NSLog(@"Exception when creating function handle: %@", exception);
    }
    
    return 0;
}
