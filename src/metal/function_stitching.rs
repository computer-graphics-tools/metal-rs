//! MTLFunctionStitching - A Rust wrapper around Metal's function stitching API.
//!
//! This module provides Rust bindings to the MTLFunctionStitching classes from Apple's Metal framework.
//! Function stitching allows dynamic shader composition, where shader functions can be combined at runtime.
//!
//! # Key Types
//! 
//! * `MTLStitchedLibraryOptions` - Options for configuring stitched library behavior
//! * `MTLFunctionStitchingAttribute` - Base type for function stitching attributes
//! * `MTLFunctionStitchingAttributeAlwaysInline` - Attribute to force inlining of a function
//! * `MTLFunctionStitchingNode` - Base type for nodes in a function stitching graph
//! * `MTLFunctionStitchingInputNode` - Input node in a function stitching graph
//! * `MTLFunctionStitchingFunctionNode` - Function node in a function stitching graph
//! * `MTLFunctionStitchingGraph` - Graph describing how functions are stitched together
//! * `MTLStitchedLibraryDescriptor` - Descriptor for creating a stitched library
//!
//! # Examples
//!
//! ```no_run
//! # use metal_rs::metal::{MTLDevice, MTLLibrary};
//! # use metal_rs::metal::{MTLFunctionStitchingGraph, MTLFunctionStitchingFunctionNode, MTLFunctionStitchingInputNode};
//! # use metal_rs::metal::MTLStitchedLibraryDescriptor;
//! # use metal_rs::foundation::{NSArray, NSString};
//! # 
//! # let device = MTLDevice::system_default().unwrap();
//! # let library = device.new_library_with_data(&[]).unwrap();
//! 
//! // Create input nodes
//! let input_node_a = MTLFunctionStitchingInputNode::new_with_argument_index(0);
//! let input_node_b = MTLFunctionStitchingInputNode::new_with_argument_index(1);
//! 
//! // Create function node referencing a function in the library
//! let func_name = NSString::from_rust_str("add");
//! let args_array = NSArray::from_refs_slice(&[&input_node_a, &input_node_b]);
//! let func_node = MTLFunctionStitchingFunctionNode::new_with_name_arguments_control_dependencies(
//!     &func_name, 
//!     &args_array, 
//!     &NSArray::new()
//! );
//! 
//! // Create stitching graph
//! let output_name = NSString::from_rust_str("addFunction");
//! let nodes_array = NSArray::from_refs_slice(&[&input_node_a, &input_node_b, &func_node]);
//! let graph = MTLFunctionStitchingGraph::new_with_function_name_nodes_output_node_attributes(
//!     &output_name,
//!     &nodes_array,
//!     &func_node,
//!     &NSArray::new()
//! );
//! 
//! // Create stitched library descriptor
//! let graphs_array = NSArray::from_refs_slice(&[&graph]);
//! let descriptor = MTLStitchedLibraryDescriptor::new();
//! descriptor.set_function_graphs(&graphs_array);
//! 
//! // Create stitched library
//! if device.supports_stitched_libraries() {
//!     let stitched_library = device.new_stitched_library_with_descriptor(&descriptor).unwrap();
//!     let function = stitched_library.new_function_with_name("addFunction").unwrap();
//! }
//! ```

use std::fmt;
use objc::{msg_send, sel, sel_impl, class};
use objc::runtime::Object;
use foreign_types::{ForeignType, ForeignTypeRef};
use bitflags::bitflags;

use crate::foundation::{NSArray, NSString, NSUInteger};

/// Options for configuring stitched library behavior.
#[repr(u64)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MTLStitchedLibraryOptions {
    /// No special options.
    None = 0,
    /// Fail if a binary archive is missing.
    FailOnBinaryArchiveMiss = 1,
    /// Store the library in a MetalScript.
    StoreLibraryInMetalScript = 2,
}

bitflags! {
    /// A set of options for configuring stitched library behavior.
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct MTLStitchedLibraryOptionFlags: NSUInteger {
        /// No special options.
        const NONE = 0;
        /// Fail if a binary archive is missing.
        const FAIL_ON_BINARY_ARCHIVE_MISS = 1;
        /// Store the library in a MetalScript.
        const STORE_LIBRARY_IN_METAL_SCRIPT = 2;
    }
}

