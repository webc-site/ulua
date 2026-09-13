use core::ffi::c_char;

use ulua_ast::records::{
  ast_name::AstName, ast_type_pack::AstTypePack, ast_type_pack_generic::AstTypePackGeneric,
  location::Location,
};

use crate::records::{
  type_function_instance_type_pack::TypeFunctionInstanceTypePack,
  type_pack_rehydration_visitor::TypePackRehydrationVisitor,
};
impl TypePackRehydrationVisitor {
  #[inline]
  pub fn operator_call_6(&self, tfitp: &TypeFunctionInstanceTypePack) -> *mut AstTypePack {
    let allocator = unsafe { &mut *self.allocator };
    let name = AstName {
      value: unsafe { (*tfitp.function).name.as_ptr() as *const c_char },
    };
    let generic = AstTypePackGeneric::new(Location::default(), name);
    allocator.alloc(generic) as *mut AstTypePack
  }
}
