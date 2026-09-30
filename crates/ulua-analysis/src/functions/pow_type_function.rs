use crate::macros::numeric_binop_wrapper;

numeric_binop_wrapper!(
  /// 对应 C++ `powTypeFunction`（BuiltinTypeFunctions.cpp:571），转调
  /// `numericBinopTypeFunction(..., "__pow")`。
  pow_type_function,
  "__pow"
);
