use ulua_ast::records::{
  ast_name::AstName, ast_type_pack::AstTypePack, ast_type_pack_generic::AstTypePackGeneric,
  location::Location,
};

use crate::{
  records::type_pack_rehydration_visitor::TypePackRehydrationVisitor,
  type_aliases::error_type_pack::ErrorTypePack,
};

impl TypePackRehydrationVisitor {
  #[inline]
  pub fn operator_call_3(&self, _tp: &ErrorTypePack) -> *mut AstTypePack {
    let allocator = unsafe { &mut *self.allocator };
    let name = AstName {
      value: c"Unifiable<Error>".as_ptr(),
    };
    let generic = AstTypePackGeneric::new(Location::default(), name);
    allocator.alloc(generic) as *mut AstTypePack
  }
}
