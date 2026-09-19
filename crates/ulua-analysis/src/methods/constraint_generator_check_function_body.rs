use ulua_ast::records::ast_expr_function::AstExprFunction;

use crate::{
  enums::control_flow::ControlFlow,
  records::{
    constraint_generator::ConstraintGenerator, pack_subtype_constraint::PackSubtypeConstraint,
  },
  type_aliases::{constraint_v::ConstraintV, scope_ptr_type::ScopePtr},
};

impl ConstraintGenerator {
  pub fn check_function_body(&mut self, scope: &ScopePtr, fn_expr: &AstExprFunction) {
    let cf = unsafe {
      self.visit_block_without_child_scope(scope.as_ref() as *const _ as *mut _, fn_expr.body)
    };

    if cf == ControlFlow::None {
      let builtin_types = unsafe { &*self.builtin_types };
      let sub_pack = builtin_types.empty_type_pack;
      let super_pack = scope.return_type;

      let constraint = PackSubtypeConstraint {
        sub_pack,
        super_pack,
        returns: true,
      };

      self.add_constraint_scope_ptr_location_constraint_v(
        scope,
        fn_expr.base.base.location,
        ConstraintV::PackSubtype(constraint),
      );
    }
  }
}
