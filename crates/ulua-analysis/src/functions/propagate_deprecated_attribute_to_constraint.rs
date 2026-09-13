//! Node: `cxx:Function:Luau.Analysis:Analysis/src/ConstraintGenerator.cpp:1720:propagate_deprecated_attribute_to_constraint`
//! Source: `Analysis/src/ConstraintGenerator.cpp` (ConstraintGenerator.cpp:1720-1731, hand-ported)

use ulua_ast::records::{ast_attr::AstAttrType, ast_expr_function::AstExprFunction};

use crate::{
  records::generalization_constraint::GeneralizationConstraint,
  type_aliases::constraint_v::{ConstraintV, ConstraintVMember},
};

/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn propagate_deprecated_attribute_to_constraint(
  c: &mut ConstraintV,
  func: *const AstExprFunction,
) {
  if let Some(gen_constraint) = GeneralizationConstraint::get_if_mut(c) {
    let deprecated_attribute = unsafe { (*func).get_attribute(AstAttrType::Deprecated) };
    gen_constraint.has_deprecated_attribute = !deprecated_attribute.is_null();
    if !deprecated_attribute.is_null() {
      gen_constraint.deprecated_info = unsafe { (*deprecated_attribute).deprecated_info() };
    }
  }
}
