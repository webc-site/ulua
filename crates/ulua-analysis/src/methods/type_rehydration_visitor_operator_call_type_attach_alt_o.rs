use core::mem::size_of;

use ulua_ast::records::{
  ast_array::AstArray, ast_type::AstType, ast_type_union::AstTypeUnion, location::Location,
};

use crate::records::{type_rehydration_visitor::TypeRehydrationVisitor, union_type::UnionType};
impl TypeRehydrationVisitor {
  pub fn operator_call_20(&mut self, uv: &UnionType) -> *mut AstType {
    let size = uv.options.len();
    let data =
      unsafe { (*self.allocator).allocate(size_of::<*mut AstType>() * size) as *mut *mut AstType };

    for (i, &option_ty) in uv.options.iter().enumerate() {
      // C++ `unionTypes.data[i] = Luau::visit(*this, uv.options[i]->ty);`
      let rehydrated = unsafe { self.visit_type(option_ty) };
      unsafe { *data.add(i) = rehydrated };
    }

    let union_types = AstArray { data, size };

    let alloc = unsafe { &mut *self.allocator };
    alloc.alloc(AstTypeUnion::new(Location::default(), union_types)) as *mut AstType
  }
}
