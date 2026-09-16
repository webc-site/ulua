use ulua_ast::records::{
  ast_array::AstArray, ast_name::AstName, ast_type::AstType, ast_type_reference::AstTypeReference,
  location::Location,
};

use crate::records::{free_type::FreeType, type_rehydration_visitor::TypeRehydrationVisitor};
impl TypeRehydrationVisitor {
  pub fn operator_call_5(&mut self, _ft: &FreeType) -> *mut AstType {
    let allocator = unsafe { &mut *self.allocator };
    let name = AstName::ast_name_c_char(c"free".as_ptr());
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
