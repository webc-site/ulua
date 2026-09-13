use core::ffi::c_char;

use ulua_ast::records::{
  ast_array::AstArray, ast_name::AstName, ast_type::AstType, ast_type_reference::AstTypeReference,
  location::Location,
};

use crate::records::{
  type_function_instance_type::TypeFunctionInstanceType,
  type_rehydration_visitor::TypeRehydrationVisitor,
};
impl TypeRehydrationVisitor {
  pub fn operator_call_18(&mut self, tfit: &TypeFunctionInstanceType) -> *mut AstType {
    let allocator = unsafe { &mut *self.allocator };
    let name = tfit.user_func_name.unwrap_or_else(|| {
      let func = unsafe { &*tfit.function.as_ptr() };
      AstName {
        value: func.name.as_ptr() as *const c_char,
      }
    });
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
