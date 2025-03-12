// Copyright 2024 The metal-rs contributors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use objc::{msg_send, sel, sel_impl, class};
use objc::runtime::Object;
use foreign_types::{ForeignType, ForeignTypeRef};
use crate::foundation::{NSUInteger, NSString, NSNumber};
use crate::metal::buffer::MTLBufferRef;
use crate::metal::texture::MTLSize;
use crate::metal::types::{MTLSizeAndAlign, MTLCoordinate2D};
use std::fmt;

/// A reference to an Objective-C `MTLRasterizationRateSampleArray`.
pub struct MTLRasterizationRateSampleArrayRef(Object);

/// An owned Objective-C `MTLRasterizationRateSampleArray`.
pub struct MTLRasterizationRateSampleArray(*mut Object);

unsafe impl ForeignTypeRef for MTLRasterizationRateSampleArrayRef {
    type CType = Object;
}

unsafe impl Send for MTLRasterizationRateSampleArrayRef {}
unsafe impl Sync for MTLRasterizationRateSampleArrayRef {}

unsafe impl ForeignType for MTLRasterizationRateSampleArray {
    type CType = Object;
    type Ref = MTLRasterizationRateSampleArrayRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLRasterizationRateSampleArray {
        MTLRasterizationRateSampleArray(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLRasterizationRateSampleArrayRef> for MTLRasterizationRateSampleArray {
    fn as_ref(&self) -> &MTLRasterizationRateSampleArrayRef {
        unsafe { &*(self.0.cast::<MTLRasterizationRateSampleArrayRef>()) }
    }
}

unsafe impl Send for MTLRasterizationRateSampleArray {}
unsafe impl Sync for MTLRasterizationRateSampleArray {}

unsafe impl objc::Message for MTLRasterizationRateSampleArrayRef {}

impl MTLRasterizationRateSampleArray {
    /// Creates a new rasterization rate sample array.
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLRasterizationRateSampleArray);
            let obj: *mut Object = msg_send![class, alloc];
            let obj: *mut Object = msg_send![obj, init];
            MTLRasterizationRateSampleArray::from_ptr(obj)
        }
    }
}

impl Default for MTLRasterizationRateSampleArray {
    fn default() -> Self {
        Self::new()
    }
}

impl MTLRasterizationRateSampleArrayRef {
    /// Gets the sample at the specified index.
    #[must_use]
    pub fn object(&self, index: NSUInteger) -> NSNumber {
        unsafe {
            let obj: *mut Object = msg_send![self, objectAtIndexedSubscript: index];
            NSNumber::from_ptr(obj)
        }
    }

    /// Sets the sample at the specified index.
    pub fn set_object(&self, value: &NSNumber, index: NSUInteger) {
        unsafe {
            let _: () = msg_send![self, setObject:value.as_ptr() atIndexedSubscript:index];
        }
    }
}

impl Clone for MTLRasterizationRateSampleArray {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLRasterizationRateSampleArray::from_ptr(obj)
        }
    }
}

impl Drop for MTLRasterizationRateSampleArray {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

/// A reference to an Objective-C `MTLRasterizationRateLayerDescriptor`.
pub struct MTLRasterizationRateLayerDescriptorRef(Object);

/// An owned Objective-C `MTLRasterizationRateLayerDescriptor`.
pub struct MTLRasterizationRateLayerDescriptor(*mut Object);

unsafe impl ForeignTypeRef for MTLRasterizationRateLayerDescriptorRef {
    type CType = Object;
}

unsafe impl Send for MTLRasterizationRateLayerDescriptorRef {}
unsafe impl Sync for MTLRasterizationRateLayerDescriptorRef {}

unsafe impl ForeignType for MTLRasterizationRateLayerDescriptor {
    type CType = Object;
    type Ref = MTLRasterizationRateLayerDescriptorRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLRasterizationRateLayerDescriptor {
        MTLRasterizationRateLayerDescriptor(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLRasterizationRateLayerDescriptorRef> for MTLRasterizationRateLayerDescriptor {
    fn as_ref(&self) -> &MTLRasterizationRateLayerDescriptorRef {
        unsafe { &*(self.0.cast::<MTLRasterizationRateLayerDescriptorRef>()) }
    }
}

unsafe impl Send for MTLRasterizationRateLayerDescriptor {}
unsafe impl Sync for MTLRasterizationRateLayerDescriptor {}

unsafe impl objc::Message for MTLRasterizationRateLayerDescriptorRef {}

impl MTLRasterizationRateLayerDescriptor {
    /// Creates a new rasterization rate layer descriptor.
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLRasterizationRateLayerDescriptor);
            let obj: *mut Object = msg_send![class, alloc];
            let obj: *mut Object = msg_send![obj, init];
            MTLRasterizationRateLayerDescriptor::from_ptr(obj)
        }
    }

