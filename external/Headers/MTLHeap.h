//
//  MTLHeap.h
//  Metal
//
//  Copyright (c) 2016 Apple Inc. All rights reserved.
//

#import <Foundation/Foundation.h>
#import <Metal/MTLDefines.h>
#import <Metal/MTLAllocation.h>
#import <Metal/MTLResource.h>
#import <Metal/MTLBuffer.h>
#import <Metal/MTLTexture.h>
#import <Metal/MTLTypes.h>
#import <Metal/MTLAccelerationStructure.h>

NS_ASSUME_NONNULL_BEGIN


/*!
 @protocol MTLHeap
 */
API_AVAILABLE(macos(10.13), ios(10.0))
@protocol MTLHeap <MTLAllocation>

/*!
 @property label
 @abstract A string to help identify this heap.
 */
@property (nullable, copy, atomic) NSString *label;

/*!
 @property device
 @abstract The device this heap was created against. This heap can only be used with this device.
 */
@property (readonly) id <MTLDevice> device;

/*!
 @property storageMode
 @abstract Current heap storage mode, default is MTLStorageModePrivate.
 @discussion All resources created from this heap share the same storage mode.
 */
@property (readonly) MTLStorageMode storageMode;

/*!
 @property cpuCacheMode
 @abstract CPU cache mode for the heap. Default is MTLCPUCacheModeDefaultCache.
 @discussion All resources created from this heap share the same cache mode.
 */
@property (readonly) MTLCPUCacheMode cpuCacheMode;

/*!
 @property hazardTrackingMode
 @abstract Whether or not the heap is hazard tracked.
 @discussion
 When a resource on a hazard tracked heap is modified, reads and writes from any other resource on that heap will be delayed until the modification is complete.
 Similarly, modifying heap resources will be delayed until all in-flight reads and writes from resources suballocated on that heap have completed.
 For optimal performance, perform hazard tracking manually through MTLFence or MTLEvent instead.
 Resources on the heap may opt-out of hazard tracking individually when the heap is hazard tracked,
 however resources cannot opt-in to hazard tracking when the heap is not hazard tracked.
 */
@property (readonly) MTLHazardTrackingMode hazardTrackingMode API_AVAILABLE(macos(10.15), ios(13.0));

/*!
 @property resourceOptions
 @abstract A packed tuple of the storageMode, cpuCacheMode and hazardTrackingMode properties.
 */
@property (readonly) MTLResourceOptions resourceOptions API_AVAILABLE(macos(10.15), ios(13.0));

/*!
 @property size
 @abstract Heap size in bytes, specified at creation time and rounded up to device specific alignment.
 */
@property (readonly) NSUInteger size;


/*!
 @property usedSize
 @abstract The size in bytes, of all resources allocated from the heap.
 */
@property (readonly) NSUInteger usedSize;

/*!
 @property currentAllocatedSize
 @abstract The size in bytes of the current heap allocation.
 */
@property (readonly) NSUInteger currentAllocatedSize API_AVAILABLE(macos(10.13), ios(11.0));

/*!
 @method maxAvailableSizeWithAlignment:
 @abstract The maximum size that can be successfully allocated from the heap in bytes, taking into notice given alignment. Alignment needs to be zero, or power of two.
 @discussion Provides a measure of fragmentation within the heap.
 */
- (NSUInteger)maxAvailableSizeWithAlignment:(NSUInteger)alignment;

/*!
 @method newBufferWithLength:options:
 @abstract Create a new buffer backed by heap memory.
 @discussion The requested storage and CPU cache modes must match the storage and CPU cache modes of the heap.
 @return The buffer or nil if heap is full.
 */
- (nullable id <MTLBuffer>)newBufferWithLength:(NSUInteger)length
                                       options:(MTLResourceOptions)options;

/*!
 @method newTextureWithDescriptor:
 @abstract Create a new texture backed by heap memory.
 @discussion The requested storage and CPU cache modes must match the storage and CPU cache modes of the heap, with the exception that the requested storage mode can be MTLStorageModeMemoryless. 
 @return The texture or nil if heap is full.
 */
- (nullable id <MTLTexture>)newTextureWithDescriptor:(MTLTextureDescriptor *)descriptor;

/*!
 @method setPurgeabilityState:
 @abstract Set or query the purgeability state of the heap.
 */
- (MTLPurgeableState)setPurgeableState:(MTLPurgeableState)state;

/*!
 @property type
 @abstract The type of the heap. The default value is MTLHeapTypeAutomatic.
 @discussion This constrains the resource creation functions that are available on the heap.
 */
@property (readonly) MTLHeapType type API_AVAILABLE(macos(10.15), ios(13.0));
 
/*!
 @method newBufferWithLength:options:offset:
 @abstract Create a new buffer backed by heap memory at the specified placement offset.
 @discussion This method can only be used when heapType is set to MTLHeapTypePlacement.
 Use "MTLDevice heapBufferSizeAndAlignWithLength:options:" to determine requiredSize and requiredAlignment.
 Any resources that exist in this heap at overlapping half-open range [offset, offset + requiredSize) are implicitly aliased with the new resource.
 @param length The requested size of the buffer, in bytes.
 @param options The requested options of the buffer, of which the storage and CPU cache mode must match these of the heap.
 @param offset The requested offset of the buffer inside the heap, in bytes. Behavior is undefined if "offset + requiredSize > heap.size" or "offset % requiredAlignment != 0".
 @return The buffer, or nil if the heap is not a placement heap
 */
