use crate::{
  enums::type_function_instance_state::TypeFunctionInstanceState,
  records::type_function_reducer::TypeFunctionReducer, type_aliases::type_pack_id::TypePackId,
};

impl TypeFunctionReducer {
  /// C++ `TypeFunction.cpp:365 getState(TypePackId)`。
  pub fn get_state_type_pack_id(&self, _tp: TypePackId) -> TypeFunctionInstanceState {
    TypeFunctionInstanceState::Unsolved
  }
}
