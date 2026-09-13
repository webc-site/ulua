use ulua_ast::records::ast_type::AstType;

use crate::{
  records::{bound::Bound, type_rehydration_visitor::TypeRehydrationVisitor},
  type_aliases::type_id::TypeId,
};

impl TypeRehydrationVisitor {
  /// C++ `AstType* operator()(const Unifiable::Bound<TypeId>& bound)` —
  /// `return Luau::visit(*this, bound.bound_to->ty);`.
  pub fn operator_call_19(&mut self, bound: &Bound<TypeId>) -> *mut AstType {
    unsafe { self.visit_type(bound.bound_to) }
  }
}
