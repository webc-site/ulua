use ulua_ast::records::{
  allocator::Allocator, ast_array::AstArray, ast_name::AstName, ast_type::AstType,
  ast_type_reference::AstTypeReference, location::Location,
};

use crate::records::{any_type::AnyType, type_rehydration_visitor::TypeRehydrationVisitor};
impl TypeRehydrationVisitor {
  #[inline]
  pub fn operator_call(&mut self, _any: &AnyType) -> *mut AstType {
    let allocator: &mut Allocator = unsafe { &mut *self.allocator };
    let _location = Location::default();
    let name = AstName::new();
    let any_type_ref = AstTypeReference::new(
      Location::default(),
      None,
      name,
      None,
      Location::default(),
      false,
      AstArray::default(),
    );
    allocator.alloc(any_type_ref) as *mut AstType
  }
}
