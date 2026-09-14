use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::type_function_instance_state::TypeFunctionInstanceState,
  functions::get_mutable_type::get_mutable_type_id,
  records::{
    type_function_instance_type::TypeFunctionInstanceType,
    type_function_reducer::TypeFunctionReducer,
  },
  type_aliases::type_id::TypeId,
};

impl TypeFunctionReducer {
  /// C++ `TypeFunction.cpp:353 setState(TypeId, ...)`。
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn set_state_type_id_type_function_instance_state(
    &self,
    ty: TypeId,
    state: TypeFunctionInstanceState,
  ) {
    // SAFETY: ty 为 arena 内有效句柄，与 C++ 同契约。
    if unsafe { (*ty).owning_arena } != unsafe { (*self.ctx.as_ptr()).arena.as_ptr() } {
      return;
    }

    let tfit = get_mutable_type_id::<TypeFunctionInstanceType>(ty);
    LUAU_ASSERT!(tfit.is_some());
    // 断言保证命中，与 C++ `LUAU_ASSERT(tfit); tfit->state = state` 一致
    tfit.unwrap().state = state;
  }
}
