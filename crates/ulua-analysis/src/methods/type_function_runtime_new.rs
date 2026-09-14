use alloc::vec::Vec;
use core::ptr::null_mut;

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  records::{
    internal_error_reporter::InternalErrorReporter, type_check_limits::TypeCheckLimits,
    type_function_runtime::TypeFunctionRuntime, typed_allocator::TypedAllocator,
  },
  type_aliases::scope_ptr_type::ScopePtr,
};
impl TypeFunctionRuntime {
  pub fn new(ice: &InternalErrorReporter, limits: &TypeCheckLimits, root_scope: ScopePtr) -> Self {
    Self {
      ice: ice.clone(),
      limits: limits.clone(),
      type_arena: TypedAllocator::default(),
      type_pack_arena: TypedAllocator::default(),
      state: (null_mut(), None),
      initialized: DenseHashSet::new(null_mut()),
      allow_evaluation: true,
      root_scope,
      messages: Vec::new(),
      runtime_builder: null_mut(),
    }
  }
}
