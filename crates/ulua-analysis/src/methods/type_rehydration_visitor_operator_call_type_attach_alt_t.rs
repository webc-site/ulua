use std::{mem::size_of, ptr::null_mut};

use ulua_ast::records::{
  allocator::Allocator, ast_array::AstArray, ast_name::AstName, ast_type::AstType,
  ast_type_or_pack::AstTypeOrPack, ast_type_reference::AstTypeReference, location::Location,
};

use crate::records::{
  negation_type::NegationType, type_rehydration_visitor::TypeRehydrationVisitor,
};
impl TypeRehydrationVisitor {
  #[inline]
  pub fn operator_call_11(&mut self, ntv: &NegationType) -> *mut AstType {
    // C++ `params.data[0] = AstTypeOrPack{Luau::visit(*this, ntv.ty->ty), nullptr};`
    let ty_rehydrated = unsafe { self.visit_type(ntv.ty) };

    let allocator: &mut Allocator = unsafe { &mut *self.allocator };

    let params_data: *mut AstTypeOrPack =
      allocator.allocate(size_of::<AstTypeOrPack>()) as *mut AstTypeOrPack;
    unsafe {
      *params_data = AstTypeOrPack {
        r#type: ty_rehydrated,
        type_pack: null_mut(),
      };
    }

    let params = AstArray {
      data: params_data,
      size: 1,
    };

    let reference = AstTypeReference::new(
      Location::default(),
      None,
      AstName::ast_name_c_char(c"negate".as_ptr()),
      None,
      Location::default(),
      true,
      params,
    );

    allocator.alloc(reference) as *mut AstType
  }
}
