//! 动态 bool 标志（`DFFlag::`），对应 `FFlag`。

pub mod _inner {
  // CodeGen/src/EmitCommonX64.cpp
  crate::LUAU_DYNAMIC_FASTFLAGVARIABLE!(
    ADD_RETURN_EXECTARGET_CHECK,
    AddReturnExectargetCheck,
    false
  );
  // Ast/src/Parser.cpp
  crate::LUAU_DYNAMIC_FASTFLAGVARIABLE!(
    DEBUG_LUAU_REPORT_RETURN_TYPE_VARIADIC_WITH_TYPE_SUFFIX,
    DebugLuauReportReturnTypeVariadicWithTypeSuffix,
    false
  );
}
pub use _inner::{
  ADD_RETURN_EXECTARGET_CHECK as AddReturnExectargetCheck,
  DEBUG_LUAU_REPORT_RETURN_TYPE_VARIADIC_WITH_TYPE_SUFFIX as DebugLuauReportReturnTypeVariadicWithTypeSuffix,
};
