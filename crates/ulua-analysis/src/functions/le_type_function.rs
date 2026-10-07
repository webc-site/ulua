use crate::{
  functions::comparison_type_function::comparison_type_function, macros::numeric_binop_wrapper,
};

numeric_binop_wrapper!(
  /// 对应 C++ `leTypeFunction`（`BuiltinTypeFunctions.cpp`，转调
  /// `comparisonTypeFunction(..., "__le")`）。
  le_type_function,
  "__le",
  comparison_type_function
);
