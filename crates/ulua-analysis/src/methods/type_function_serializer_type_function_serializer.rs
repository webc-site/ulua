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
  /// 调用方须保证 `state` 非空、对齐，指向序列化期存活的 builder state，其 `ctx` 构造接线恒非空
  /// （`(*state).ctx` 解引用取 `type_function_runtime`）。该指针与 runtime 地址被存入 self，须在 builder
  /// 生命周期内保持有效、bump arena 块地址不移动。cpp `Analysis/src/TypeFunctionRuntimeBuilder.cpp:53`。单线程独占。
  pub unsafe fn type_function_serializer(&mut self, state: *mut TypeFunctionRuntimeBuilderState) {
    self.state = state;
    self.type_function_runtime = unsafe {
      // 契约：`TypeFunctionRuntimeBuilderState::new` 以存活 `&mut TypeFunctionContext`
      // 接线 ctx（`Handle` 类型编码非空，cpp `state->ctx->` 同前提直解），
      // 故句柄物化的借用恒有效。
      (*state).ctx.get().type_function_runtime.as_ptr()
    };
    self.queue = Vec::new();
    self.types = SeenTypes::new(null());
    self.packs = SeenTypePacks::new(null());
    self.steps = 0;
  }
}
