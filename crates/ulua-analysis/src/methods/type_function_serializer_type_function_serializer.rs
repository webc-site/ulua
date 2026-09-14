use alloc::vec::Vec;
use core::ptr::null;

use crate::{
  records::{
    type_function_runtime_builder_state::TypeFunctionRuntimeBuilderState,
    type_function_serializer::TypeFunctionSerializer,
  },
  type_aliases::{
    seen_type_packs_type_function_runtime_builder::SeenTypePacks,
    seen_types_type_function_runtime_builder::SeenTypes,
  },
};
impl TypeFunctionSerializer {
  /// # Safety
  /// 调用方须保证 `state` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn type_function_serializer(&mut self, state: *mut TypeFunctionRuntimeBuilderState) {
    self.state = state;
    self.type_function_runtime = unsafe {
      (*state)
        .ctx
        .as_ref()
        .unwrap()
        .type_function_runtime
        .as_ptr()
    };
    self.queue = Vec::new();
    self.types = SeenTypes::new(null());
    self.packs = SeenTypePacks::new(null());
    self.steps = 0;
  }
}
