use ulua_ast::records::{ast_stat_for::AstStatFor, ast_type::AstType};

use crate::records::type_map_visitor::TypeMapVisitor;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) fn visit_ast_stat_for(this: &mut TypeMapVisitor<'_>, node: *mut AstStatFor) -> bool {
  unsafe {
    if !node.is_null() {
      let n = &*node;
      let ty = &this.builtin_types.number_type as *const _ as *const AstType;
      this.record_resolved_type_ast_local_ast_type(n.var, ty);
    }
  }
  true
}

impl TypeMapVisitor<'_> {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub(crate) fn visit_ast_stat_for(&mut self, node: *mut AstStatFor) -> bool {
    visit_ast_stat_for(self, node)
  }
}