//
// MTLFunctionStitchingAttribute
//

/// A reference to an Objective-C `MTLFunctionStitchingAttribute`.
pub struct MTLFunctionStitchingAttributeRef(Object);

/// An owned Objective-C `MTLFunctionStitchingAttribute`.
pub struct MTLFunctionStitchingAttribute(*mut Object);

unsafe impl ForeignTypeRef for MTLFunctionStitchingAttributeRef {
    type CType = Object;
}

unsafe impl Send for MTLFunctionStitchingAttributeRef {}
unsafe impl Sync for MTLFunctionStitchingAttributeRef {}

unsafe impl ForeignType for MTLFunctionStitchingAttribute {
    type CType = Object;
    type Ref = MTLFunctionStitchingAttributeRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLFunctionStitchingAttribute {
        MTLFunctionStitchingAttribute(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLFunctionStitchingAttributeRef> for MTLFunctionStitchingAttribute {
    fn as_ref(&self) -> &MTLFunctionStitchingAttributeRef {
        unsafe { &*(self.0.cast::<MTLFunctionStitchingAttributeRef>()) }
    }
}

unsafe impl Send for MTLFunctionStitchingAttribute {}
unsafe impl Sync for MTLFunctionStitchingAttribute {}

unsafe impl objc::Message for MTLFunctionStitchingAttributeRef {}

impl fmt::Debug for MTLFunctionStitchingAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLFunctionStitchingAttribute")
            .finish()
    }
}

impl Drop for MTLFunctionStitchingAttribute {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLFunctionStitchingAttribute {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLFunctionStitchingAttribute::from_ptr(obj)
        }
    }
}

//
// MTLFunctionStitchingAttributeAlwaysInline
//

/// A reference to an Objective-C `MTLFunctionStitchingAttributeAlwaysInline`.
pub struct MTLFunctionStitchingAttributeAlwaysInlineRef(Object);

/// An owned Objective-C `MTLFunctionStitchingAttributeAlwaysInline`.
pub struct MTLFunctionStitchingAttributeAlwaysInline(*mut Object);

unsafe impl ForeignTypeRef for MTLFunctionStitchingAttributeAlwaysInlineRef {
    type CType = Object;
}

unsafe impl Send for MTLFunctionStitchingAttributeAlwaysInlineRef {}
unsafe impl Sync for MTLFunctionStitchingAttributeAlwaysInlineRef {}

unsafe impl ForeignType for MTLFunctionStitchingAttributeAlwaysInline {
    type CType = Object;
    type Ref = MTLFunctionStitchingAttributeAlwaysInlineRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLFunctionStitchingAttributeAlwaysInline {
        MTLFunctionStitchingAttributeAlwaysInline(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLFunctionStitchingAttributeAlwaysInlineRef> for MTLFunctionStitchingAttributeAlwaysInline {
    fn as_ref(&self) -> &MTLFunctionStitchingAttributeAlwaysInlineRef {
        unsafe { &*(self.0.cast::<MTLFunctionStitchingAttributeAlwaysInlineRef>()) }
    }
}

impl AsRef<MTLFunctionStitchingAttributeRef> for MTLFunctionStitchingAttributeAlwaysInline {
    fn as_ref(&self) -> &MTLFunctionStitchingAttributeRef {
        unsafe { &*(self.0.cast::<MTLFunctionStitchingAttributeRef>()) }
    }
}

unsafe impl Send for MTLFunctionStitchingAttributeAlwaysInline {}
unsafe impl Sync for MTLFunctionStitchingAttributeAlwaysInline {}

unsafe impl objc::Message for MTLFunctionStitchingAttributeAlwaysInlineRef {}

impl MTLFunctionStitchingAttributeAlwaysInline {
    /// Creates a new attribute for forcing function inlining.
    #[must_use]
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLFunctionStitchingAttributeAlwaysInline);
            let obj: *mut Object = msg_send![class, alloc];
            let obj: *mut Object = msg_send![obj, init];
            MTLFunctionStitchingAttributeAlwaysInline::from_ptr(obj)
        }
    }
}

impl fmt::Debug for MTLFunctionStitchingAttributeAlwaysInline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLFunctionStitchingAttributeAlwaysInline")
            .finish()
    }
}

