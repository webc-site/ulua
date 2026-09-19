use core::ptr::null_mut;

use ulua_ast::records::{
  ast_array::AstArray, ast_name::AstName, ast_type::AstType, ast_type_reference::AstTypeReference,
  location::Location,
};

use crate::records::{type_rehydration_visitor::TypeRehydrationVisitor, unknown_type::UnknownType};
impl TypeRehydrationVisitor {
  pub fn operator_call_21(&mut self, _ttv: &UnknownType) -> *mut AstType {
    let allocator = unsafe { &mut *self.allocator };
    let name = AstName::ast_name_c_char(c"unknown".as_ptr());
    let loc = Location::default();
    let empty_params = AstArray {
      data: null_mut(),
      size: 0,
    };
    let reference = AstTypeReference::new(loc, None, name, None, loc, false, empty_params);
    allocator.alloc(reference) as *mut AstType
  }
}
