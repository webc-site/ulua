use crate::records::{refinement_key::RefinementKey, typed_allocator::TypedAllocator};

#[derive(Debug, Default)]
pub struct RefinementKeyArena {
  pub(crate) allocator: TypedAllocator<RefinementKey>,
}
