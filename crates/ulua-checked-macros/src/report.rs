use proc_macro2::{Span, TokenStream};
use syn::Error;

/// 单条编译错误出口：一律经 `syn::Error::to_compile_error`，不散落 `compile_error!`。
fn compile_error(span: Span, message: impl Into<String>) -> TokenStream {
  Error::new(span, message.into()).to_compile_error()
}

/// 类型检查诊断的统一出口：Luau 侧只有 1 基行列、没有可映射的 Rust span，
/// 汇总报在宏入口 span 上。
pub(crate) fn diagnostics_error(
  span: Span,
  diagnostics: &[ulua_rt::TypeDiagnostic],
) -> TokenStream {
  let mut message = String::from("Luau type check failed");
  for diagnostic in diagnostics {
    message.push('\n');
    message.push_str("  ");
    message.push_str(&diagnostic.to_string());
  }
  compile_error(span, message)
}
