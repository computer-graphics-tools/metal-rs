//
//  MTLTensor.h
//  Metal
//
//  Created by Vatsin Suchak on 2024/9/17.
//  Copyright © 2024 Apple, Inc. All rights reserved.
//

#ifndef MTLTensor_h
#define MTLTensor_h

#import <Metal/MTLDefines.h>
#import <Metal/MTLTypes.h>
#import <Metal/MTLResource.h>
#import <Metal/MTLDataType.h>


NS_ASSUME_NONNULL_BEGIN

@protocol MTLBuffer;

/// An error domain for errors that pertain to creating a tensor.
MTL_EXTERN NSErrorDomain const MTLTensorDomain API_AVAILABLE(macos(26.0), ios(26.0));

/// The error codes that Metal can raise when you create a tensor.
typedef NS_ENUM(NSInteger, MTLTensorError)
{
    MTLTensorErrorNone              = 0,
    MTLTensorErrorInternalError     = 1,
    MTLTensorErrorInvalidDescriptor = 2,
} API_AVAILABLE(macos(26.0), ios(26.0));

/// The type that represents the different contexts for a tensor.
typedef NS_OPTIONS(NSUInteger, MTLTensorUsage)
{
    /// A tensor context that applies to compute encoders.
    ///
    /// You can use tensors with this context in ``MTL4ComputeCommandEncoder`` or ``MTLComputeCommandEncoder`` instances.
    MTLTensorUsageCompute = 1 << 0,
    /// A tensor context that applies to render encoders.
    ///
    /// You can use tensors with this context in ``MTL4RenderCommandEncoder`` or ``MTLRenderCommandEncoder`` instances.
    MTLTensorUsageRender  = 1 << 1,
    /// A tensor context that applies to machine learning encoders.
    ///
    /// You can use tensors with this context in ``MTL4MachineLearningCommandEncoder`` instances.
    MTLTensorUsageMachineLearning      = 1 << 2,
} API_AVAILABLE(macos(26.0), ios(26.0));


/// A configuration type for creating new tensor instances.
MTL_EXPORT API_AVAILABLE(macos(26.0), ios(26.0))
@interface MTLTensorDescriptor : NSObject <NSCopying>

/// An array of sizes, in elements, one for each dimension of the tensors you create with this descriptor.
///
/// The default value of this property is a rank one extents with size one.
@property (readwrite, nonatomic, copy) MTLTensorExtents *dimensions;

/// An array of strides, in elements, one for each dimension in the tensors you create with this descriptor, if applicable.
///
/// This property only applies to tensors you create from a buffer, otherwise it is nil. You are responsible for ensuring `strides` meets the following requirements:
/// - Elements of `strides`are in monotonically non-decreasing order.
/// - The first element of `strides` is one.
/// - For any `i` larger than zero, `strides[i]` is greater than or equal to `strides[i-1] * dimensions[i-1]`.
/// - If `usage` contains ``MTLTensorUsage/MTLTensorUsageMachineLearning``, the second element of `strides` is aligned to 64 bytes, and for any `i` larger than one, `strides[i]` is equal to `strides[i-1] * dimensions[i-1]`.
@property (readwrite, nonatomic, copy, nullable) MTLTensorExtents *strides;

/// A data format for the tensors you create with this descriptor.
///
/// The default value of this property is ``MTLTensorDataType/MTLTensorDataTypeFloat32``.
@property (readwrite, nonatomic) MTLTensorDataType dataType;

/// A set of contexts in which you can use tensors you create with this descriptor.
///
/// The default value for this property is a bitwise `OR` of:
/// - ``MTLTensorUsage/MTLTensorUsageRender``
/// - ``MTLTensorUsage/MTLTensorUsageCompute``
@property (readwrite, nonatomic) MTLTensorUsage usage;

/// A packed set of the `storageMode`, `cpuCacheMode` and `hazardTrackingMode` properties.
@property (readwrite, nonatomic) MTLResourceOptions resourceOptions;

