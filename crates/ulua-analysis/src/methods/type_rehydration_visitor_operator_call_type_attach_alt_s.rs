use ulua_ast::records::{
  ast_array::AstArray, ast_name::AstName, ast_type::AstType, ast_type_reference::AstTypeReference,
  location::Location,
};

use crate::records::{never_type::NeverType, type_rehydration_visitor::TypeRehydrationVisitor};
impl TypeRehydrationVisitor {
  #[inline]
  pub fn operator_call_12(&mut self, _ttv: &NeverType) -> *mut AstType {
    let allocator = unsafe { &mut *self.allocator };
    let name = AstName::ast_name_c_char(c"never".as_ptr());
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
