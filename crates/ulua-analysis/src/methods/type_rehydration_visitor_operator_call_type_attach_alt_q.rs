use ulua_ast::records::{
  allocator::Allocator, ast_array::AstArray, ast_name::AstName, ast_type::AstType,
  ast_type_reference::AstTypeReference, location::Location,
};

use crate::{
  records::{lazy_type::LazyType, type_rehydration_visitor::TypeRehydrationVisitor},
  type_aliases::type_id::TypeId,
};
impl TypeRehydrationVisitor {
  #[inline]
  pub fn operator_call_9(&mut self, ltv: &LazyType) -> *mut AstType {
    // C++ `if (TypeId unwrapped = ltv.unwrapped.load()) return Luau::visit(*this, unwrapped->ty);`
    let unwrapped: TypeId = ltv.unwrapped;
    if !unwrapped.is_null() {
      return unsafe { self.visit_type(unwrapped) };
    }

    let allocator: &mut Allocator = unsafe { &mut *self.allocator };
    let name = AstName::ast_name_c_char(c"<Lazy?>".as_ptr());
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