    /// Creates a new rasterization rate layer descriptor with sample count.
    pub fn with_sample_count(sample_count: MTLSize) -> Self {
        unsafe {
            let class = class!(MTLRasterizationRateLayerDescriptor);
            let obj: *mut Object = msg_send![class, alloc];
            let obj: *mut Object = msg_send![obj, initWithSampleCount:sample_count];
            MTLRasterizationRateLayerDescriptor::from_ptr(obj)
        }
    }

    /// Creates a new rasterization rate layer descriptor with sample count and rate data.
    ///
    /// # Safety
    ///
    /// The `horizontal` and `vertical` arrays must be valid pointers to arrays of floats,
    /// with length matching or exceeding the width and height of `sample_count`, respectively.
    pub unsafe fn with_sample_count_and_rates(
        sample_count: MTLSize,
        horizontal: *const f32,
        vertical: *const f32,
    ) -> Self {
        let class = class!(MTLRasterizationRateLayerDescriptor);
        let obj: *mut Object = msg_send![class, alloc];
        let obj: *mut Object = msg_send![obj, initWithSampleCount:sample_count horizontal:horizontal vertical:vertical];
        unsafe { MTLRasterizationRateLayerDescriptor::from_ptr(obj) }
    }
}

impl Default for MTLRasterizationRateLayerDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl MTLRasterizationRateLayerDescriptorRef {
    /// Gets the sample count.
    #[must_use]
    pub fn sample_count(&self) -> MTLSize {
        unsafe { msg_send![self, sampleCount] }
    }

    /// Gets the maximum sample count.
    #[must_use]
    pub fn max_sample_count(&self) -> MTLSize {
        unsafe { msg_send![self, maxSampleCount] }
    }

    /// Gets a pointer to the horizontal sample storage.
    ///
    /// # Safety
    ///
    /// This returns a raw pointer to the internal storage, which must not be used
    /// after the descriptor is deallocated.
    #[must_use]
    pub unsafe fn horizontal_sample_storage(&self) -> *mut f32 {
        msg_send![self, horizontalSampleStorage]
    }

    /// Gets a pointer to the vertical sample storage.
    ///
    /// # Safety
    ///
    /// This returns a raw pointer to the internal storage, which must not be used
    /// after the descriptor is deallocated.
    #[must_use]
    pub unsafe fn vertical_sample_storage(&self) -> *mut f32 {
        msg_send![self, verticalSampleStorage]
    }

    /// Gets the horizontal sample array.
    #[must_use]
    pub fn horizontal(&self) -> MTLRasterizationRateSampleArray {
        unsafe {
            let obj: *mut Object = msg_send![self, horizontal];
            MTLRasterizationRateSampleArray::from_ptr(obj)
        }
    }

    /// Gets the vertical sample array.
    #[must_use]
    pub fn vertical(&self) -> MTLRasterizationRateSampleArray {
        unsafe {
            let obj: *mut Object = msg_send![self, vertical];
            MTLRasterizationRateSampleArray::from_ptr(obj)
        }
    }

    /// Sets the sample count.
    pub fn set_sample_count(&self, sample_count: MTLSize) {
        unsafe { msg_send![self, setSampleCount: sample_count] }
    }
}

impl Clone for MTLRasterizationRateLayerDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLRasterizationRateLayerDescriptor::from_ptr(obj)
        }
    }
}

impl Drop for MTLRasterizationRateLayerDescriptor {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl fmt::Debug for MTLRasterizationRateLayerDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MTLRasterizationRateLayerDescriptor")
            .field("sample_count", &self.as_ref().sample_count())
            .field("max_sample_count", &self.as_ref().max_sample_count())
            .finish()
    }
}

/// A reference to an Objective-C `MTLRasterizationRateLayerArray`.
pub struct MTLRasterizationRateLayerArrayRef(Object);

/// An owned Objective-C `MTLRasterizationRateLayerArray`.
pub struct MTLRasterizationRateLayerArray(*mut Object);

unsafe impl ForeignTypeRef for MTLRasterizationRateLayerArrayRef {
    type CType = Object;
}

