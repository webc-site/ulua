use core::ptr::null_mut;

use crate::{
  records::{
    type_function_deserializer::TypeFunctionDeserializer,
    type_function_runtime_builder_state::TypeFunctionRuntimeBuilderState,
  },
  type_aliases::{
    seen_type_packs_type_function_runtime_builder_alt_d::SeenTypePacks,
    seen_types_type_function_runtime_builder_alt_d::SeenTypes,
  },
};
impl TypeFunctionDeserializer {
  /// # Safety
  /// 调用方须保证 `state` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn type_function_deserializer(&mut self, state: *mut TypeFunctionRuntimeBuilderState) {
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
    self.types = SeenTypes::new(null_mut());
    self.packs = SeenTypePacks::new(null_mut());
    self.generic_types = Vec::new();
    self.generic_packs = Vec::new();
    self.function_scopes = Vec::new();
    self.steps = 0;
  }
}
