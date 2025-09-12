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

@protocol MTLDevice;

@protocol MTLHeap;

/*!
 @protocol MTLResource
 @abstract Common APIs available for MTLBuffer and MTLTexture instances
 */
API_AVAILABLE(macos(10.11), ios(8.0))
@protocol MTLResource <MTLAllocation>

/*!
 @property heap
 @abstract The heap from which this resouce was created.
 @discussion Nil when this resource is not backed by a heap.
 */
@property (readonly, nullable) id <MTLHeap> heap API_AVAILABLE(macos(10.13), ios(10.0));

@end


NS_ASSUME_NONNULL_END
