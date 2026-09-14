use ulua_ast::records::{
  ast_name::AstName, ast_type_pack::AstTypePack, ast_type_pack_generic::AstTypePackGeneric,
  location::Location,
};

use crate::{
  functions::get_name_type_attach_alt_c::get_name_allocator_synthetic_names_generic_type_pack,
  records::{
    generic_type_pack::GenericTypePack, type_pack_rehydration_visitor::TypePackRehydrationVisitor,
  },
};

impl TypePackRehydrationVisitor {
  #[inline]
  pub fn operator_call_5(&self, gtp: &GenericTypePack) -> *mut AstTypePack {
    let allocator = unsafe { &mut *self.allocator };
    let synthetic_names = unsafe { &mut *self.synthetic_names };
    let name_ptr =
      get_name_allocator_synthetic_names_generic_type_pack(allocator, synthetic_names, gtp);
    let name = { AstName { value: name_ptr } };
    let generic = AstTypePackGeneric::new(Location::default(), name);
    allocator.alloc(generic) as *mut AstTypePack
  }
}
