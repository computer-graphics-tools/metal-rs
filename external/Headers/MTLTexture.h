//
//  MTLTexture.h
//  Metal
//
//  Copyright (c) 2014 Apple Inc. All rights reserved.
//

#import <Foundation/Foundation.h>
#import <Metal/MTLDefines.h>
#import <Metal/MTLResource.h>
#import <Metal/MTLBuffer.h>
#import <Metal/MTLTypes.h>


#import <IOSurface/IOSurface.h>


NS_ASSUME_NONNULL_BEGIN


/*!
 @protocol MTLTexture
 @abstract MTLTexture represents a collection of 1D, 2D, or 3D images.
 @discussion
 Each image in a texture is a 1D, 2D, 2DMultisample, or 3D image. The texture contains one or more images arranged in a mipmap stack. If there are multiple mipmap stacks, each one is referred to as a slice of the texture. 1D, 2D, 2DMultisample, and 3D textures have a single slice. In 1DArray and 2DArray textures, every slice is an array element. A Cube texture always has 6 slices, one for each face. In a CubeArray texture, each set of six slices is one element in the array.
 
 Most APIs that operate on individual images in a texture address those images via a tuple of a Slice, and Mipmap Level within that slice.
 */
API_AVAILABLE(macos(10.11), ios(8.0))
@protocol MTLTexture <MTLResource>

/*!
 @property rootResource
 @abstract The resource this texture was created from. It may be a texture or a buffer. If this texture is not reusing storage of another MTLResource, then nil is returned.
 */
@property (nullable, readonly) id <MTLResource> rootResource API_DEPRECATED("Use parentTexture or buffer instead", macos(10.11, 10.12), ios(8.0, 10.0));

/*!
 @property parentTexture
 @abstract The texture this texture view was created from, or nil if this is not a texture view or it was not created from a texture.
 */
@property (nullable, readonly) id <MTLTexture> parentTexture API_AVAILABLE(macos(10.11), ios(9.0));

/*!
 @property parentRelativeLevel
 @abstract The base level of the texture this texture view was created from, or 0 if this is not a texture view.
 */
@property (readonly) NSUInteger parentRelativeLevel API_AVAILABLE(macos(10.11), ios(9.0));

/*!
 @property parentRelativeSlice
 @abstract The base slice of the texture this texture view was created from, or 0 if this is not a texture view.
 */
@property (readonly) NSUInteger parentRelativeSlice API_AVAILABLE(macos(10.11), ios(9.0));

/*!
 @property buffer
 @abstract The buffer this texture view was created from, or nil if this is not a texture view or it was not created from a buffer.
 */
@property (nullable, readonly) id <MTLBuffer> buffer API_AVAILABLE(macos(10.12), ios(9.0));

/*!
 @property bufferOffset
 @abstract The offset of the buffer this texture view was created from, or 0 if this is not a texture view.
 */
@property (readonly) NSUInteger bufferOffset API_AVAILABLE(macos(10.12), ios(9.0));

/*!
 @property bufferBytesPerRow
 @abstract The bytesPerRow of the buffer this texture view was created from, or 0 if this is not a texture view.
 */
@property (readonly) NSUInteger bufferBytesPerRow API_AVAILABLE(macos(10.12), ios(9.0));


/*!
 @property iosurface
 @abstract If this texture was created from an IOSurface, this returns a reference to that IOSurface. iosurface is nil if this texture was not created from an IOSurface.
 */
@property (nullable, readonly) IOSurfaceRef iosurface API_AVAILABLE(macos(10.11), ios(11.0));

/*!
 @property iosurfacePlane
 @abstract If this texture was created from an IOSurface, this returns the plane of the IOSurface from which the texture was created. iosurfacePlane is 0 if this texture was not created from an IOSurface.
 */
@property (readonly) NSUInteger iosurfacePlane API_AVAILABLE(macos(10.11), ios(11.0));

/*!
 @property type
 @abstract The type of this texture.
 */
@property (readonly) MTLTextureType textureType;

/*!
 @property pixelFormat
 @abstract The MTLPixelFormat that is used to interpret this texture's contents.
 */
@property (readonly) MTLPixelFormat pixelFormat;

/*!
 @property width
 @abstract The width of the MTLTexture instance in pixels.
 */
@property (readonly) NSUInteger width;

/*!
 @property height
 @abstract The height of the MTLTexture instance in pixels.
 @discussion. height is 1 if the texture is 1D.
 */