unsafe impl Send for MTLRasterizationRateLayerArrayRef {}
unsafe impl Sync for MTLRasterizationRateLayerArrayRef {}

unsafe impl ForeignType for MTLRasterizationRateLayerArray {
    type CType = Object;
    type Ref = MTLRasterizationRateLayerArrayRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLRasterizationRateLayerArray {
        MTLRasterizationRateLayerArray(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLRasterizationRateLayerArrayRef> for MTLRasterizationRateLayerArray {
    fn as_ref(&self) -> &MTLRasterizationRateLayerArrayRef {
        unsafe { &*(self.0.cast::<MTLRasterizationRateLayerArrayRef>()) }
    }
}

unsafe impl Send for MTLRasterizationRateLayerArray {}
unsafe impl Sync for MTLRasterizationRateLayerArray {}

unsafe impl objc::Message for MTLRasterizationRateLayerArrayRef {}

impl MTLRasterizationRateLayerArray {
    /// Creates a new rasterization rate layer array.
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLRasterizationRateLayerArray);
            let obj: *mut Object = msg_send![class, alloc];
            let obj: *mut Object = msg_send![obj, init];
            MTLRasterizationRateLayerArray::from_ptr(obj)
        }
    }
}

impl Default for MTLRasterizationRateLayerArray {
    fn default() -> Self {
        Self::new()
    }
}

impl MTLRasterizationRateLayerArrayRef {
    /// Gets the layer descriptor at the specified index.
    #[must_use]
    pub fn object(&self, layer_index: NSUInteger) -> Option<MTLRasterizationRateLayerDescriptor> {
        unsafe {
            let obj: *mut Object = msg_send![self, objectAtIndexedSubscript: layer_index];
            if obj.is_null() {
                None
            } else {
                Some(MTLRasterizationRateLayerDescriptor::from_ptr(obj))
            }
        }
    }

    /// Sets the layer descriptor at the specified index.
    pub fn set_object(&self, layer: Option<&MTLRasterizationRateLayerDescriptor>, layer_index: NSUInteger) {
        unsafe {
            let ptr = match layer {
                Some(layer) => layer.as_ptr(),
                None => std::ptr::null_mut(),
            };
            let _: () = msg_send![self, setObject:ptr atIndexedSubscript:layer_index];
        }
    }
}

impl Clone for MTLRasterizationRateLayerArray {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLRasterizationRateLayerArray::from_ptr(obj)
        }
    }
}

impl Drop for MTLRasterizationRateLayerArray {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

/// A reference to an Objective-C `MTLRasterizationRateMapDescriptor`.
pub struct MTLRasterizationRateMapDescriptorRef(Object);

/// An owned Objective-C `MTLRasterizationRateMapDescriptor`.
pub struct MTLRasterizationRateMapDescriptor(*mut Object);

unsafe impl ForeignTypeRef for MTLRasterizationRateMapDescriptorRef {
    type CType = Object;
}

unsafe impl Send for MTLRasterizationRateMapDescriptorRef {}
unsafe impl Sync for MTLRasterizationRateMapDescriptorRef {}

unsafe impl ForeignType for MTLRasterizationRateMapDescriptor {
    type CType = Object;
    type Ref = MTLRasterizationRateMapDescriptorRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLRasterizationRateMapDescriptor {
        MTLRasterizationRateMapDescriptor(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLRasterizationRateMapDescriptorRef> for MTLRasterizationRateMapDescriptor {
    fn as_ref(&self) -> &MTLRasterizationRateMapDescriptorRef {
        unsafe { &*(self.0.cast::<MTLRasterizationRateMapDescriptorRef>()) }
    }
}

unsafe impl Send for MTLRasterizationRateMapDescriptor {}
unsafe impl Sync for MTLRasterizationRateMapDescriptor {}

unsafe impl objc::Message for MTLRasterizationRateMapDescriptorRef {}

impl MTLRasterizationRateMapDescriptor {
    /// Creates a new rasterization rate map descriptor.
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLRasterizationRateMapDescriptor);
            let obj: *mut Object = msg_send![class, alloc];
            let obj: *mut Object = msg_send![obj, init];
            MTLRasterizationRateMapDescriptor::from_ptr(obj)
        }
    }

    /// Creates a new rasterization rate map descriptor with screen size.
    pub fn with_screen_size(screen_size: MTLSize) -> Self {
        unsafe {
            let class = class!(MTLRasterizationRateMapDescriptor);
            let obj: *mut Object = msg_send![class, rasterizationRateMapDescriptorWithScreenSize:screen_size];
            MTLRasterizationRateMapDescriptor::from_ptr(obj)
        }
    }