/// A value that configures the cache mode of CPU mapping of tensors you create with this descriptor.
///
/// The default value of this property is ``MTLCPUCacheMode/MTLCPUCacheModeDefaultCache``.
@property (readwrite, nonatomic) MTLCPUCacheMode cpuCacheMode;

/// A value that configures the memory location and access permissions of tensors you create with this descriptor.
///
/// The default value of this property defaults to ``MTLStorageMode/MTLStorageModeShared``.
@property (readwrite, nonatomic) MTLStorageMode storageMode;

/// A value that configures the hazard tracking of tensors you create with this descriptor.
///
/// The default value of this property is ``MTLHazardTrackingMode/MTLHazardTrackingModeDefault``.
@property (readwrite, nonatomic) MTLHazardTrackingMode hazardTrackingMode;

@end


/// A resource representing a multi-dimensional array that you can use with machine learning workloads.
MTL_EXTERN API_AVAILABLE(macos(26.0), ios(26.0))
@protocol MTLTensor <MTLResource>

/// A handle that represents the GPU resource, which you can store in an argument buffer.
@property (readonly) MTLResourceID gpuResourceID;

/// A buffer instance this tensor shares its storage with or nil if this tensor does not wrap an underlying buffer.
@property (nullable, readonly) id<MTLBuffer> buffer;

/// An offset, in bytes, into the buffer instance this tensor shares its storage with, or zero if this tensor does not wrap an underlying buffer.
@property (readonly) NSUInteger bufferOffset;

/// An array of strides, in elements, one for each dimension of this tensor.
///
/// This property only applies if this tensor shares its storage with a buffer, otherwise it's nil.
@property (nullable, readonly) MTLTensorExtents *strides;

/// An array of sizes, in elements, one for each dimension of this tensor.
@property (readonly) MTLTensorExtents *dimensions;

/// An underlying data format of this tensor.
@property (readonly) MTLTensorDataType dataType;

/// A set of contexts in which you can use this tensor.
@property (readonly) MTLTensorUsage usage;

/// Replaces the contents of a slice of this tensor with data you provide.
///
/// - Parameters:
///   - sliceOrigin: An array of offsets, in elements, to the first element of the slice that this method writes data to.
///   - sliceDimensions: An array of sizes, in elements, of the slice this method writes data to.
///   - bytes: A pointer to bytes of data that this method copies into the slice you specify with `sliceOrigin` and `sliceDimensions`.
///   - strides: An array of strides, in elements, that describes the layout of the data in `bytes`. You are responsible for ensuring `strides` meets the following requirements:
///     - Elements of `strides`are in monotonically non-decreasing order.
///     - For any `i` larger than zero, `strides[i]` is greater than or equal to `strides[i-1] * dimensions[i-1]`.
- (void)replaceSliceOrigin:(MTLTensorExtents *)sliceOrigin
           sliceDimensions:(MTLTensorExtents *)sliceDimensions
                 withBytes:(const void *)bytes
                   strides:(MTLTensorExtents *)strides;

/// Copies the data corresponding to a slice of this tensor into a pointer you provide.
///
/// - Parameters:
///   - bytes: A pointer to bytes of data that this method copies into the slice you specify with `sliceOrigin` and `sliceDimensions`.
///   - strides: An array of strides, in elements, that describes the layout of the data in `bytes`. You are responsible for ensuring `strides` meets the following requirements:
///     - Elements of `strides`are in monotonically non-decreasing order.
///     - For any `i` larger than zero, `strides[i]` is greater than or equal to `strides[i-1] * dimensions[i-1]`.
///   - sliceOrigin: An array of offsets, in elements, to the first element of the slice that this method reads data from.
///   - sliceDimensions: An array of sizes, in elements, of the slice this method reads data from.
- (void)getBytes:(void *)bytes
         strides:(MTLTensorExtents *)strides
 fromSliceOrigin:(MTLTensorExtents *)sliceOrigin
 sliceDimensions:(MTLTensorExtents *)sliceDimensions;

@end

NS_ASSUME_NONNULL_END



#endif // MTLTensor_h
