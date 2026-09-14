use ulua_ast::records::{
  allocator::Allocator, ast_array::AstArray, ast_name::AstName, ast_type::AstType,
  ast_type_reference::AstTypeReference, location::Location,
};

use crate::records::{blocked_type::BlockedType, type_rehydration_visitor::TypeRehydrationVisitor};
impl TypeRehydrationVisitor {
  #[inline]
  pub fn operator_call_2(&mut self, _btv: &BlockedType) -> *mut AstType {
    let allocator: &mut Allocator = unsafe { &mut *self.allocator };
    let name = AstName {
      value: c"*blocked*".as_ptr(),
    };
    let reference = AstTypeReference::new(
      Location::default(),
      None,
      name,
      None,
      Location::default(),
      false,
      AstArray::default(),
    );
    allocator.alloc(reference) as *mut AstType
  }
}
