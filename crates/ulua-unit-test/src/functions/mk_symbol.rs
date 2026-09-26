use alloc::{boxed::Box, vec::Vec};

use ulua_analysis::records::symbol::Symbol;
use ulua_ast::records::ast_name::AstName;

/// 测试专用：把 `&str` 变成存活到进程结束的 `Symbol`。`AstName::value` 要求
/// NUL 结尾缓冲，故泄漏「字节 + 尾 NUL」的 `Box<[u8]>`，经 `from_static` 收口；
/// 不用 `CString`（散落的堆 C 串），泄漏范围仅限测试进程。
pub fn mk_symbol(s: &str) -> Symbol {
  let mut bytes: Vec<u8> = Vec::with_capacity(s.len() + 1);
  bytes.extend_from_slice(s.as_bytes());
  bytes.push(0);
  Symbol::from_global(AstName::from_static(Box::leak(bytes.into_boxed_slice())))
}
