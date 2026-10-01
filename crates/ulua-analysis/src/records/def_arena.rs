use crate::records::{def::Def, typed_allocator::TypedAllocator};

#[derive(Debug)]
pub struct DefArena {
  pub allocator: TypedAllocator<Def>,
}

impl Default for DefArena {
  fn default() -> Self {
    Self {
      allocator: TypedAllocator::new(),
    }
  }
}
