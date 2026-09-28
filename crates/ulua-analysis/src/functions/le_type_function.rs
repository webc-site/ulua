use crate::{
  functions::comparison_type_function::comparison_type_function, macros::numeric_binop_wrapper,
};

numeric_binop_wrapper!(
  /// 对应 C++ `leTypeFunction`（`BuiltinTypeFunctions.cpp`，转调
  /// `comparisonTypeFunction(..., "__le")`）。
  ///
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  le_type_function,
  "__le",
  comparison_type_function
);