impl Drop for MTLFunctionStitchingAttributeAlwaysInline {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLFunctionStitchingAttributeAlwaysInline {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLFunctionStitchingAttributeAlwaysInline::from_ptr(obj)
        }
    }
}

//
// MTLFunctionStitchingNode
//

/// A reference to an Objective-C `MTLFunctionStitchingNode`.
pub struct MTLFunctionStitchingNodeRef(Object);

/// An owned Objective-C `MTLFunctionStitchingNode`.
pub struct MTLFunctionStitchingNode(*mut Object);

unsafe impl ForeignTypeRef for MTLFunctionStitchingNodeRef {
    type CType = Object;
}

unsafe impl Send for MTLFunctionStitchingNodeRef {}
unsafe impl Sync for MTLFunctionStitchingNodeRef {}

unsafe impl ForeignType for MTLFunctionStitchingNode {
    type CType = Object;
    type Ref = MTLFunctionStitchingNodeRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLFunctionStitchingNode {
        MTLFunctionStitchingNode(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLFunctionStitchingNodeRef> for MTLFunctionStitchingNode {
    fn as_ref(&self) -> &MTLFunctionStitchingNodeRef {
        unsafe { &*(self.0.cast::<MTLFunctionStitchingNodeRef>()) }
    }
}

unsafe impl Send for MTLFunctionStitchingNode {}
unsafe impl Sync for MTLFunctionStitchingNode {}

unsafe impl objc::Message for MTLFunctionStitchingNodeRef {}

impl fmt::Debug for MTLFunctionStitchingNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLFunctionStitchingNode")
            .finish()
    }
}

impl Drop for MTLFunctionStitchingNode {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLFunctionStitchingNode {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLFunctionStitchingNode::from_ptr(obj)
        }
    }
}

//
// MTLFunctionStitchingInputNode
//

/// A reference to an Objective-C `MTLFunctionStitchingInputNode`.
pub struct MTLFunctionStitchingInputNodeRef(Object);

/// An owned Objective-C `MTLFunctionStitchingInputNode`.
pub struct MTLFunctionStitchingInputNode(*mut Object);

unsafe impl ForeignTypeRef for MTLFunctionStitchingInputNodeRef {
    type CType = Object;
}

unsafe impl Send for MTLFunctionStitchingInputNodeRef {}
unsafe impl Sync for MTLFunctionStitchingInputNodeRef {}

unsafe impl ForeignType for MTLFunctionStitchingInputNode {
    type CType = Object;
    type Ref = MTLFunctionStitchingInputNodeRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLFunctionStitchingInputNode {
        MTLFunctionStitchingInputNode(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLFunctionStitchingInputNodeRef> for MTLFunctionStitchingInputNode {
    fn as_ref(&self) -> &MTLFunctionStitchingInputNodeRef {
        unsafe { &*(self.0.cast::<MTLFunctionStitchingInputNodeRef>()) }
    }
}

impl AsRef<MTLFunctionStitchingNodeRef> for MTLFunctionStitchingInputNode {
    fn as_ref(&self) -> &MTLFunctionStitchingNodeRef {
        unsafe { &*(self.0.cast::<MTLFunctionStitchingNodeRef>()) }
    }
}

unsafe impl Send for MTLFunctionStitchingInputNode {}
unsafe impl Sync for MTLFunctionStitchingInputNode {}

unsafe impl objc::Message for MTLFunctionStitchingInputNodeRef {}

impl MTLFunctionStitchingInputNode {
    /// Creates a new input node.
    #[must_use]
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLFunctionStitchingInputNode);
            let obj: *mut Object = msg_send![class, alloc];
            let obj: *mut Object = msg_send![obj, init];
            MTLFunctionStitchingInputNode::from_ptr(obj)
        }
    }

    /// Creates a new input node with the given argument index.
    #[must_use]
    pub fn new_with_argument_index(argument_index: NSUInteger) -> Self {
        unsafe {
            let class = class!(MTLFunctionStitchingInputNode);
            let obj: *mut Object = msg_send![class, alloc];
            let obj: *mut Object = msg_send![obj, initWithArgumentIndex:argument_index];
            MTLFunctionStitchingInputNode::from_ptr(obj)
        }
    }

