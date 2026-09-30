use crate::{
  functions::comparison_type_function::comparison_type_function, macros::numeric_binop_wrapper,
};

numeric_binop_wrapper!(
  /// 对应 C++ `ltTypeFunction`（`BuiltinTypeFunctions.cpp`，转调
  /// `comparisonTypeFunction(..., "__lt")`）。
  lt_type_function,
  "__lt",
  comparison_type_function
);
