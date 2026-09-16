use core::mem::size_of;

use ulua_ast::records::{
  ast_array::AstArray, ast_type::AstType, ast_type_intersection::AstTypeIntersection,
  location::Location,
};

use crate::records::{
  intersection_type::IntersectionType, type_rehydration_visitor::TypeRehydrationVisitor,
};
impl TypeRehydrationVisitor {
  pub fn operator_call_8(&mut self, uv: &IntersectionType) -> *mut AstType {
    let size = uv.parts.len();
    let data =
      unsafe { (*self.allocator).allocate(size_of::<*mut AstType>() * size) as *mut *mut AstType };

    for (i, &part_ty) in uv.parts.iter().enumerate() {
      // C++ `intersectionTypes.data[i] = Luau::visit(*this, uv.parts[i]->ty);`
      let ast_part = unsafe { self.visit_type(part_ty) };
      unsafe { *data.add(i) = ast_part };
    }

    let intersection_types = AstArray { data, size };

    let location = Location::default();
    let alloc = unsafe { &mut *self.allocator };
    alloc.alloc(AstTypeIntersection::new(location, intersection_types)) as *mut AstType
  }
}