    /// Gets the argument index of this input node.
    #[must_use]
    pub fn argument_index(&self) -> NSUInteger {
        unsafe {
            let input_node_ref: &MTLFunctionStitchingInputNodeRef = self.as_ref();
            msg_send![input_node_ref, argumentIndex]
        }
    }

    /// Sets the argument index of this input node.
    pub fn set_argument_index(&self, argument_index: NSUInteger) {
        unsafe {
            let input_node_ref: &MTLFunctionStitchingInputNodeRef = self.as_ref();
            let _: () = msg_send![input_node_ref, setArgumentIndex:argument_index];
        }
    }
}

impl fmt::Debug for MTLFunctionStitchingInputNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLFunctionStitchingInputNode")
            .field("argument_index", &self.argument_index())
            .finish()
    }
}

impl Drop for MTLFunctionStitchingInputNode {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLFunctionStitchingInputNode {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLFunctionStitchingInputNode::from_ptr(obj)
        }
    }
}

//
// MTLFunctionStitchingFunctionNode
//

/// A reference to an Objective-C `MTLFunctionStitchingFunctionNode`.
pub struct MTLFunctionStitchingFunctionNodeRef(Object);

/// An owned Objective-C `MTLFunctionStitchingFunctionNode`.
pub struct MTLFunctionStitchingFunctionNode(*mut Object);

unsafe impl ForeignTypeRef for MTLFunctionStitchingFunctionNodeRef {
    type CType = Object;
}

unsafe impl Send for MTLFunctionStitchingFunctionNodeRef {}
unsafe impl Sync for MTLFunctionStitchingFunctionNodeRef {}

unsafe impl ForeignType for MTLFunctionStitchingFunctionNode {
    type CType = Object;
    type Ref = MTLFunctionStitchingFunctionNodeRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLFunctionStitchingFunctionNode {
        MTLFunctionStitchingFunctionNode(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLFunctionStitchingFunctionNodeRef> for MTLFunctionStitchingFunctionNode {
    fn as_ref(&self) -> &MTLFunctionStitchingFunctionNodeRef {
        unsafe { &*(self.0.cast::<MTLFunctionStitchingFunctionNodeRef>()) }
    }
}

impl AsRef<MTLFunctionStitchingNodeRef> for MTLFunctionStitchingFunctionNode {
    fn as_ref(&self) -> &MTLFunctionStitchingNodeRef {
        unsafe { &*(self.0.cast::<MTLFunctionStitchingNodeRef>()) }
    }
}

unsafe impl Send for MTLFunctionStitchingFunctionNode {}
unsafe impl Sync for MTLFunctionStitchingFunctionNode {}

unsafe impl objc::Message for MTLFunctionStitchingFunctionNodeRef {}

impl MTLFunctionStitchingFunctionNode {
    /// Creates a new function node.
    #[must_use]
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLFunctionStitchingFunctionNode);
            let obj: *mut Object = msg_send![class, alloc];
            let obj: *mut Object = msg_send![obj, init];
            MTLFunctionStitchingFunctionNode::from_ptr(obj)
        }
    }

    /// Creates a new function node with the given name, arguments, and control dependencies.
    #[must_use]
    pub fn new_with_name_arguments_control_dependencies(
        name: &NSString,
        arguments: &NSArray,
        control_dependencies: &NSArray
    ) -> Self {
        unsafe {
            let class = class!(MTLFunctionStitchingFunctionNode);
            let obj: *mut Object = msg_send![class, alloc];
            let obj: *mut Object = msg_send![obj, initWithName:name.as_ptr() 
                                                arguments:arguments.as_ptr() 
                                     controlDependencies:control_dependencies.as_ptr()];
            MTLFunctionStitchingFunctionNode::from_ptr(obj)
        }
    }

    /// Gets the name of this function node.
    #[must_use]
    pub fn name(&self) -> NSString {
        unsafe {
            let function_node_ref: &MTLFunctionStitchingFunctionNodeRef = self.as_ref();
            let obj: *mut Object = msg_send![function_node_ref, name];
            NSString::from_ptr(obj)
        }
    }

