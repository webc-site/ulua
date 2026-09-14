use core::ffi::CStr;

use ulua_ast::records::ast_stat_type_alias::AstStatTypeAlias;

use crate::{records::usage_finder::UsageFinder, type_aliases::name_type::Name};
impl UsageFinder {
  /// # Safety
  /// 调用方须保证 `alias` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_stat_type_alias(&mut self, alias: *mut AstStatTypeAlias) -> bool {
    let alias_ref = unsafe { &*alias };
    let name_str = unsafe {
      CStr::from_ptr(alias_ref.name.value)
        .to_string_lossy()
        .into_owned()
    };
    self.declared_aliases.insert(Name::from(name_str));
    true
  }
}
