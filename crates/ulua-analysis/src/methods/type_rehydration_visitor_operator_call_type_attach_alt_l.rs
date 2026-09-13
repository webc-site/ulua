use ulua_ast::records::{
  allocator::Allocator, ast_array::AstArray, ast_name::AstName, ast_type::AstType,
  ast_type_reference::AstTypeReference, location::Location,
};

use crate::{
  functions::get_name_type_attach::get_name_allocator_synthetic_names_generic_type,
  records::{generic_type::GenericType, type_rehydration_visitor::TypeRehydrationVisitor},
  type_aliases::synthetic_names::SyntheticNames,
};
impl TypeRehydrationVisitor {
  #[inline]
  pub fn operator_call_7(&mut self, gtv: &GenericType) -> *mut AstType {
    let allocator: &mut Allocator = unsafe { &mut *self.allocator };
    let synthetic_names: &mut SyntheticNames = unsafe { &mut *self.synthetic_names };
    let name_ptr = get_name_allocator_synthetic_names_generic_type(allocator, synthetic_names, gtv);
    let name = AstName::ast_name_c_char(name_ptr);
    let type_ref = AstTypeReference::new(
      Location::default(),
      None,
      name,
      None,
      Location::default(),
      false,
      AstArray::default(),
    );
    allocator.alloc(type_ref) as *mut AstType
  }
}
