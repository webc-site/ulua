use ulua_ast::records::{
  ast_name::AstName, ast_type_pack::AstTypePack, ast_type_pack_generic::AstTypePackGeneric,
  location::Location,
};

use crate::records::{
  blocked_type_pack::BlockedTypePack, type_pack_rehydration_visitor::TypePackRehydrationVisitor,
};

impl TypePackRehydrationVisitor {
  #[inline]
  pub fn operator_call(&self, _btp: &BlockedTypePack) -> *mut AstTypePack {
    let allocator = unsafe { &mut *self.allocator };
    let name = AstName {
      value: c"*blocked*".as_ptr(),
    };
    let generic = AstTypePackGeneric::new(Location::default(), name);
    allocator.alloc(generic) as *mut AstTypePack
  }
}
