use alloc::vec::Vec;

use ulua_ast::records::ast_stat_return::AstStatReturn;

use crate::{
  enums::control_flow::ControlFlow,
  functions::{begin_type_pack::begin, end_type_pack::end},
  records::{
    constraint_generator::ConstraintGenerator, pack_subtype_constraint::PackSubtypeConstraint,
  },
  type_aliases::{constraint_v::ConstraintV, scope_ptr_type::ScopePtr, type_id::TypeId},
};

impl ConstraintGenerator {
  // ConstraintGenerator::visit(const ScopePtr&, AstStatReturn*)
  // (ConstraintGenerator.cpp:1961).
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn visit_scope_ptr_ast_stat_return(
    &mut self,
    scope: ScopePtr,
    ret: *mut AstStatReturn,
  ) -> ControlFlow {
    // At this point, the only way scope->returnType should have anything
    // interesting in it is if the function has an explicit return annotation.
    // If this is the case, then we can expect that the return expression
    // conforms to that.
    let mut expected_types: Vec<Option<TypeId>> = Vec::new();
    let scope_ref = { scope.as_ref() };
    let return_type = scope_ref.return_type;
    let mut iter = begin(return_type);
    let end_iter = end(return_type);
    while iter.operator_ne(&end_iter) {
      expected_types.push(Some(*iter.operator_deref()));
      iter.operator_inc();
    }

    let list = unsafe { (*ret).list };
    let expr_types = self
      .check_pack_scope_ptr_ast_array_ast_expr_vector_optional_type_id(
        &scope,
        list,
        &expected_types,
      )
      .tp;

    self.add_constraint_scope_ptr_location_constraint_v(
      &scope,
      unsafe { (*ret).base.base.location },
      ConstraintV::PackSubtype(PackSubtypeConstraint {
        sub_pack: expr_types,
        super_pack: return_type,
        returns: true,
      }),
    );

    ControlFlow::Returns
  }
}
