use block2::{Block, RcBlock};
use objc2::{rc::Retained, runtime::ProtocolObject};
use objc2_foundation::NSError;

use crate::{CallbackBlock, MTLRenderPipelineReflection, MTLRenderPipelineState, MetalError};

type RenderPipelineStateBlock = dyn Fn(*mut ProtocolObject<dyn MTLRenderPipelineState>, *mut NSError);
type RenderPipelineStateWithReflectionBlock =
    dyn Fn(*mut ProtocolObject<dyn MTLRenderPipelineState>, *mut MTLRenderPipelineReflection, *mut NSError);

/// The Metal completion handler for asynchronous compute-pipeline creation
/// that also returns reflection information.
pub type MTLNewComputePipelineStateWithReflectionCompletionHandler =
    crate::NewComputePipelineStateWithReflectionCompletionHandler;

/// A completion handler for asynchronous render-pipeline creation.
pub struct MTLNewRenderPipelineStateCompletionHandler(RcBlock<RenderPipelineStateBlock>);

impl MTLNewRenderPipelineStateCompletionHandler {
    pub fn new<F>(handler: F) -> Self
    where
        F: Fn(Option<Retained<ProtocolObject<dyn MTLRenderPipelineState>>>, Option<MetalError>) + 'static,
    {
        Self(RcBlock::new(move |pipeline_ptr: *mut ProtocolObject<dyn MTLRenderPipelineState>, error: *mut NSError| {
            let pipeline = unsafe { Retained::retain(pipeline_ptr) };
            let error = unsafe { MetalError::from_unretained(error) };
            handler(pipeline, error);
        }))
    }
}

impl CallbackBlock for MTLNewRenderPipelineStateCompletionHandler {
    type Signature = RenderPipelineStateBlock;

    fn as_block(&self) -> &Block<Self::Signature> {
        &self.0
    }
}

/// A completion handler for asynchronous render-pipeline creation that also
/// returns reflection information.
pub struct MTLNewRenderPipelineStateWithReflectionCompletionHandler(RcBlock<RenderPipelineStateWithReflectionBlock>);

impl MTLNewRenderPipelineStateWithReflectionCompletionHandler {
    pub fn new<F>(handler: F) -> Self
    where
        F: Fn(
                Option<Retained<ProtocolObject<dyn MTLRenderPipelineState>>>,
                Option<Retained<MTLRenderPipelineReflection>>,
                Option<MetalError>,
            ) + 'static,
    {
        Self(RcBlock::new(
            move |state_ptr: *mut ProtocolObject<dyn MTLRenderPipelineState>,
                  reflection_ptr: *mut MTLRenderPipelineReflection,
                  error: *mut NSError| {
                let state = unsafe { Retained::retain(state_ptr) };
                let reflection = unsafe { Retained::retain(reflection_ptr) };
                let error = unsafe { MetalError::from_unretained(error) };
                handler(state, reflection, error);
            },
        ))
    }
}

impl CallbackBlock for MTLNewRenderPipelineStateWithReflectionCompletionHandler {
    type Signature = RenderPipelineStateWithReflectionBlock;

    fn as_block(&self) -> &Block<Self::Signature> {
        &self.0
    }
}
