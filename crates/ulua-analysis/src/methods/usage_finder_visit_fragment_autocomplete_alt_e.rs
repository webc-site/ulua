use ulua_ast::records::ast_type_reference::AstTypeReference;

use crate::records::usage_finder::UsageFinder;
impl UsageFinder {
  /// # Safety
  /// 调用方须保证 `ref_` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_type_reference(&mut self, ref_: *mut AstTypeReference) -> bool {
    let ref_ = unsafe { &*ref_ };
    if let Some(prefix) = ref_.prefix {
      let prefix_value = prefix.as_str_or_empty().to_string();
      let name_value = ref_.name.as_str_or_empty().to_string();
      self
        .referenced_imported_bindings
        .push((prefix_value, name_value));
    } else {
      let name_value = ref_.name.as_str_or_empty().to_string();
      self.referenced_bindings.push(name_value);
    }
    true
  }
}
