use core::ptr::null_mut;

use crate::{
  records::{
    type_function_deserializer::TypeFunctionDeserializer,
    type_function_runtime_builder_state::TypeFunctionRuntimeBuilderState,
  },
  type_aliases::{
    seen_type_packs_type_function_runtime_builder::DeserializedTypePacks,
    seen_types_type_function_runtime_builder::DeserializerSeenTypes,
  },
};
impl TypeFunctionDeserializer {
  /// # Safety
  /// 调用方须保证 `state` 非空、对齐，指向反序列化期存活的 builder state，其 `ctx` 构造接线恒非空。
  /// 该指针与 runtime 地址存入 self 后须在 builder 生命周期内有效、arena 块地址不移动。
  /// cpp `Analysis/src/TypeFunctionRuntimeBuilder.cpp:588`。单线程独占。
  pub unsafe fn type_function_deserializer(&mut self, state: *mut TypeFunctionRuntimeBuilderState) {
    self.state = state;
    self.type_function_runtime = unsafe {
      // 契约：`TypeFunctionRuntimeBuilderState::new` 以存活 `&mut TypeFunctionContext`
      // 接线 ctx（`Handle` 类型编码非空，cpp `state->ctx->` 同前提直解），
      // 故句柄物化的借用恒有效。
      (*state).ctx.get().type_function_runtime.as_ptr()
    };
    self.queue = Vec::new();
    self.types = DeserializerSeenTypes::new(null_mut());
    self.packs = DeserializedTypePacks::new(null_mut());

    self.generic_types = Vec::new();
    self.generic_packs = Vec::new();
    self.function_scopes = Vec::new();
    self.steps = 0;
  }
}
