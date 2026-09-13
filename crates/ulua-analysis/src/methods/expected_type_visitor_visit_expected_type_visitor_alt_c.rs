use ulua_ast::records::ast_stat_compound_assign::AstStatCompoundAssign;

use crate::records::expected_type_visitor::ExpectedTypeVisitor;

impl ExpectedTypeVisitor {
  pub(crate) fn visit_ast_stat_compound_assign(
    &mut self,
    stat: *mut AstStatCompoundAssign,
  ) -> bool {
    unsafe {
      let var = (*stat).var;
      let lhs_type = (*self.ast_types).find(&(var as *const _));
      if let Some(lhs_type) = lhs_type {
        self.apply_expected_type(*lhs_type, (*stat).value);
      }
    }
    true
  }
}
