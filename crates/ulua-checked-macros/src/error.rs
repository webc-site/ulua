//! 宏展开的失败类型：区分「带 Rust span 的输入错误」与「只有 Luau 位置的
//! 类型检查诊断」。前者必须原样带 span 落到用户写错的那个字面量上，
//! 后者没有 Rust 位置可用，只能报在宏入口 span 上。

use ulua_rt::TypeDiagnostic;

/// [`crate::modules_check::check`] / [`crate::modules_check::check_dispatch`]
/// 的失败原因。
#[derive(Debug, thiserror::Error)]
pub(crate) enum CheckFailure {
  /// 读盘失败、`CARGO_MANIFEST_DIR` 缺失、路径非法等：`syn::Error` 已带
  /// 字面量 span，展开侧 `to_compile_error()` 即可精确定位（旧写法把它
  /// `to_string()` 后重新合成 (1,1) 诊断，定位信息在这一步丢掉）。
  #[error(transparent)]
  Syn(#[from] syn::Error),
  /// 类型检查诊断：Luau 侧只有 1 基行列，没有可映射的 Rust span，
  /// 由 [`crate::report`] 汇总报在宏入口 span 上。
  #[error("Luau type check failed")]
  Diagnostics(Vec<TypeDiagnostic>),
}
