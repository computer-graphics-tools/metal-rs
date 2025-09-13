use objc2::{extern_protocol, rc::Retained, runtime::ProtocolObject};
use objc2_io_surface::IOSurfaceRef;
use std::{os::raw::c_void, ptr::NonNull};

use crate::{
    Device, PixelFormat, Region, Resource, ResourceID, SharedTextureHandle, TextureCompressionType,
    TextureSwizzleChannels, TextureType, TextureUsage,
};

extern_protocol!(
    /// MTLTexture represents a collection of 1D, 2D, or 3D images.
    ///
    /// Each image in a texture is a 1D, 2D, 2DMultisample, or 3D image. The texture contains one or more images arranged in a mipmap stack. If there are multiple mipmap stacks, each one is referred to as a slice of the texture. 1D, 2D, 2DMultisample, and 3D textures have a single slice. In 1DArray and 2DArray textures, every slice is an array element. A Cube texture always has 6 slices, one for each face. In a CubeArray texture, each set of six slices is one element in the array.
    ///
    /// Most APIs that operate on individual images in a texture address those images via a tuple of a Slice, and Mipmap Level within that slice.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtltexture?language=objc)
    #[name = "MTLTexture"]
    pub unsafe trait Texture: Resource {
        /// The resource this texture was created from. It may be a texture or a buffer. If this texture is not reusing storage of another MTLResource, then nil is returned.
        #[deprecated = "Use parentTexture or buffer instead"]
        #[unsafe(method(rootResource))]
        #[unsafe(method_family = none)]
        fn root_resource(&self) -> Option<Retained<ProtocolObject<dyn Resource>>>;

        /// The texture this texture view was created from, or nil if this is not a texture view or it was not created from a texture.
        #[unsafe(method(parentTexture))]
        #[unsafe(method_family = none)]
        fn parent_texture(&self) -> Option<Retained<ProtocolObject<dyn Texture>>>;

        /// The base level of the texture this texture view was created from, or 0 if this is not a texture view.
        #[unsafe(method(parentRelativeLevel))]
        #[unsafe(method_family = none)]
        fn parent_relative_level(&self) -> usize;

        /// The base slice of the texture this texture view was created from, or 0 if this is not a texture view.
        #[unsafe(method(parentRelativeSlice))]
        #[unsafe(method_family = none)]
        fn parent_relative_slice(&self) -> usize;

        /// The buffer this texture view was created from, or nil if this is not a texture view or it was not created from a buffer.
        #[unsafe(method(buffer))]
        #[unsafe(method_family = none)]
        fn buffer(&self) -> Option<Retained<ProtocolObject<dyn Buffer>>>;

        /// The offset of the buffer this texture view was created from, or 0 if this is not a texture view.
        #[unsafe(method(bufferOffset))]
        #[unsafe(method_family = none)]
        fn buffer_offset(&self) -> usize;

        /// The bytesPerRow of the buffer this texture view was created from, or 0 if this is not a texture view.
        #[unsafe(method(bufferBytesPerRow))]
        #[unsafe(method_family = none)]
        fn buffer_bytes_per_row(&self) -> usize;

        /// If this texture was created from an IOSurface, this returns a reference to that IOSurface. iosurface is nil if this texture was not created from an IOSurface.
        #[unsafe(method(iosurface))]
        #[unsafe(method_family = none)]
        unsafe fn iosurface(&self) -> Option<Retained<IOSurfaceRef>>;

        /// If this texture was created from an IOSurface, this returns the plane of the IOSurface from which the texture was created. iosurfacePlane is 0 if this texture was not created from an IOSurface.
        #[unsafe(method(iosurfacePlane))]
        #[unsafe(method_family = none)]
        fn iosurface_plane(&self) -> usize;

        /// The type of this texture.
        #[unsafe(method(textureType))]
        #[unsafe(method_family = none)]
        fn texture_type(&self) -> TextureType;

        /// The MTLPixelFormat that is used to interpret this texture's contents.
        #[unsafe(method(pixelFormat))]
        #[unsafe(method_family = none)]
        fn pixel_format(&self) -> PixelFormat;

        /// The width of the MTLTexture instance in pixels.
        #[unsafe(method(width))]
        #[unsafe(method_family = none)]
        fn width(&self) -> usize;

        /// The height of the MTLTexture instance in pixels.
        ///
        /// . height is 1 if the texture is 1D.
        #[unsafe(method(height))]
        #[unsafe(method_family = none)]
        fn height(&self) -> usize;

        /// The depth of this MTLTexture instance in pixels.
        ///
        /// If this MTLTexture is not a 3D texture, the depth is 1
        #[unsafe(method(depth))]
        #[unsafe(method_family = none)]
        fn depth(&self) -> usize;

        /// The number of mipmap levels in each slice of this MTLTexture.
        #[unsafe(method(mipmapLevelCount))]
        #[unsafe(method_family = none)]
        fn mipmap_level_count(&self) -> usize;

        /// The number of samples in each pixel of this MTLTexture.
        ///
        /// If this texture is any type other than 2DMultisample, samples is 1.
        #[unsafe(method(sampleCount))]
        #[unsafe(method_family = none)]
        fn sample_count(&self) -> usize;

        /// The number of array elements in this MTLTexture.
        ///
        /// For non-Array texture types, arrayLength is 1.
        #[unsafe(method(arrayLength))]
        #[unsafe(method_family = none)]
        fn array_length(&self) -> usize;

        /// Description of texture usage.
        #[unsafe(method(usage))]
        #[unsafe(method_family = none)]
        fn usage(&self) -> TextureUsage;

        /// If YES, this texture can be shared with other processes.
        ///
        /// Texture can be shared across process addres space boundaries through use of sharedTextureHandle and XPC.
        #[unsafe(method(isShareable))]
        #[unsafe(method_family = none)]
        fn is_shareable(&self) -> bool;

        /// If YES, this texture can only be used with a MTLAttachmentDescriptor, and cannot be used as a texture argument for MTLRenderCommandEncoder, MTLBlitCommandEncoder, or MTLComputeCommandEncoder. Furthermore, when this property's value is YES, readPixels/writePixels may not be used with this texture.
        ///
        /// Textures obtained from CAMetalDrawables may have this property set to YES, depending on the value of frameBufferOnly passed to their parent CAMetalLayer. Textures created directly by the application will not have any restrictions.
        #[unsafe(method(isFramebufferOnly))]
        #[unsafe(method_family = none)]
        fn is_framebuffer_only(&self) -> bool;

        /// For sparse textures this property returns index of first mipmap that is packed in tail.
        /// Mapping this mipmap level will map all subsequent mipmap levels.
        #[optional]
        #[unsafe(method(firstMipmapInTail))]
        #[unsafe(method_family = none)]
        fn first_mipmap_in_tail(&self) -> usize;

        /// Amount of memory in bytes required to map sparse texture tail.
        #[optional]
        #[unsafe(method(tailSizeInBytes))]
        #[unsafe(method_family = none)]
        fn tail_size_in_bytes(&self) -> usize;

        #[optional]
        #[unsafe(method(isSparse))]
        #[unsafe(method_family = none)]
        fn is_sparse(&self) -> bool;

        /// Allow GPU-optimization for the contents texture. The default value is true.
        ///
        /// Useful for opting-out of GPU-optimization when implicit optimization (e.g. RT writes) is regressing CPU-read-back performance. See the documentation for optimizeContentsForGPUAccess: and optimizeContentsForCPUAccess: APIs.
        #[unsafe(method(allowGPUOptimizedContents))]
        #[unsafe(method_family = none)]
        fn allow_gpu_optimized_contents(&self) -> bool;

        /// Returns the compression type of the texture
        ///
        /// See the compressionType property on MTLTextureDescriptor
        #[unsafe(method(compressionType))]
        #[unsafe(method_family = none)]
        unsafe fn compression_type(&self) -> TextureCompressionType;

        /// Handle of the GPU resource suitable for storing in an Argument Buffer
        #[unsafe(method(gpuResourceID))]
        #[unsafe(method_family = none)]
        unsafe fn gpu_resource_id(&self) -> ResourceID;

        /// Copies a block of pixels from a texture slice into the application's memory.
        ///
        /// # Safety
        ///
        /// `pixel_bytes` must be a valid pointer.
        #[unsafe(method(getBytes:bytesPerRow:bytesPerImage:fromRegion:mipmapLevel:slice:))]
        #[unsafe(method_family = none)]
        unsafe fn get_bytes_bytes_per_row_bytes_per_image_from_region_mipmap_level_slice(
            &self,
            pixel_bytes: NonNull<c_void>,
            bytes_per_row: usize,
            bytes_per_image: usize,
            region: Region,
            level: usize,
            slice: usize,
        );

        /// Copy a block of pixel data from the caller's pointer into a texture slice.
        ///
        /// # Safety
        ///
        /// `pixel_bytes` must be a valid pointer.
        #[unsafe(method(replaceRegion:mipmapLevel:slice:withBytes:bytesPerRow:bytesPerImage:))]
        #[unsafe(method_family = none)]
        unsafe fn replace_region_mipmap_level_slice_withBytes_bytesPerRow_bytesPerImage(
            &self,
            region: Region,
            level: usize,
            slice: usize,
            pixel_bytes: NonNull<c_void>,
            bytes_per_row: usize,
            bytes_per_image: usize,
        );

        /// Convenience for getBytes:bytesPerRow:bytesPerImage:fromRegion:mipmapLevel:slice: that doesn't require slice related arguments
        ///
        /// # Safety
        ///
        /// `pixel_bytes` must be a valid pointer.
        #[unsafe(method(getBytes:bytesPerRow:fromRegion:mipmapLevel:))]
        #[unsafe(method_family = none)]
        unsafe fn get_bytes_bytes_per_row_from_region_mipmap_level(
            &self,
            pixel_bytes: NonNull<c_void>,
            bytes_per_row: usize,
            region: Region,
            level: usize,
        );

        /// Convenience for replaceRegion:mipmapLevel:slice:withBytes:bytesPerRow:bytesPerImage: that doesn't require slice related arguments
        ///
        /// # Safety
        ///
        /// `pixel_bytes` must be a valid pointer.
        #[unsafe(method(replaceRegion:mipmapLevel:withBytes:bytesPerRow:))]
        #[unsafe(method_family = none)]
        unsafe fn replace_region_mipmap_level_with_bytes_bytes_per_row(
            &self,
            region: Region,
            level: usize,
            pixel_bytes: NonNull<c_void>,
            bytes_per_row: usize,
        );

        /// Create a new texture which shares the same storage as the source texture, but with a different (but compatible) pixel format.
        #[unsafe(method(newTextureViewWithPixelFormat:))]
        #[unsafe(method_family = new)]
        fn new_texture_view_with_pixel_format(
            &self,
            pixel_format: PixelFormat,
        ) -> Option<Retained<ProtocolObject<dyn Texture>>>;

        /// Create a new texture which shares the same storage as the source texture, but with a different (but compatible) pixel format, texture type, levels and slices.
        #[unsafe(method(newTextureViewWithPixelFormat:textureType:levels:slices:))]
        #[unsafe(method_family = new)]
        unsafe fn new_texture_view_with_pixel_format_texture_type_levels_slices(
            &self,
            pixel_format: PixelFormat,
            texture_type: TextureType,
            level_range: NSRange,
            slice_range: NSRange,
        ) -> Option<Retained<ProtocolObject<dyn Texture>>>;

        /// Create a new texture handle, that can be shared across process addres space boundaries.
        #[unsafe(method(newSharedTextureHandle))]
        #[unsafe(method_family = new)]
        fn new_shared_texture_handle(&self) -> Option<Retained<SharedTextureHandle>>;

        /// For Metal texture objects that are remote views, this returns the texture associated with the storage on the originating device.
        #[unsafe(method(remoteStorageTexture))]
        #[unsafe(method_family = none)]
        fn remote_storage_texture(&self) -> Option<Retained<ProtocolObject<dyn Texture>>>;

        /// On Metal devices that support peer to peer transfers, this method is used to create a remote texture view on another device
        /// within the peer group.  The receiver must use MTLStorageModePrivate or be backed by an IOSurface.
        #[unsafe(method(newRemoteTextureViewForDevice:))]
        #[unsafe(method_family = new)]
        unsafe fn new_remote_texture_view_for_device(
            &self,
            device: &ProtocolObject<dyn Device>,
        ) -> Option<Retained<ProtocolObject<dyn Texture>>>;

        /// The channel swizzle used when reading or sampling from this texture
        #[unsafe(method(swizzle))]
        #[unsafe(method_family = none)]
        fn swizzle(&self) -> TextureSwizzleChannels;

        /// Create a new texture which shares the same storage as the source texture, but with a different (but compatible) pixel format, texture type, levels, slices and swizzle.
        #[unsafe(method(newTextureViewWithPixelFormat:textureType:levels:slices:swizzle:))]
        #[unsafe(method_family = new)]
        unsafe fn new_texture_view_with_pixel_format_texture_type_levels_slices_swizzle(
            &self,
            pixel_format: PixelFormat,
            texture_type: TextureType,
            level_range: NSRange,
            slice_range: NSRange,
            swizzle: TextureSwizzleChannels,
        ) -> Option<Retained<ProtocolObject<dyn Texture>>>;
    }
);
