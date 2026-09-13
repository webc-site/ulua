use ulua_ast::records::ast_type::AstType;

use crate::records::{
  metatable_type::MetatableType, type_rehydration_visitor::TypeRehydrationVisitor,
};

impl TypeRehydrationVisitor {
  /// C++ `AstType* operator()(const MetatableType& mtv)` —
  /// `return Luau::visit(*this, mtv.table->ty);`.
  pub fn operator_call_10(&mut self, mtv: &MetatableType) -> *mut AstType {
    unsafe { self.visit_type(mtv.table()) }
  }
}
