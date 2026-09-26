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
    let cf = self.visit_block_without_child_scope(scope, fn_expr.body.get());

    if cf == ControlFlow::None {
      // Safety: `self.builtin_types.as_ptr()` 是 ConstraintGenerator 构造期接线的 `*mut BuiltinTypes`，
      // 指向比本 generator 长寿的内置类型单例，全程非空存活；此处重建共享借用仅读
      // `empty_type_pack`（Copy 的 TypePackId），`scope.return_type` 亦只读，单线程无别名。
      let builtin_types = self.builtin_types.get();
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
