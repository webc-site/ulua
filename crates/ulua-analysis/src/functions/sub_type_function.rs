use crate::macros::numeric_binop_wrapper;

numeric_binop_wrapper!(
  /// 对应 C++ `subTypeFunction`（`cpp/Analysis/src/BuiltinTypeFunctions.cpp:520`，
  /// 转调 `numericBinopTypeFunction(..., "__sub")`）。
  sub_type_function,
  "__sub"
);