- (nullable id<MTLBuffer>)newBufferWithLength:(NSUInteger)length
                                      options:(MTLResourceOptions)options
                                       offset:(NSUInteger)offset API_AVAILABLE(macos(10.15), ios(13.0));

/*!
 @method newTextureWithDescriptor:offset:
 @abstract Create a new texture backed by heap memory at the specified placement offset.
 @discussion This method can only be used when heapType is set to MTLHeapTypePlacement.
 Use "MTLDevice heapTextureSizeAndAlignWithDescriptor:" to determine requiredSize and requiredAlignment.
 Any resources that exist in this heap at overlapping half-open range [offset, offset + requiredSize) are implicitly aliased with the new resource.
 @param descriptor The requested properties of the texture, of which the storage and CPU cache mode must match those of the heap.
 @param offset The requested offset of the texture inside the heap, in bytes. Behavior is undefined if "offset + requiredSize > heap.size" and "offset % requiredAlignment != 0".
 @return The texture, or nil if the heap is not a placement heap.
 */
- (nullable id<MTLTexture>)newTextureWithDescriptor:(MTLTextureDescriptor *)descriptor
                                             offset:(NSUInteger)offset API_AVAILABLE(macos(10.15), ios(13.0));

/*!
 @method newAccelerationStructureWithSize:
 @abstract Create a new acceleration structure backed by heap memory.
 @return The acceleration structure or nil if heap is full. Note that the MTLAccelerationStructure merely represents storage for an acceleration structure. It will still need to be populated via a build, copy, refit, etc.
 */
- (nullable id <MTLAccelerationStructure>)newAccelerationStructureWithSize:(NSUInteger)size API_AVAILABLE(macos(13.0), ios(16.0));

/*!
 @method newAccelerationStructureWithDescriptor:
 @abstract Create a new acceleration structure backed by heap memory.
 @discussion This is a convenience method which creates the acceleration structure backed by heap memory. The acceleration structure size is inferred based on the descriptor.
 @return The acceleration structure or nil if heap is full. Note that the MTLAccelerationStructure merely represents storage for an acceleration structure. It will still need to be populated via a build, copy, refit, etc.
 */
- (nullable id <MTLAccelerationStructure>)newAccelerationStructureWithDescriptor:(MTLAccelerationStructureDescriptor *)descriptor API_AVAILABLE(macos(13.0), ios(16.0));

/*!
 @method newAccelerationStructureWithSize:offset:
 @abstract Create a new acceleration structure backed by heap memory at the specified placement offset.
 @discussion This method can only be used when heapType is set to MTLHeapTypePlacement.
 Use "MTLDevice heapAccelerationStructureSizeAndAlignWithSize:" or "MTLDevice heapAccelerationStructureSizeAndAlignWithDescriptor:" to determine requiredSize and requiredAlignment.
 Any resources that exist in this heap at overlapping half-open range [offset, offset + requiredSize) are implicitly aliased with the new resource.
 @param size The requested size of the acceleration structure, in bytes.
 @param offset The requested offset of the acceleration structure inside the heap, in bytes. Behavior is undefined if "offset + requiredSize > heap.size" or "offset % requiredAlignment != 0".
 @return The acceleration structure, or nil if the heap is not a placement heap
 */
- (nullable id <MTLAccelerationStructure>)newAccelerationStructureWithSize:(NSUInteger)size offset:(NSUInteger)offset API_AVAILABLE(macos(13.0), ios(16.0));

/*!
 @method newAccelerationStructureWithDescriptor:offset:
 @abstract Create a new acceleration structure backed by heap memory at the specified placement offset.
 @discussion This is a convenience method which computes the acceleration structure size based on the descriptor.
 This method can only be used when heapType is set to MTLHeapTypePlacement.
 Use "MTLDevice heapAccelerationStructureSizeAndAlignWithSize:" or "MTLDevice heapAccelerationStructureSizeAndAlignWithDescriptor:" to determine requiredSize and requiredAlignment.
 Any resources that exist in this heap at overlapping half-open range [offset, offset + requiredSize) are implicitly aliased with the new resource.
 @param descriptor The acceleration structure descriptor
 @param offset The requested offset of the acceleration structure inside the heap, in bytes. Behavior is undefined if "offset + requiredSize > heap.size" or "offset % requiredAlignment != 0".
 @return The acceleration structure, or nil if the heap is not a placement heap
 */
- (nullable id <MTLAccelerationStructure>)newAccelerationStructureWithDescriptor:(MTLAccelerationStructureDescriptor *)descriptor offset:(NSUInteger)offset API_AVAILABLE(macos(13.0), ios(16.0));


@end

NS_ASSUME_NONNULL_END
