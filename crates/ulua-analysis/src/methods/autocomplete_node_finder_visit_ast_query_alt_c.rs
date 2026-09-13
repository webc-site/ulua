use ulua_ast::records::{ast_node::AstNode, ast_type::AstType};

use crate::records::autocomplete_node_finder::AutocompleteNodeFinder;
impl AutocompleteNodeFinder {
  /// # Safety
  /// 调用方须保证 `type_` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn visit_ast_type(&mut self, type_: *mut AstType) -> bool {
    let location = unsafe { (*type_).base.location };
    if location.begin < self.pos && self.pos <= location.end {
      self.ancestry.push(type_ as *mut AstNode);
      return true;
    }
    false
  }
}
