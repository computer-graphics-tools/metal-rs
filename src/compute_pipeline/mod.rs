mod completion_handler;
mod descriptor;
mod reflection;
mod state;

pub use completion_handler::{
    NewComputePipelineStateCompletionHandler, NewComputePipelineStateWithReflectionCompletionHandler,
};
pub use descriptor::MTLComputePipelineDescriptor;
pub use reflection::MTLComputePipelineReflection;
pub use state::{MTLComputePipelineState, MTLComputePipelineStateExt};

#[cfg(test)]
mod tests {
    use super::{MTLComputePipelineReflection, MTLComputePipelineState};
    use crate::MTLAllocation;

    fn assert_send_sync<T: Send + Sync + ?Sized>() {}
    fn assert_allocation<T: MTLAllocation + ?Sized>() {}

    #[test]
    fn pipeline_types_match_header_conformance() {
        assert_send_sync::<MTLComputePipelineReflection>();
        assert_send_sync::<dyn MTLComputePipelineState>();
        assert_allocation::<dyn MTLComputePipelineState>();
    }
}
