use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::type_function_instance_state::TypeFunctionInstanceState,
  functions::get_type_alt_j::get_type_id,
  records::{
    type_function_instance_type::TypeFunctionInstanceType,
    type_function_reducer::TypeFunctionReducer,
  },
  type_aliases::type_id::TypeId,
};

impl TypeFunctionReducer {
  /// C++ `TypeFunction.cpp:346 getState(TypeId)`。
  pub fn get_state_type_id(&self, ty: TypeId) -> TypeFunctionInstanceState {
    let tfit = get_type_id::<TypeFunctionInstanceType>(ty);
    LUAU_ASSERT!(tfit.is_some());
    // 断言保证命中，与 C++ `LUAU_ASSERT(tfit); tfit->state` 一致
    tfit.unwrap().state
  }
}