@property (readonly) NSUInteger height;

/*!
 @property depth
 @abstract The depth of this MTLTexture instance in pixels.
 @discussion If this MTLTexture is not a 3D texture, the depth is 1
 */
@property (readonly) NSUInteger depth;

/*!
 @property mipmapLevelCount
 @abstract The number of mipmap levels in each slice of this MTLTexture.
 */
@property (readonly) NSUInteger mipmapLevelCount;

/*!
 @property sampleCount
 @abstract The number of samples in each pixel of this MTLTexture.
 @discussion If this texture is any type other than 2DMultisample, samples is 1.
 */
@property (readonly) NSUInteger sampleCount;

/*!
 @property arrayLength
 @abstract The number of array elements in this MTLTexture.
 @discussion For non-Array texture types, arrayLength is 1.
 */
@property (readonly) NSUInteger arrayLength;

/*!
 @property usage
 @abstract Description of texture usage.
 */
@property (readonly) MTLTextureUsage usage;

/*!
 @property shareable
 @abstract If YES, this texture can be shared with other processes.
 @discussion Texture can be shared across process addres space boundaries through use of sharedTextureHandle and XPC.
 */
@property (readonly, getter = isShareable) BOOL shareable API_AVAILABLE(macos(10.14), ios(13.0));

/*!
 @property framebufferOnly
 @abstract If YES, this texture can only be used with a MTLAttachmentDescriptor, and cannot be used as a texture argument for MTLRenderCommandEncoder, MTLBlitCommandEncoder, or MTLComputeCommandEncoder. Furthermore, when this property's value is YES, readPixels/writePixels may not be used with this texture.
 @discussion Textures obtained from CAMetalDrawables may have this property set to YES, depending on the value of frameBufferOnly passed to their parent CAMetalLayer. Textures created directly by the application will not have any restrictions.
 */
@property (readonly, getter = isFramebufferOnly) BOOL framebufferOnly;

@optional
/*!
 @property firstMipmapInTail
 @abstract For sparse textures this property returns index of first mipmap that is packed in tail.
 Mapping this mipmap level will map all subsequent mipmap levels.
 */
@property (readonly) NSUInteger firstMipmapInTail API_AVAILABLE(macos(11.0), macCatalyst(14.0), ios(13.0), tvos(16.0));

/*!
 @property tailSizeInBytes
 @abstract Amount of memory in bytes required to map sparse texture tail.
 */
@property (readonly) NSUInteger tailSizeInBytes API_AVAILABLE(macos(11.0), macCatalyst(14.0), ios(13.0), tvos(16.0));

@property (readonly) BOOL isSparse API_AVAILABLE(macos(11.0), macCatalyst(14.0), ios(13.0), tvos(16.0));
@required

/*!
 @property allowGPUOptimizedContents
 @abstract Allow GPU-optimization for the contents texture. The default value is true.
 @discussion Useful for opting-out of GPU-optimization when implicit optimization (e.g. RT writes) is regressing CPU-read-back performance. See the documentation for optimizeContentsForGPUAccess: and optimizeContentsForCPUAccess: APIs.
 */
@property (readonly) BOOL allowGPUOptimizedContents API_AVAILABLE(macos(10.14), ios(12.0));

/*!
 @property compressionType
 @abstract Returns the compression type of the texture
 @discussion See the compressionType property on MTLTextureDescriptor
 */
@property (readonly) MTLTextureCompressionType compressionType API_AVAILABLE(macos(12.5), ios(15.0), tvos(16.0));


/*!
 @property gpuResourceID
 @abstract Handle of the GPU resource suitable for storing in an Argument Buffer
 */
@property (readonly) MTLResourceID gpuResourceID API_AVAILABLE(macos(13.0), ios(16.0));

/*!
 @method getBytes:bytesPerRow:bytesPerImage:fromRegion:mipmapLevel:slice:
 @abstract Copies a block of pixels from a texture slice into the application's memory.
 */
- (void)getBytes:(void *)pixelBytes bytesPerRow:(NSUInteger)bytesPerRow bytesPerImage:(NSUInteger)bytesPerImage fromRegion:(MTLRegion)region mipmapLevel:(NSUInteger)level slice:(NSUInteger)slice;

/*!
 @method replaceRegion:mipmapLevel:slice:withBytes:bytesPerRow:bytesPerImage:
 @abstract Copy a block of pixel data from the caller's pointer into a texture slice.
 */
