use core::ffi::c_char;

use ulua_ast::records::{
  ast_array::AstArray, ast_name::AstName, ast_type::AstType, ast_type_reference::AstTypeReference,
  location::Location,
};

use crate::records::{
  primitive_type::{PrimitiveType, Type},
  type_rehydration_visitor::TypeRehydrationVisitor,
};
impl TypeRehydrationVisitor {
  /// C++ `AstType* operator()(const PrimitiveType& ptv)`.
  pub fn operator_call_15(&mut self, ptv: &PrimitiveType) -> *mut AstType {
    let allocator = unsafe { &mut *self.allocator };
    let location = Location::default();

    let name = match ptv.r#type {
      Type::NilType => AstName::ast_name_c_char(c"nil".as_ptr() as *const c_char),
      Type::Boolean => AstName::ast_name_c_char(c"boolean".as_ptr() as *const c_char),
      Type::Number => AstName::ast_name_c_char(c"number".as_ptr() as *const c_char),
      Type::Integer => AstName::ast_name_c_char(c"integer".as_ptr() as *const c_char),
      Type::String => AstName::ast_name_c_char(c"string".as_ptr() as *const c_char),
      Type::Thread => AstName::ast_name_c_char(c"thread".as_ptr() as *const c_char),
      Type::Buffer => AstName::ast_name_c_char(c"buffer".as_ptr() as *const c_char),
      Type::Function => AstName::ast_name_c_char(c"function".as_ptr() as *const c_char),
      Type::Table => AstName::ast_name_c_char(c"table".as_ptr() as *const c_char),
    };

    let result = allocator.alloc(AstTypeReference::new(
      location,
      None,
      name,
      None,
      location,
      false,
      AstArray::default(),
    ));

    result as *mut AstType
  }
}
