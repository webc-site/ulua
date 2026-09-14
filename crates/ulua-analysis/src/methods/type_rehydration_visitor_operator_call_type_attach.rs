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
      // CStr::as_ptr 已返回 *const c_char，无需再强转
      Type::NilType => AstName::ast_name_c_char(c"nil".as_ptr()),
      Type::Boolean => AstName::ast_name_c_char(c"boolean".as_ptr()),
      Type::Number => AstName::ast_name_c_char(c"number".as_ptr()),
      Type::Integer => AstName::ast_name_c_char(c"integer".as_ptr()),
      Type::String => AstName::ast_name_c_char(c"string".as_ptr()),
      Type::Thread => AstName::ast_name_c_char(c"thread".as_ptr()),
      Type::Buffer => AstName::ast_name_c_char(c"buffer".as_ptr()),
      Type::Function => AstName::ast_name_c_char(c"function".as_ptr()),
      Type::Table => AstName::ast_name_c_char(c"table".as_ptr()),
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
