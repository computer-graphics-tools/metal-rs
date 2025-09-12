//
//  MTLTypes.h
//  Metal
//
//  Copyright (c) 2014 Apple Inc. All rights reserved.
//

#import <Foundation/Foundation.h>
#import <Metal/MTLDefines.h>

MTL_INLINE MTLOrigin MTLOriginMake(NSUInteger x, NSUInteger y, NSUInteger z)
{
    MTLOrigin origin = {x, y, z};
    return origin;
}

MTL_INLINE MTLSize MTLSizeMake(NSUInteger width, NSUInteger height, NSUInteger depth)
{
    MTLSize size = {width, height, depth};
    return size;
}

MTL_INLINE MTLRegion MTLRegionMake1D(NSUInteger x, NSUInteger width)
{
    MTLRegion region;
    region.origin.x = x; region.origin.y = 0; region.origin.z = 0;
    region.size.width = width; region.size.height = 1; region.size.depth = 1;
    return region;
}

MTL_INLINE MTLRegion MTLRegionMake2D(NSUInteger x, NSUInteger y, NSUInteger width, NSUInteger height)
{
    MTLRegion region;
    region.origin.x = x; region.origin.y = y; region.origin.z = 0;
    region.size.width = width; region.size.height = height; region.size.depth = 1;
    return region;
}

MTL_INLINE MTLRegion MTLRegionMake3D(NSUInteger x, NSUInteger y, NSUInteger z, NSUInteger width, NSUInteger height, NSUInteger depth)
{
    MTLRegion region;
    region.origin.x = x; region.origin.y = y; region.origin.z = z;
    region.size.width = width; region.size.height = height; region.size.depth = depth;
    return region;
}

/*!
 @struct MTLSamplePosition
 @abstract Identify a sample within a pixel. Origin is top-left with a range [0,1) for both x and y.
 */

 

MTL_INLINE MTLSamplePosition MTLSamplePositionMake(float x, float y) API_AVAILABLE(macos(10.13), ios(11.0))
{
    MTLSamplePosition position = {x, y};
    return position;
}

/*!
 @typedef MTLCoordinate2D
 @abstract A floating point coordinate in an abstract 2D space.
 Refer to location of use for concrete information on the space in which the coordinate exists.
 */
 

/*!
 @function MTLCoordinate2DMake
 @abstract Convenience function to create a 2D coordinate from 2 values.
 */
MTL_INLINE MTLCoordinate2D MTLCoordinate2DMake(float x, float y)
{
    MTLCoordinate2D result = {x, y};
    return result;
} 

/*!
 @typedef MTLResourceID
 @abstract Handle of the GPU resource used for binding resources to argument tables, navigating resource view pools and storing resources in an argument buffer
 @discussion
 MTLResourceID represents a specific GPU resource. This handle can be mutated by modifying textureID or samplerID values to get to individual resource views in a resource view pool.
 */
 
