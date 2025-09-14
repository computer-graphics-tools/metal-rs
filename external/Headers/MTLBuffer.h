//
//  MTLBuffer.h
//  Metal
//
//  Copyright (c) 2014 Apple Inc. All rights reserved.
//

#import <Foundation/Foundation.h>
#import <Metal/MTLDefines.h>
#import <Metal/MTLPixelFormat.h>
#import <Metal/MTLResource.h>
#import <Metal/MTLTensor.h>
#import <Metal/MTLGPUAddress.h>

NS_ASSUME_NONNULL_BEGIN

@class MTLTextureDescriptor;
@protocol MTLTexture;
@protocol MTLResource;

/*!
 @protocol MTLBuffer
 @abstract A typeless allocation accessible by both the CPU and the GPU (MTLDevice) or by only the GPU when the storage mode is
 MTLResourceStorageModePrivate.
 
 @discussion
 Unlike in OpenGL and OpenCL, access to buffers is not synchronized.  The caller may use the CPU to modify the data at any time
 but is also responsible for ensuring synchronization and coherency.
 
 The contents become undefined if both the CPU and GPU write to the same buffer without a synchronizing action between those writes.
 This is true even when the regions written do not overlap.
 */
API_AVAILABLE(macos(10.11), ios(8.0))
@protocol MTLBuffer <MTLResource>

/// Creates a tensor that shares storage with this buffer.
///
/// - Parameters:
///   - descriptor: A description of the properties for the new tensor.
///   - offset: Offset into the buffer at which the data of the tensor begins.
///   - error: If an error occurs during creation, Metal populates this parameter to provide you information about it.
///
/// If the descriptor specifies `MTLTensorUsageMachineLearning` usage, you need to observe the following restrictions:
/// * pass in `0` for the `offset` parameter
/// * set the element stride the descriptor to `1`
/// * ensure that number of bytes per row is a multiple of `64`
/// * for dimensions greater than `2`, make sure `strides[dim] = strides[dim -1] * dimensions[dim - 1]`
///
- (nullable id <MTLTensor>)newTensorWithDescriptor:(MTLTensorDescriptor *)descriptor
                                            offset:(NSUInteger)offset
                                             error:(__autoreleasing NSError * _Nullable * _Nullable)error API_AVAILABLE(macos(26.0), ios(26.0));

@end
NS_ASSUME_NONNULL_END