- (void)replaceRegion:(MTLRegion)region mipmapLevel:(NSUInteger)level slice:(NSUInteger)slice withBytes:(const void *)pixelBytes bytesPerRow:(NSUInteger)bytesPerRow bytesPerImage:(NSUInteger)bytesPerImage;

/*!
 @method getBytes:bytesPerRow:fromRegion:mipmapLevel:
 @abstract Convenience for getBytes:bytesPerRow:bytesPerImage:fromRegion:mipmapLevel:slice: that doesn't require slice related arguments
 */
- (void)getBytes:(void *)pixelBytes bytesPerRow:(NSUInteger)bytesPerRow fromRegion:(MTLRegion)region mipmapLevel:(NSUInteger)level;

/*!
 @method replaceRegion:mipmapLevel:withBytes:bytesPerRow:
 @abstract Convenience for replaceRegion:mipmapLevel:slice:withBytes:bytesPerRow:bytesPerImage: that doesn't require slice related arguments
 */
- (void)replaceRegion:(MTLRegion)region mipmapLevel:(NSUInteger)level withBytes:(const void *)pixelBytes bytesPerRow:(NSUInteger)bytesPerRow;

/*!
 @method newTextureViewWithPixelFormat:
 @abstract Create a new texture which shares the same storage as the source texture, but with a different (but compatible) pixel format.
 */
- (nullable id<MTLTexture>)newTextureViewWithPixelFormat:(MTLPixelFormat)pixelFormat;

/*!
 @method newTextureViewWithPixelFormat:textureType:levels:slices:
 @abstract Create a new texture which shares the same storage as the source texture, but with a different (but compatible) pixel format, texture type, levels and slices.
 */
- (nullable id<MTLTexture>)newTextureViewWithPixelFormat:(MTLPixelFormat)pixelFormat textureType:(MTLTextureType)textureType levels:(NSRange)levelRange slices:(NSRange)sliceRange API_AVAILABLE(macos(10.11), ios(9.0));

/*!
 @method newSharedTextureHandle
 @abstract Create a new texture handle, that can be shared across process addres space boundaries.
 */
- (nullable MTLSharedTextureHandle *)newSharedTextureHandle API_AVAILABLE(macos(10.14), ios(13.0));

/*!
 @method newTextureViewWithDescriptor:
 @abstract Create a new texture which shares the same storage as the source texture, but with different (but compatible) properties specified by the descriptor
 */
- (nullable id<MTLTexture>)newTextureViewWithDescriptor:(MTLTextureViewDescriptor *)descriptor API_AVAILABLE(macos(26.0), ios(26.0));


/*!
 @property remoteStorageTexture
 @abstract For Metal texture objects that are remote views, this returns the texture associated with the storage on the originating device.
 */
@property (nullable, readonly) id<MTLTexture> remoteStorageTexture API_AVAILABLE(macos(10.15)) API_UNAVAILABLE(ios);

/*!
 @method newRemoteTextureViewForDevice:
 @abstract On Metal devices that support peer to peer transfers, this method is used to create a remote texture view on another device
 within the peer group.  The receiver must use MTLStorageModePrivate or be backed by an IOSurface.
 */
- (nullable id <MTLTexture>) newRemoteTextureViewForDevice:(id <MTLDevice>)device API_AVAILABLE(macos(10.15)) API_UNAVAILABLE(ios);

/*!
 @property swizzle
 @abstract The channel swizzle used when reading or sampling from this texture
 */
@property (readonly, nonatomic) MTLTextureSwizzleChannels swizzle API_AVAILABLE(macos(10.15), ios(13.0));

/*!
 @method newTextureViewWithPixelFormat:textureType:levels:slices:swizzle:
 @abstract Create a new texture which shares the same storage as the source texture, but with a different (but compatible) pixel format, texture type, levels, slices and swizzle. 
 */
- (nullable id<MTLTexture>)newTextureViewWithPixelFormat:(MTLPixelFormat)pixelFormat textureType:(MTLTextureType)textureType levels:(NSRange)levelRange slices:(NSRange)sliceRange swizzle:(MTLTextureSwizzleChannels)swizzle API_AVAILABLE(macos(10.15), ios(13.0));

/*!
 @property sparseTextureTier
 @abstract Query support tier for sparse textures.
 */
@property (readonly) MTLTextureSparseTier sparseTextureTier API_AVAILABLE(macos(26.0), ios(26.0));

@end
NS_ASSUME_NONNULL_END
