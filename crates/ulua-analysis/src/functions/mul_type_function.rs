use crate::macros::numeric_binop_wrapper;

numeric_binop_wrapper!(
  /// 对应 C++ `mulTypeFunction`（BuiltinTypeFunctions.cpp:523），转调
  /// `numericBinopTypeFunction(..., "__mul")`。
  mul_type_function,
  "__mul"
);