    /// Sets the name of this function node.
    pub fn set_name(&self, name: &NSString) {
        unsafe {
            let function_node_ref: &MTLFunctionStitchingFunctionNodeRef = self.as_ref();
            let _: () = msg_send![function_node_ref, setName:name.as_ptr()];
        }
    }

    /// Gets the arguments of this function node.
    #[must_use]
    pub fn arguments(&self) -> NSArray {
        unsafe {
            let function_node_ref: &MTLFunctionStitchingFunctionNodeRef = self.as_ref();
            let obj: *mut Object = msg_send![function_node_ref, arguments];
            NSArray::from_ptr(obj)
        }
    }

    /// Sets the arguments of this function node.
    pub fn set_arguments(&self, arguments: &NSArray) {
        unsafe {
            let function_node_ref: &MTLFunctionStitchingFunctionNodeRef = self.as_ref();
            let _: () = msg_send![function_node_ref, setArguments:arguments.as_ptr()];
        }
    }

    /// Gets the control dependencies of this function node.
    #[must_use]
    pub fn control_dependencies(&self) -> NSArray {
        unsafe {
            let function_node_ref: &MTLFunctionStitchingFunctionNodeRef = self.as_ref();
            let obj: *mut Object = msg_send![function_node_ref, controlDependencies];
            NSArray::from_ptr(obj)
        }
    }

    /// Sets the control dependencies of this function node.
    pub fn set_control_dependencies(&self, control_dependencies: &NSArray) {
        unsafe {
            let function_node_ref: &MTLFunctionStitchingFunctionNodeRef = self.as_ref();
            let _: () = msg_send![function_node_ref, setControlDependencies:control_dependencies.as_ptr()];
        }
    }
}

impl fmt::Debug for MTLFunctionStitchingFunctionNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLFunctionStitchingFunctionNode")
            .field("name", &self.name())
            .finish()
    }
}

impl Drop for MTLFunctionStitchingFunctionNode {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLFunctionStitchingFunctionNode {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLFunctionStitchingFunctionNode::from_ptr(obj)
        }
    }
}

//
// MTLFunctionStitchingGraph
//

/// A reference to an Objective-C `MTLFunctionStitchingGraph`.
pub struct MTLFunctionStitchingGraphRef(Object);

/// An owned Objective-C `MTLFunctionStitchingGraph`.
pub struct MTLFunctionStitchingGraph(*mut Object);

unsafe impl ForeignTypeRef for MTLFunctionStitchingGraphRef {
    type CType = Object;
}

unsafe impl Send for MTLFunctionStitchingGraphRef {}
unsafe impl Sync for MTLFunctionStitchingGraphRef {}

unsafe impl ForeignType for MTLFunctionStitchingGraph {
    type CType = Object;
    type Ref = MTLFunctionStitchingGraphRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLFunctionStitchingGraph {
        MTLFunctionStitchingGraph(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLFunctionStitchingGraphRef> for MTLFunctionStitchingGraph {
    fn as_ref(&self) -> &MTLFunctionStitchingGraphRef {
        unsafe { &*(self.0.cast::<MTLFunctionStitchingGraphRef>()) }
    }
}

unsafe impl Send for MTLFunctionStitchingGraph {}
unsafe impl Sync for MTLFunctionStitchingGraph {}

unsafe impl objc::Message for MTLFunctionStitchingGraphRef {}

impl MTLFunctionStitchingGraph {
    /// Creates a new function stitching graph.
    #[must_use]
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLFunctionStitchingGraph);
            let obj: *mut Object = msg_send![class, alloc];
            let obj: *mut Object = msg_send![obj, init];
            MTLFunctionStitchingGraph::from_ptr(obj)
        }
    }

    /// Creates a new function stitching graph with the given name, nodes, output node, and attributes.
    #[must_use]
    pub fn new_with_function_name_nodes_output_node_attributes(
        function_name: &NSString,
        nodes: &NSArray,
        output_node: &MTLFunctionStitchingFunctionNode,
        attributes: &NSArray
    ) -> Self {
        unsafe {
            let class = class!(MTLFunctionStitchingGraph);
            let obj: *mut Object = msg_send![class, alloc];
            let obj: *mut Object = msg_send![obj, initWithFunctionName:function_name.as_ptr() 
                                                   nodes:nodes.as_ptr() 
                                              outputNode:output_node.as_ptr()
                                              attributes:attributes.as_ptr()];
            MTLFunctionStitchingGraph::from_ptr(obj)
        }
    }

