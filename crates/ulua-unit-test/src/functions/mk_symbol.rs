use ulua_analysis::records::symbol::Symbol;
use ulua_ast::records::ast_name::AstName;

/// 测试专用：把名字面量变成 `Symbol`（全局臂）。
///
/// `Symbol` 的 `Eq`/`Hash` 按 `global` 的**字节内容**判等，故此处无需驻留或泄漏：
/// 名字恒为调用点的 `'static` 字面量，直接经 `AstName::from_str` 借用静态字节即可，
/// 不再伪造 NUL 结尾缓冲、也不再泄漏堆块换取 `'static`。
#[inline]
pub fn mk_symbol(s: &'static str) -> Symbol {
  Symbol::from_global(AstName::from_str(s))
}
