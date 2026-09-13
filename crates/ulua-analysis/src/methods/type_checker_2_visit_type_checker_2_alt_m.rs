//! `TypeChecker2::visit(AstStatAssign*)`（TypeChecker2.cpp:1218-1265）。
use ulua_ast::records::ast_stat_assign::AstStatAssign;

use crate::{
  enums::value_context::ValueContext,
  functions::get_type_alt_j::get_type_id,
  records::{never_type::NeverType, type_checker_2::TypeChecker2},
};

impl TypeChecker2 {
  pub fn visit_ast_stat_assign(&mut self, assign: &AstStatAssign) {
    for (&lhs, &rhs) in assign.vars.as_slice().iter().zip(assign.values.as_slice()) {
      self.visit_ast_expr_value_context(lhs, ValueContext::LValue);
      let lhs_type = self.lookup_type(unsafe { &*lhs });

      self.visit_ast_expr_value_context(rhs, ValueContext::RValue);
      let rhs_type = self.lookup_type(unsafe { &*rhs });

      if get_type_id::<NeverType>(lhs_type).is_some() {
        self.report_errors_from_assigning_to_never(unsafe { &*lhs }, rhs_type);
        continue;
      }

      // SAFETY: lhs 来自 AST arena，与 visit 树同寿。
      if unsafe { self.test_literal_or_ast_type_is_subtype(rhs, lhs_type) }
        && let Some(binding_type) = self.get_binding_type(unsafe { &mut *lhs })
      {
        unsafe { self.test_literal_or_ast_type_is_subtype(rhs, binding_type) };
      }
    }
  }
}
