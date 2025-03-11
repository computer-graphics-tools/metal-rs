//! MTLRenderPassDescriptor implementation
//! 
//! This file contains additional implementations for the render pass descriptor types.

use super::*;

impl fmt::Debug for MTLRenderPassColorAttachmentDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ds = f.debug_struct("MTLRenderPassColorAttachmentDescriptor");
        ds.field("load_action", &self.load_action());
        ds.field("store_action", &self.store_action());
        ds.field("clear_color", &self.clear_color());
        if let Some(_) = self.texture() {
            ds.field("has_texture", &true);
        } else {
            ds.field("has_texture", &false);
        }
        ds.finish()
    }
}

impl fmt::Debug for MTLRenderPassDepthAttachmentDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ds = f.debug_struct("MTLRenderPassDepthAttachmentDescriptor");
        ds.field("load_action", &self.load_action());
        ds.field("store_action", &self.store_action());
        ds.field("clear_depth", &self.clear_depth());
        if let Some(_) = self.texture() {
            ds.field("has_texture", &true);
        } else {
            ds.field("has_texture", &false);
        }
        ds.finish()
    }
}

impl fmt::Debug for MTLRenderPassStencilAttachmentDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ds = f.debug_struct("MTLRenderPassStencilAttachmentDescriptor");
        ds.field("load_action", &self.load_action());
        ds.field("store_action", &self.store_action());
        ds.field("clear_stencil", &self.clear_stencil());
        if let Some(_) = self.texture() {
            ds.field("has_texture", &true);
        } else {
            ds.field("has_texture", &false);
        }
        ds.finish()
    }
}

impl Drop for MTLRenderPassColorAttachmentDescriptor {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLRenderPassColorAttachmentDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, copy];
            MTLRenderPassColorAttachmentDescriptor::from_ptr(obj)
        }
    }
}

impl Drop for MTLRenderPassDepthAttachmentDescriptor {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLRenderPassDepthAttachmentDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, copy];
            MTLRenderPassDepthAttachmentDescriptor::from_ptr(obj)
        }
    }
}

impl Drop for MTLRenderPassStencilAttachmentDescriptor {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLRenderPassStencilAttachmentDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, copy];
            MTLRenderPassStencilAttachmentDescriptor::from_ptr(obj)
        }
    }
}

impl Drop for MTLRenderPassColorAttachmentDescriptorArray {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLRenderPassColorAttachmentDescriptorArray {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLRenderPassColorAttachmentDescriptorArray::from_ptr(obj)
        }
    }
}