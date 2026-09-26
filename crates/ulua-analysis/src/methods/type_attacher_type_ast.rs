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
    // 双写合一：`is_none()` 早退与 `unwrap()` 并为一条 let-else，Some 直接绑定。
    // C++ `return Luau::visit(TypeRehydrationVisitor(allocator, &synthetic_names), (*type)->ty);`
    let Some(ty) = r#type else {
      return null_mut();
    };
    let mut visitor = TypeRehydrationVisitor::type_rehydration_visitor_type_rehydration_visitor(
      self.allocator,
      &mut self.synthetic_names as *mut SyntheticNames,
      &TypeRehydrationOptions::default(),
    );
    visitor.visit_type(ty)
  }
}