    /// Creates a new rasterization rate map descriptor with screen size and layer.
    pub fn with_screen_size_and_layer(
        screen_size: MTLSize,
        layer: &MTLRasterizationRateLayerDescriptor,
    ) -> Self {
        unsafe {
            let class = class!(MTLRasterizationRateMapDescriptor);
            let obj: *mut Object = msg_send![class, rasterizationRateMapDescriptorWithScreenSize:screen_size
                                                                                         layer:layer.as_ptr()];
            MTLRasterizationRateMapDescriptor::from_ptr(obj)
        }
    }

    /// Creates a new rasterization rate map descriptor with screen size and multiple layers.
    ///
    /// # Safety
    ///
    /// The `layers` array must be a valid pointer to an array of `MTLRasterizationRateLayerDescriptor` pointers,
    /// with length at least equal to `layer_count`.
    pub unsafe fn with_screen_size_and_layers(
        screen_size: MTLSize,
        layer_count: NSUInteger,
        layers: *const *const MTLRasterizationRateLayerDescriptorRef,
    ) -> Self {
        let class = class!(MTLRasterizationRateMapDescriptor);
        let obj: *mut Object = msg_send![class, rasterizationRateMapDescriptorWithScreenSize:screen_size
                                                                              layerCount:layer_count
                                                                                  layers:layers];
        unsafe { MTLRasterizationRateMapDescriptor::from_ptr(obj) }
    }
}

impl Default for MTLRasterizationRateMapDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl MTLRasterizationRateMapDescriptorRef {
    /// Gets the layer descriptor at the specified index.
    #[must_use]
    pub fn layer(&self, layer_index: NSUInteger) -> Option<MTLRasterizationRateLayerDescriptor> {
        unsafe {
            let obj: *mut Object = msg_send![self, layerAtIndex: layer_index];
            if obj.is_null() {
                None
            } else {
                Some(MTLRasterizationRateLayerDescriptor::from_ptr(obj))
            }
        }
    }

    /// Sets the layer descriptor at the specified index.
    pub fn set_layer(&self, layer: Option<&MTLRasterizationRateLayerDescriptor>, layer_index: NSUInteger) {
        unsafe {
            let ptr = match layer {
                Some(layer) => layer.as_ptr(),
                None => std::ptr::null_mut(),
            };
            let _: () = msg_send![self, setLayer:ptr atIndex:layer_index];
        }
    }

    /// Gets all layer descriptors.
    #[must_use]
    pub fn layers(&self) -> MTLRasterizationRateLayerArray {
        unsafe {
            let obj: *mut Object = msg_send![self, layers];
            MTLRasterizationRateLayerArray::from_ptr(obj)
        }
    }

    /// Gets the screen size.
    #[must_use]
    pub fn screen_size(&self) -> MTLSize {
        unsafe { msg_send![self, screenSize] }
    }

    /// Sets the screen size.
    pub fn set_screen_size(&self, screen_size: MTLSize) {
        unsafe { msg_send![self, setScreenSize: screen_size] }
    }

    /// Gets the label.
    #[must_use]
    pub fn label(&self) -> Option<String> {
        unsafe {
            let label: *mut Object = msg_send![self, label];
            if label.is_null() {
                None
            } else {
                let ns_string = NSString::from_ptr(label);
                Some(ns_string.to_rust_string())
            }
        }
    }

    /// Sets the label.
    pub fn set_label(&self, label: &str) {
        unsafe {
            let ns_string = NSString::from_rust_str(label);
            let _: () = msg_send![self, setLabel: ns_string.as_ptr()];
        }
    }

    /// Gets the layer count.
    #[must_use]
    pub fn layer_count(&self) -> NSUInteger {
        unsafe { msg_send![self, layerCount] }
    }
}

impl Clone for MTLRasterizationRateMapDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLRasterizationRateMapDescriptor::from_ptr(obj)
        }
    }
}

impl Drop for MTLRasterizationRateMapDescriptor {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl fmt::Debug for MTLRasterizationRateMapDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MTLRasterizationRateMapDescriptor")
            .field("screen_size", &self.as_ref().screen_size())
            .field("layer_count", &self.as_ref().layer_count())
            .field("label", &self.as_ref().label())
            .finish()
    }
}

