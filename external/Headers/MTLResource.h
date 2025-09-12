//
//  MTLResource.h
//  Metal
//
//  Copyright (c) 2014 Apple Inc. All rights reserved.
//

#import <Foundation/Foundation.h>
#import <Metal/MTLDefines.h>
#import <Metal/MTLAllocation.h>
#import <mach/mach.h>

NS_ASSUME_NONNULL_BEGIN

/*!
 @enum MTLSparsePageSize
 @abstract Physical size of sparse resource page in KBs.
 */
typedef NS_ENUM(NSInteger, MTLSparsePageSize)
{
    MTLSparsePageSize16 = 101,
    MTLSparsePageSize64 = 102,
    MTLSparsePageSize256 = 103,
} API_AVAILABLE(macos(13.0), ios(16.0));


/// Enumerates the different support levels for sparse buffers.
typedef NS_ENUM(NSInteger, MTLBufferSparseTier)
{
    /// Indicates that the buffer is not sparse.
    MTLBufferSparseTierNone = 0,
    
    /// Indicates support for sparse buffers tier 1.
    ///
    /// Tier 1 sparse buffers allow the following:
    /// * Partial memory backing at sparse page granularity.
    /// * Defined behavior for accessing an *unbacked* buffer range.
    ///
    /// An unbacked buffer range indicates a range within the buffer that doesn't
    /// have memory backing at a given point in time. Accessing an unbacked buffer
    /// range of a sparse buffer produces the following results:
    /// * Reading return zero.
    /// * Writing produces no result.
    MTLBufferSparseTier1 = 1,
} API_AVAILABLE(macos(26.0), ios(26.0));

/// Enumerates the different support levels for sparse textures.
typedef NS_ENUM(NSInteger, MTLTextureSparseTier)
{
    /// Indicates that the texture is not sparse.
    MTLTextureSparseTierNone = 0,
    
    /// Indicates support for sparse textures tier 1.
    ///
    /// Tier 1 sparse textures allow the following:
    /// * Partial memory backing at sparse tile granularity.
    /// * Defined behavior for accessing an unbacked texture region.
    /// * Shader feedback on texture access to determine memory backing.
    ///
    /// An unbacked texture region indicates a region within the texture that doesn't
    /// have memory backing at a given point in time. Accessing an unbacked texture
    /// region produces the following results:
    /// * Reading returns zero (transparent black) for pixel formats with an alpha (A) channel.
    /// * Reading return zero in RGB and one in alpha (A) channels (opaque black) otherwise.
    /// * Writing produces no result.
    MTLTextureSparseTier1 = 1,
    
    /// Indicates support for sparse textures tier 2.
    ///
    /// In addition to the guarantees tier 1 sparse textures provide,
    /// tier 2 sparse textures allow the following:
    /// * Obtain per-tile activity counters.
    MTLTextureSparseTier2 = 2,
} API_AVAILABLE(macos(26.0), ios(26.0));


@protocol MTLDevice;

@protocol MTLHeap;

/*!
 @protocol MTLResource
 @abstract Common APIs available for MTLBuffer and MTLTexture instances
 */
API_AVAILABLE(macos(10.11), ios(8.0))
@protocol MTLResource <MTLAllocation>

/*!
 @property label
 @abstract A string to help identify this object.
 */
@property (nullable, copy, atomic) NSString *label;

/*!
 @property device
 @abstract The device this resource was created against.  This resource can only be used with this device.
 */
@property (readonly) id <MTLDevice> device;

/*!
 @property cpuCacheMode
 @abstract The cache mode used for the CPU mapping for this resource
 */
@property (readonly) MTLCPUCacheMode cpuCacheMode;

/*!
 @property storageMode
 @abstract The resource storage mode used for the CPU mapping for this resource
 */
@property (readonly) MTLStorageMode storageMode API_AVAILABLE(macos(10.11), ios(9.0));

/*!
 @property hazardTrackingMode
 @abstract Whether or not the resource is hazard tracked.
 @discussion This value can be either MTLHazardTrackingModeUntracked or MTLHazardTrackingModeTracked.
 Resources created from heaps are by default untracked, whereas resources created from the device are by default tracked.
 */
@property (readonly) MTLHazardTrackingMode hazardTrackingMode API_AVAILABLE(macos(10.15), ios(13.0));

/*!
 @property resourceOptions
 @abstract A packed tuple of the storageMode, cpuCacheMode and hazardTrackingMode properties.
 */
@property (readonly) MTLResourceOptions resourceOptions API_AVAILABLE(macos(10.15), ios(13.0));

/*!
 @method setPurgeableState
 @abstract Set (or query) the purgeability state of a resource
 @discussion Synchronously set the purgeability state of a resource and return what the prior (or current) state is.
 FIXME: If the device is keeping a cached copy of the resource, both the shared copy and cached copy are made purgeable.  Any access to the resource by either the CPU or device will be undefined.
 */
- (MTLPurgeableState)setPurgeableState:(MTLPurgeableState)state;

/*!
 @property heap
 @abstract The heap from which this resouce was created.
 @discussion Nil when this resource is not backed by a heap.
 */
@property (readonly, nullable) id <MTLHeap> heap API_AVAILABLE(macos(10.13), ios(10.0));

/*!
 @property heapOffset
 @abstract The offset inside the heap at which this resource was created.
 @discussion Zero when this resource was not created on a heap with MTLHeapTypePlacement.
 */
@property (readonly) NSUInteger heapOffset API_AVAILABLE(macos(10.15), ios(13.0));

/*!
 @property allocatedSize
 @abstract The size in bytes occupied by this resource
*/
@property (readonly) NSUInteger allocatedSize API_AVAILABLE(macos(10.13), ios(11.0));

/*!
 @method makeAliasable
 @abstract Allow future heap sub-allocations to alias against this resource's memory.
 @discussion It is illegal to call this method on a non heap-based resource. 
 It is also illegal to call this method on texture views created from heap-based textures.
 The debug layer will raise an exception. Calling this method on textures sub-allocated
 from Buffers backed by heap memory has no effect.
 Once a resource is made aliasable, the decision cannot be reverted.
 */
-(void) makeAliasable API_AVAILABLE(macos(10.13), ios(10.0));

/*!
 @method isAliasable
 @abstract Returns whether future heap sub-allocations may alias against this resource's memory.
 @return YES if <st>makeAliasable</st> was previously successfully called on this resource. NO otherwise.
 If resource is sub-allocated from other resource created on the heap, isAliasable returns 
 aliasing state of that base resource. Also returns NO when storage mode is memoryless.
 */
-(BOOL) isAliasable API_AVAILABLE(macos(10.13), ios(10.0));

/*!
 @method setOwnerWithIdentity:
 @abstract Assigns ownership of the resource's underlying memory to another task for the purposes of VM accounting.
*/
- (kern_return_t)setOwnerWithIdentity:(task_id_token_t)task_id_token API_AVAILABLE(ios(17.4), watchos(10.4), tvos(17.4), macos(14.4));
@end


NS_ASSUME_NONNULL_END
