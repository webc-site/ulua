//! Source: `Analysis/src/TypeFunctionRuntimeBuilder.cpp:593-606`
//!
//! ```cpp
//! TypeId deserialize(TypeFunctionTypeId ty)
//! {
//!     shallowDeserialize(ty);
//!     run();
//!
//!     if (hasExceededIterationLimit() || hasErrors())
//!     {
//!         TypeId error = state->ctx->builtins->error_type;
//!         types[ty] = error;
//!         return error;
//!     }
//!
//!     return find(ty).value_or(state->ctx->builtins->error_type);
//! }
//! ```
use crate::{
  records::type_function_deserializer::TypeFunctionDeserializer,
  type_aliases::{type_function_type_id::TypeFunctionTypeId, type_id::TypeId},
};

impl TypeFunctionDeserializer {
  pub fn deserialize_type_function_type_id(&mut self, ty: TypeFunctionTypeId) -> TypeId {
    self.shallow_deserialize_type_function_type_id(ty);
    self.run();

    if self.has_exceeded_iteration_limit() || self.has_errors() {
      // Safety: self.state 由 builder 入口注入且在本轮反序列化期间于 builder 栈上存活；
      // (*self.state).ctx 是 Handle（类型编码非空），物化后指向调用方持有的存活
      // TypeFunctionContext，其 builtins 是构造期接线的 NonNull<BuiltinTypes>（as_ptr 恒非空、比 ctx 长寿），error_type 为按值读出的
      // TypeId 字段，全程只读不写。
      let error: TypeId = unsafe { (*(*self.state).ctx.get().builtins.as_ptr()).error_type };
      *self.types.get_or_insert(ty) = error;
      return error;
    }

    // Safety: 同上——state/ctx 存活、builtins 为 NonNull 恒非空，仅按值只读 error_type。
    let error: TypeId = unsafe { (*(*self.state).ctx.get().builtins.as_ptr()).error_type };
    self.find_type_function_type_id(ty).unwrap_or(error)
  }
}