    /// Gets the function name of this graph.
    #[must_use]
    pub fn function_name(&self) -> NSString {
        unsafe {
            let graph_ref: &MTLFunctionStitchingGraphRef = self.as_ref();
            let obj: *mut Object = msg_send![graph_ref, functionName];
            NSString::from_ptr(obj)
        }
    }

    /// Sets the function name of this graph.
    pub fn set_function_name(&self, function_name: &NSString) {
        unsafe {
            let graph_ref: &MTLFunctionStitchingGraphRef = self.as_ref();
            let _: () = msg_send![graph_ref, setFunctionName:function_name.as_ptr()];
        }
    }

    /// Gets the nodes of this graph.
    #[must_use]
    pub fn nodes(&self) -> NSArray {
        unsafe {
            let graph_ref: &MTLFunctionStitchingGraphRef = self.as_ref();
            let obj: *mut Object = msg_send![graph_ref, nodes];
            NSArray::from_ptr(obj)
        }
    }

    /// Sets the nodes of this graph.
    pub fn set_nodes(&self, nodes: &NSArray) {
        unsafe {
            let graph_ref: &MTLFunctionStitchingGraphRef = self.as_ref();
            let _: () = msg_send![graph_ref, setNodes:nodes.as_ptr()];
        }
    }

    /// Gets the output node of this graph.
    #[must_use]
    pub fn output_node(&self) -> MTLFunctionStitchingFunctionNode {
        unsafe {
            let graph_ref: &MTLFunctionStitchingGraphRef = self.as_ref();
            let obj: *mut Object = msg_send![graph_ref, outputNode];
            MTLFunctionStitchingFunctionNode::from_ptr(obj)
        }
    }

    /// Sets the output node of this graph.
    pub fn set_output_node(&self, output_node: &MTLFunctionStitchingFunctionNode) {
        unsafe {
            let graph_ref: &MTLFunctionStitchingGraphRef = self.as_ref();
            let _: () = msg_send![graph_ref, setOutputNode:output_node.as_ptr()];
        }
    }

    /// Gets the attributes of this graph.
    #[must_use]
    pub fn attributes(&self) -> NSArray {
        unsafe {
            let graph_ref: &MTLFunctionStitchingGraphRef = self.as_ref();
            let obj: *mut Object = msg_send![graph_ref, attributes];
            NSArray::from_ptr(obj)
        }
    }

    /// Sets the attributes of this graph.
    pub fn set_attributes(&self, attributes: &NSArray) {
        unsafe {
            let graph_ref: &MTLFunctionStitchingGraphRef = self.as_ref();
            let _: () = msg_send![graph_ref, setAttributes:attributes.as_ptr()];
        }
    }
}

impl fmt::Debug for MTLFunctionStitchingGraph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLFunctionStitchingGraph")
            .field("function_name", &self.function_name())
            .finish()
    }
}

impl Drop for MTLFunctionStitchingGraph {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLFunctionStitchingGraph {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLFunctionStitchingGraph::from_ptr(obj)
        }
    }
}

//
// MTLStitchedLibraryDescriptor
//

/// A reference to an Objective-C `MTLStitchedLibraryDescriptor`.
pub struct MTLStitchedLibraryDescriptorRef(Object);

/// An owned Objective-C `MTLStitchedLibraryDescriptor`.
pub struct MTLStitchedLibraryDescriptor(*mut Object);

unsafe impl ForeignTypeRef for MTLStitchedLibraryDescriptorRef {
    type CType = Object;
}

unsafe impl Send for MTLStitchedLibraryDescriptorRef {}
unsafe impl Sync for MTLStitchedLibraryDescriptorRef {}

unsafe impl ForeignType for MTLStitchedLibraryDescriptor {
    type CType = Object;
    type Ref = MTLStitchedLibraryDescriptorRef;
    
    unsafe fn from_ptr(ptr: *mut Object) -> MTLStitchedLibraryDescriptor {
        MTLStitchedLibraryDescriptor(ptr)
    }