/// A reference to an Objective-C `MTLRasterizationRateMap`.
pub struct MTLRasterizationRateMapRef(Object);

/// An owned Objective-C `MTLRasterizationRateMap`.
pub struct MTLRasterizationRateMap(*mut Object);

unsafe impl ForeignTypeRef for MTLRasterizationRateMapRef {
    type CType = Object;
}

unsafe impl Send for MTLRasterizationRateMapRef {}
unsafe impl Sync for MTLRasterizationRateMapRef {}

unsafe impl ForeignType for MTLRasterizationRateMap {
    type CType = Object;
    type Ref = MTLRasterizationRateMapRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLRasterizationRateMap {
        MTLRasterizationRateMap(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLRasterizationRateMapRef> for MTLRasterizationRateMap {
    fn as_ref(&self) -> &MTLRasterizationRateMapRef {
        unsafe { &*(self.0.cast::<MTLRasterizationRateMapRef>()) }
    }
}

unsafe impl Send for MTLRasterizationRateMap {}
unsafe impl Sync for MTLRasterizationRateMap {}

unsafe impl objc::Message for MTLRasterizationRateMapRef {}

impl MTLRasterizationRateMapRef {
    /// Gets the device that created this rasterization rate map.
    #[must_use]
    pub fn device(&self) -> crate::metal::MTLDevice {
        unsafe {
            let ptr: *mut Object = msg_send![self, device];
            crate::metal::MTLDevice::from_ptr(ptr)
        }
    }

    /// Gets the label.
    #[must_use]
    pub fn label(&self) -> Option<String> {
        unsafe {
            let label: *mut Object = msg_send![self, label];
            if label.is_null() {
                None
            } else {
                let ns_string = NSString::from_ptr(label);
                Some(ns_string.to_rust_string())
            }
        }
    }

    /// Gets the screen size.
    #[must_use]
    pub fn screen_size(&self) -> MTLSize {
        unsafe { msg_send![self, screenSize] }
    }

    /// Gets the physical granularity.
    #[must_use]
    pub fn physical_granularity(&self) -> MTLSize {
        unsafe { msg_send![self, physicalGranularity] }
    }

    /// Gets the layer count.
    #[must_use]
    pub fn layer_count(&self) -> NSUInteger {
        unsafe { msg_send![self, layerCount] }
    }

    /// Gets the parameter buffer size and alignment.
    #[must_use]
    pub fn parameter_buffer_size_and_align(&self) -> MTLSizeAndAlign {
        unsafe { msg_send![self, parameterBufferSizeAndAlign] }
    }

    /// Copies parameter data to a buffer.
    pub fn copy_parameter_data_to_buffer(&self, buffer: &impl AsRef<MTLBufferRef>, offset: NSUInteger) {
        unsafe {
            let _: () = msg_send![self, copyParameterDataToBuffer:buffer.as_ref().as_ptr() offset:offset];
        }
    }

    /// Gets the physical size for a layer.
    #[must_use]
    pub fn physical_size(&self, layer_index: NSUInteger) -> MTLSize {
        unsafe { msg_send![self, physicalSizeForLayer: layer_index] }
    }

    /// Maps screen coordinates to physical coordinates.
    #[must_use]
    pub fn map_screen_to_physical_coordinates(
        &self,
        screen_coordinates: MTLCoordinate2D,
        layer_index: NSUInteger,
    ) -> MTLCoordinate2D {
        unsafe {
            msg_send![self, mapScreenToPhysicalCoordinates:screen_coordinates forLayer:layer_index]
        }
    }

    /// Maps physical coordinates to screen coordinates.
    #[must_use]
    pub fn map_physical_to_screen_coordinates(
        &self,
        physical_coordinates: MTLCoordinate2D,
        layer_index: NSUInteger,
    ) -> MTLCoordinate2D {
        unsafe {
            msg_send![self, mapPhysicalToScreenCoordinates:physical_coordinates forLayer:layer_index]
        }
    }
}

impl Clone for MTLRasterizationRateMap {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLRasterizationRateMap::from_ptr(obj)
        }
    }
}

impl Drop for MTLRasterizationRateMap {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl fmt::Debug for MTLRasterizationRateMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MTLRasterizationRateMap")
            .field("screen_size", &self.as_ref().screen_size())
            .field("physical_granularity", &self.as_ref().physical_granularity())
            .field("layer_count", &self.as_ref().layer_count())
            .field("label", &self.as_ref().label())
            .finish()
    }
}