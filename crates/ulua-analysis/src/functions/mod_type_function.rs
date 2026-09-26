use crate::macros::numeric_binop_wrapper;

numeric_binop_wrapper!(
  /// 对应 C++ `modTypeFunction`（BuiltinTypeFunctions.cpp:587），转调
  /// `numericBinopTypeFunction(..., "__mod")`。
  mod_type_function,
  "__mod"
);
