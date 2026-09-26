use crate::{
  enums::type_function_instance_state::TypeFunctionInstanceState,
  records::type_function_reducer::TypeFunctionReducer, type_aliases::type_pack_id::TypePackId,
};

impl TypeFunctionReducer {
  /// C++ `TypeFunction.cpp:370 setState(TypePackId, ...)`：上游即为显式 no-op
  /// （"We do not presently have any type pack functions at all."），保留同名
  /// 单体以对齐调用点；出现带状态的 pack family 时在此落地。
  pub fn set_state_type_pack_id(&self, _tp: TypePackId, _state: TypeFunctionInstanceState) {}
}
