//! Source: `Analysis/src/ConstraintGenerator.cpp` (ConstraintGenerator.cpp:1720-1731, hand-ported)

use ulua_ast::records::{ast_attr::AstAttrType, ast_expr_function::AstExprFunction};

use crate::{
  records::{arena_handle::alias_ref, generalization_constraint::GeneralizationConstraint},
  type_aliases::constraint_v::{ConstraintV, ConstraintVMember},
};

pub fn propagate_deprecated_attribute_to_constraint(c: &mut ConstraintV, func: &AstExprFunction) {
  if let Some(gen_constraint) = GeneralizationConstraint::get_if_mut(c) {
    let deprecated_attribute = func.get_attribute(AstAttrType::Deprecated);
    gen_constraint.has_deprecated_attribute = !deprecated_attribute.is_null();
    if !deprecated_attribute.is_null() {
      gen_constraint.deprecated_info = alias_ref(deprecated_attribute).deprecated_info();
    }
  }
}
