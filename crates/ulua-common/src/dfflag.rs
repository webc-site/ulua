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
}
pub use _inner::{
  ADD_RETURN_EXECTARGET_CHECK as AddReturnExectargetCheck,
  DEBUG_LUAU_REPORT_RETURN_TYPE_VARIADIC_WITH_TYPE_SUFFIX as DebugLuauReportReturnTypeVariadicWithTypeSuffix,
};

/// C++ `FValue` ctor 的 `list = this` 自注册对应物：把本文件的 DFFlag 挂入
/// `FValue<bool>` 链表，否则 `--fflags=` 按名遍历看不到它们（上游
/// `LUAU_DYNAMIC_FASTFLAGVARIABLE` 同样是 `FValue<bool>` 实例）。
///
/// # Safety
/// 每个 flag 仅注册一次（由 `ensure_flags_registered` 的 `OnceLock` 串行化），
/// 且早于任何并发链表遍历。
pub fn register_flags() {
  unsafe {
    _inner::ADD_RETURN_EXECTARGET_CHECK.register();
    _inner::DEBUG_LUAU_REPORT_RETURN_TYPE_VARIADIC_WITH_TYPE_SUFFIX.register();
  }
}
