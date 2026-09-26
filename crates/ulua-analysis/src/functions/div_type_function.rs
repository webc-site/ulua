use crate::macros::numeric_binop_wrapper;

numeric_binop_wrapper!(
  /// 对应 C++ `divTypeFunction`（`cpp/Analysis/src/BuiltinTypeFunctions.cpp:539`）。
  div_type_function,
  "__div"
);
