use crate::macros::numeric_binop_wrapper;

numeric_binop_wrapper!(
  /// 对应 C++ `idivTypeFunction`（BuiltinTypeFunctions.cpp:555），转调
  /// `numericBinopTypeFunction(..., "__idiv")`。
  idiv_type_function,
  "__idiv"
);
