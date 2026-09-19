use ulua_ast::records::{
  ast_array::AstArray, ast_name::AstName, ast_type::AstType, ast_type_reference::AstTypeReference,
  location::Location,
};

use crate::records::{
  pending_expansion_type::PendingExpansionType, type_rehydration_visitor::TypeRehydrationVisitor,
};
impl TypeRehydrationVisitor {
  pub fn operator_call_14(&mut self, _petv: &PendingExpansionType) -> *mut AstType {
    let allocator = unsafe { &mut *self.allocator };
    let name = AstName::ast_name_c_char(c"*pending-expansion*".as_ptr());
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
