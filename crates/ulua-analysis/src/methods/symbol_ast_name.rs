use ulua_ast::records::ast_name::AstName;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::symbol::Symbol;

impl Symbol {
  /// 返回 `&AstName` 而非按值的 `AstName`：`AstName::as_str` 系列的输出寿命
  /// 取自对 `AstName` 的借用（不再凭空 `'static`），按值返回会让调用方拿到的
  /// `&str` 挂到临时对象上（E0515）。字符串本体由 `AstNameTable` 持有，寿命
  /// 覆盖 `Symbol` 所属 arena。
  pub fn ast_name(&self) -> &AstName {
    if !self.local.is_null() {
      // SAFETY: local 指向 arena 存活的 AstLocal；Symbol 与其 arena 同生命周期
      // （cpp 不变式），故借用不超过 `&self` 的寿命。
      return unsafe { &(*self.local).name };
    }

    LUAU_ASSERT!(!self.global.value.is_null());
    &self.global
  }
}
