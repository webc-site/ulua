use core::{ffi::c_char, ptr::null_mut};

use ulua_ast::records::{
  ast_array::AstArray, ast_type::AstType, ast_type_singleton_bool::AstTypeSingletonBool,
  ast_type_singleton_string::AstTypeSingletonString, location::Location,
};

use crate::{
  functions::get_singleton_type::get_singleton_type,
  records::{
    boolean_singleton::BooleanSingleton, singleton_type::SingletonType,
    string_singleton::StringSingleton, type_rehydration_visitor::TypeRehydrationVisitor,
  },
};
impl TypeRehydrationVisitor {
  pub fn operator_call_16(&mut self, stv: &SingletonType) -> *mut AstType {
    if let Some(bs) = get_singleton_type::<BooleanSingleton>(stv) {
      let location = Location::default();
      let allocator = unsafe { &mut *self.allocator };
      return allocator.alloc(AstTypeSingletonBool::new(location, bs.value)) as *mut AstType;
    }
    if let Some(ss) = get_singleton_type::<StringSingleton>(stv) {
      let location = Location::default();
      let value = {
        let s = &ss.value;
        let data = s.as_ptr() as *mut c_char;
        let size = s.len();
        AstArray { data, size }
      };
      let allocator = unsafe { &mut *self.allocator };
      return allocator.alloc(AstTypeSingletonString::new(location, value)) as *mut AstType;
    }
    null_mut()
  }
}
