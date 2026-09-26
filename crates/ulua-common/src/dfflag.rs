//! 动态 bool 标志（`dfflag` 模块），对应 `fflag`。

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
  // Require/src/RequireNavigator.cpp
  crate::LUAU_DYNAMIC_FASTFLAGVARIABLE!(
    LUAU_SELF_IS_SELF_AND_ALWAYS_SELF,
    LuauSelfIsSelfAndAlwaysSelf,
    false
  );
}

/// 宏自产的 Pascal 别名（宏展开内含 `pub use X as Y;`）经 glob 一次性再导出；
/// 别名对与宏定义同 token 生成，一致性由编译器而非人工清单保证。
pub use _inner::*;

/// C++ `FValue` ctor 的 `list = this` 自注册对应物：把本文件的 DFFlag 挂入
/// `FValue<bool>` 注册表，否则 `--fflags=` 按名遍历看不到它们（上游
/// `LUAU_DYNAMIC_FASTFLAGVARIABLE` 同样是 `FValue<bool>` 实例）。仅本 crate 消费，
/// 降 `pub(crate)`（b28 零消费点收口）。
pub(crate) fn register_flags() {
  _inner::ADD_RETURN_EXECTARGET_CHECK.register();
  _inner::DEBUG_LUAU_REPORT_RETURN_TYPE_VARIADIC_WITH_TYPE_SUFFIX.register();
  _inner::LUAU_SELF_IS_SELF_AND_ALWAYS_SELF.register();
}