    fn as_ptr(&self) -> *mut Object {
        self.0
    }
}

impl AsRef<MTLStitchedLibraryDescriptorRef> for MTLStitchedLibraryDescriptor {
    fn as_ref(&self) -> &MTLStitchedLibraryDescriptorRef {
        unsafe { &*(self.0.cast::<MTLStitchedLibraryDescriptorRef>()) }
    }
}

unsafe impl Send for MTLStitchedLibraryDescriptor {}
unsafe impl Sync for MTLStitchedLibraryDescriptor {}

unsafe impl objc::Message for MTLStitchedLibraryDescriptorRef {}

impl MTLStitchedLibraryDescriptor {
    /// Creates a new stitched library descriptor.
    #[must_use]
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLStitchedLibraryDescriptor);
            let obj: *mut Object = msg_send![class, alloc];
            let obj: *mut Object = msg_send![obj, init];
            MTLStitchedLibraryDescriptor::from_ptr(obj)
        }
    }

    /// Gets the function graphs of this descriptor.
    #[must_use]
    pub fn function_graphs(&self) -> NSArray {
        unsafe {
            let descriptor_ref: &MTLStitchedLibraryDescriptorRef = self.as_ref();
            let obj: *mut Object = msg_send![descriptor_ref, functionGraphs];
            NSArray::from_ptr(obj)
        }
    }

    /// Sets the function graphs of this descriptor.
    pub fn set_function_graphs(&self, function_graphs: &NSArray) {
        unsafe {
            let descriptor_ref: &MTLStitchedLibraryDescriptorRef = self.as_ref();
            let _: () = msg_send![descriptor_ref, setFunctionGraphs:function_graphs.as_ptr()];
        }
    }

    /// Gets the functions of this descriptor.
    #[must_use]
    pub fn functions(&self) -> NSArray {
        unsafe {
            let descriptor_ref: &MTLStitchedLibraryDescriptorRef = self.as_ref();
            let obj: *mut Object = msg_send![descriptor_ref, functions];
            NSArray::from_ptr(obj)
        }
    }

    /// Sets the functions of this descriptor.
    pub fn set_functions(&self, functions: &NSArray) {
        unsafe {
            let descriptor_ref: &MTLStitchedLibraryDescriptorRef = self.as_ref();
            let _: () = msg_send![descriptor_ref, setFunctions:functions.as_ptr()];
        }
    }

    /// Gets the binary archives of this descriptor.
    #[must_use]
    pub fn binary_archives(&self) -> NSArray {
        unsafe {
            let descriptor_ref: &MTLStitchedLibraryDescriptorRef = self.as_ref();
            let obj: *mut Object = msg_send![descriptor_ref, binaryArchives];
            NSArray::from_ptr(obj)
        }
    }

    /// Sets the binary archives of this descriptor.
    pub fn set_binary_archives(&self, binary_archives: &NSArray) {
        unsafe {
            let descriptor_ref: &MTLStitchedLibraryDescriptorRef = self.as_ref();
            let _: () = msg_send![descriptor_ref, setBinaryArchives:binary_archives.as_ptr()];
        }
    }

    /// Gets the options of this descriptor.
    #[must_use]
    pub fn options(&self) -> MTLStitchedLibraryOptions {
        unsafe {
            let descriptor_ref: &MTLStitchedLibraryDescriptorRef = self.as_ref();
            let options: u64 = msg_send![descriptor_ref, options];
            std::mem::transmute(options)
        }
    }

    /// Sets the options of this descriptor.
    pub fn set_options(&self, options: MTLStitchedLibraryOptions) {
        unsafe {
            let descriptor_ref: &MTLStitchedLibraryDescriptorRef = self.as_ref();
            let _: () = msg_send![descriptor_ref, setOptions:options as u64];
        }
    }
}

impl Default for MTLStitchedLibraryDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for MTLStitchedLibraryDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MTLStitchedLibraryDescriptor")
            .field("options", &self.options())
            .finish()
    }
}

impl Drop for MTLStitchedLibraryDescriptor {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.0, release];
        }
    }
}

impl Clone for MTLStitchedLibraryDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![self.0, retain];
            MTLStitchedLibraryDescriptor::from_ptr(obj)
        }
    }
}