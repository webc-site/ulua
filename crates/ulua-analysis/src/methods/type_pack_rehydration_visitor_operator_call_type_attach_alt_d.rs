use core::ptr::null_mut;

use ulua_ast::records::{
  ast_type_pack::AstTypePack, ast_type_pack_variadic::AstTypePackVariadic, location::Location,
};

use crate::records::{
  type_pack_rehydration_visitor::TypePackRehydrationVisitor, variadic_type_pack::VariadicTypePack,
};
impl TypePackRehydrationVisitor {
  #[inline]
  pub fn operator_call_8(&self, vtp: &VariadicTypePack) -> *mut AstTypePack {
    if vtp.hidden {
      return null_mut();
    }

    // C++ `Luau::visit(*typeVisitor, vtp.ty->ty)`.
    let type_visitor = unsafe { &mut *self.type_visitor };
    let variadic_type = unsafe { type_visitor.visit_type(vtp.ty) };

    let allocator = unsafe { &mut *self.allocator };
    let node = AstTypePackVariadic::new(Location::default(), variadic_type);
    allocator.alloc(node) as *mut AstTypePack
  }
}
