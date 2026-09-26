use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::type_function_instance_state::TypeFunctionInstanceState,
  functions::get_type,
  records::{
    type_function_instance_type::TypeFunctionInstanceType,
    type_function_reducer::TypeFunctionReducer,
  },
  type_aliases::type_id::TypeId,
};

impl TypeFunctionReducer {
  /// C++ `TypeFunction.cpp:346 getState(TypeId)`。
  pub fn get_state_type_id(&self, ty: TypeId) -> TypeFunctionInstanceState {
    let tfit = get_type::get::<TypeFunctionInstanceType>(ty);
    LUAU_ASSERT!(tfit.is_some());
    // 紧邻 LUAU_ASSERT 蕴含 Some，与 C++ `LUAU_ASSERT(tfit); tfit->state` 一致
    tfit.expect("紧邻 LUAU_ASSERT(tfit.is_some()) 蕴含").state
  }
}
