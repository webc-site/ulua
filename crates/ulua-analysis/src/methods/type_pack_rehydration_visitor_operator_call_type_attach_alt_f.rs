use ulua_ast::records::{
  ast_name::AstName, ast_type_pack::AstTypePack, ast_type_pack_generic::AstTypePackGeneric,
  location::Location,
};

use crate::records::{
  free_type_pack::FreeTypePack, type_pack_rehydration_visitor::TypePackRehydrationVisitor,
};

impl TypePackRehydrationVisitor {
  #[inline]
  pub fn operator_call_4(&self, _gtp: &FreeTypePack) -> *mut AstTypePack {
    let allocator = unsafe { &mut *self.allocator };
    let name = AstName {
      value: c"free".as_ptr(),
    };
    let generic = AstTypePackGeneric::new(Location::default(), name);
    allocator.alloc(generic) as *mut AstTypePack
  }
}
