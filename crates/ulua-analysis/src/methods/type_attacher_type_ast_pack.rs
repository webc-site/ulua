use core::mem::size_of;

use ulua_ast::records::{ast_array::AstArray, ast_type::AstType};

use crate::{
  functions::flatten_type_pack::flatten_type_pack_id,
  records::{
    type_attacher::TypeAttacher, type_rehydration_options::TypeRehydrationOptions,
    type_rehydration_visitor::TypeRehydrationVisitor,
  },
  type_aliases::{synthetic_names::SyntheticNames, type_pack_id::TypePackId},
};
impl TypeAttacher {
  pub fn type_ast_pack(&mut self, r#type: TypePackId) -> AstArray<*mut AstType> {
    let (v, _tail) = flatten_type_pack_id(r#type);

    let size = v.len();
    let data =
      unsafe { (*self.allocator).allocate(size * size_of::<*mut AstType>()) as *mut *mut AstType };

    for (index, item) in v.iter().enumerate() {
      // C++ `result.data[i] = Luau::visit(TypeRehydrationVisitor(allocator, &synthetic_names), v[i]->ty);`
      let mut rehydrator =
        TypeRehydrationVisitor::type_rehydration_visitor_type_rehydration_visitor(
          self.allocator,
          &mut self.synthetic_names as *mut SyntheticNames,
          &TypeRehydrationOptions::default(),
        );
      let ast_type = unsafe { rehydrator.visit_type(*item) };
      unsafe {
        *data.add(index) = ast_type;
      }
    }

    AstArray { data, size }
  }
}
