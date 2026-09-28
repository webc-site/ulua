//! 动态 bool 标志（`dfflag` 模块），对应 `fflag`。

use crate::macros::fast_flags::luau_flag_module;

luau_flag_module! {
  // CodeGen/src/EmitCommonX64.cpp
  LUAU_DYNAMIC_FASTFLAGVARIABLE!(ADD_RETURN_EXECTARGET_CHECK, AddReturnExectargetCheck, false);
  // Ast/src/Parser.cpp
  LUAU_DYNAMIC_FASTFLAGVARIABLE!(DEBUG_LUAU_REPORT_RETURN_TYPE_VARIADIC_WITH_TYPE_SUFFIX, DebugLuauReportReturnTypeVariadicWithTypeSuffix, false);
  // Require/src/RequireNavigator.cpp
  LUAU_DYNAMIC_FASTFLAGVARIABLE!(LUAU_SELF_IS_SELF_AND_ALWAYS_SELF, LuauSelfIsSelfAndAlwaysSelf, false);
}
