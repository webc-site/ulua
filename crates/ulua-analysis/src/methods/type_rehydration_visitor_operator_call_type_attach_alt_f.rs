use ulua_ast::records::{
  ast_name::AstName, ast_type::AstType, ast_type_reference::AstTypeReference, location::Location,
};

use crate::records::{
  no_refine_type::NoRefineType, type_rehydration_visitor::TypeRehydrationVisitor,
};

impl TypeRehydrationVisitor {
  #[inline]
  pub fn operator_call_13(&mut self, _no_refine: &NoRefineType) -> *mut AstType {
    let allocator = unsafe { &mut *self.allocator };
    let name = AstName::ast_name_c_char(c"*no-refine*".as_ptr());
    let reference = AstTypeReference::new(
      Location::default(),
      None,
      name,
      None,
      Location::default(),
      false,
      Default::default(),
    );
    allocator.alloc(reference) as *mut AstType
  }
}
