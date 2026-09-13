use ulua_ast::{records::ast_type::AstType, visit::ast_type_visit};

use crate::records::find_nth_occurence_of::FindNthOccurenceOf;
impl FindNthOccurenceOf {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn visit_ast_type(&mut self, t: *mut AstType) -> bool {
    unsafe {
      ast_type_visit(t, self);
    }
    !self.the_node.is_null()
  }
}
