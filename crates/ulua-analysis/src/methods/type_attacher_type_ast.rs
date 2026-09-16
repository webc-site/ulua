use core::ptr::null_mut;

use ulua_ast::records::ast_type::AstType;

use crate::{
  records::{
    type_attacher::TypeAttacher, type_rehydration_options::TypeRehydrationOptions,
    type_rehydration_visitor::TypeRehydrationVisitor,
  },
  type_aliases::{synthetic_names::SyntheticNames, type_id::TypeId},
};
impl TypeAttacher {
  pub fn type_ast(&mut self, r#type: Option<TypeId>) -> *mut AstType {
    if r#type.is_none() {
      return null_mut();
    }

    // C++ `return Luau::visit(TypeRehydrationVisitor(allocator, &synthetic_names), (*type)->ty);`
    let ty = r#type.unwrap();
    let mut visitor = TypeRehydrationVisitor::type_rehydration_visitor_type_rehydration_visitor(
      self.allocator,
      &mut self.synthetic_names as *mut SyntheticNames,
      &TypeRehydrationOptions::default(),
    );
    unsafe { visitor.visit_type(ty) }
  }
}
