use alloc::vec::Vec;

use ulua_ast::records::location::Location;

use crate::{
  functions::{follow_type::follow, get_type, simplify_union::simplify_union},
  records::{
    constraint_generator::ConstraintGenerator, never_type::NeverType, scope::Scope,
    union_builder::UnionBuilder, union_type::UnionType,
  },
  type_aliases::type_id::TypeId,
};

impl ConstraintGenerator {
  pub(crate) fn make_union_scope_ptr_location_type_id_type_id(
    &mut self,
    _scope: *mut Scope,
    _location: Location,
    lhs: TypeId,
    rhs: TypeId,
  ) -> TypeId {
    // 对照 C++ makeUnion：`if (get<NeverType>(follow(lhs))) return rhs;`
    if get_type::get::<NeverType>(follow(lhs)).is_some() {
      return rhs;
    }

    if get_type::get::<NeverType>(follow(rhs)).is_some() {
      return lhs;
    }

    let result = simplify_union(self.builtin_types, self.arena, lhs, rhs).result;

    if get_type::get::<UnionType>(follow(result)).is_some() {
      self.unions_to_simplify.push(result);
    }

    result
  }

  pub fn make_union_vector_type_id(&mut self, options: Vec<TypeId>) -> TypeId {
    let mut ub = UnionBuilder::new(self.arena, self.builtin_types);
    ub.reserve(options.len());

    for option in options {
      ub.add(option);
    }

    let union_ty = ub.build();

    if get_type::get::<UnionType>(union_ty).is_some() {
      self.unions_to_simplify.push(union_ty);
    }

    union_ty
  }
}
