use crate::macros::numeric_binop_wrapper;

numeric_binop_wrapper!(
  /// 对应 C++ `addTypeFunction`（BuiltinTypeFunctions.cpp:490-505）：校验 `__add`
  /// 的实参形态（2 个类型参、0 个类型包参）后转发 `numeric_binop_type_function`。
  add_type_function,
  "__add"
);
