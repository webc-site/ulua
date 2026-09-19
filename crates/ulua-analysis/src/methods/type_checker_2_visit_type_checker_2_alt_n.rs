use ulua_ast::records::{
  ast_expr_binary::AstExprBinary, ast_node::AstNode, ast_stat::AstStat,
  ast_stat_compound_assign::AstStatCompoundAssign,
};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::type_checker_2::TypeChecker2;
impl TypeChecker2 {
  /// # Safety
  /// 调用方须保证 `stat` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_stat_compound_assign(&mut self, stat: *mut AstStatCompoundAssign) {
    unsafe {
      let location = (*stat).base.base.location;
      let op = (*stat).op;
      let var = (*stat).var;
      let value = (*stat).value;

      // C++: AstExprBinary fake{stat->location, stat->op, stat->var, stat->value}; visit(&fake, stat);
      let fake = AstExprBinary::new(location, op, var, value);
      // fake 为本函数栈上构造的临时节点（C++ 同构）。
      self.visit_ast_expr_binary_ast_node(&fake, stat as *mut AstNode);

      let result_ty = (*self.module)
        .ast_compound_assign_result_types
        .find(&(stat as *const AstStat));

      if (*self.module).constraint_generation_did_not_complete && result_ty.is_none() {
        return;
      }

      LUAU_ASSERT!(result_ty.is_some());
      let result_ty = *result_ty.unwrap();
      // SAFETY: var 指向 AST arena 节点。
      let var_ty = self.lookup_type(&*var);

      self.test_is_subtype_type_id_type_id_location(result_ty, var_ty, location);
    }
  }
}
