use proc_macro2::{Span, TokenStream};
use syn::Error;

use crate::error::CheckFailure;

pub fn compile_error(span: Span, message: impl Into<String>) -> TokenStream {
  Error::new(span, message.into()).to_compile_error()
}

/// 检查失败的统一出口：`Syn` 走 `syn::Error` 自带的字面量 span，
/// 只有 `Diagnostics` 才汇总到宏入口 span 上。
pub fn failure_error(entry_span: Span, failure: CheckFailure) -> TokenStream {
  match failure {
    CheckFailure::Syn(err) => err.to_compile_error(),
    CheckFailure::Diagnostics(diagnostics) => diagnostics_error(entry_span, &diagnostics),
  }
}

pub fn diagnostics_error(span: Span, diagnostics: &[ulua_rt::TypeDiagnostic]) -> TokenStream {
  let mut message = String::from("Luau type check failed");
  for diagnostic in diagnostics {
    message.push('\n');
    message.push_str("  ");
    message.push_str(&diagnostic.to_string());
  }
  compile_error(span, message)
}
