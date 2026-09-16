use ulua_ast::records::location::Location;

use crate::{
  functions::{
    follow_type::follow_type_id, get_type_alt_j::get_type_id, reduce_union::reduce_union,
  },
  records::{free_type::FreeType, type_checker::TypeChecker, union_type::UnionType},
  type_aliases::{scope_ptr_type::ScopePtr, type_id::TypeId},
};

impl TypeChecker {
  pub fn union_of_types(
    &mut self,
    a: TypeId,
    b: TypeId,
    scope: &ScopePtr,
    location: &Location,
    unify_free_types: bool,
  ) -> TypeId {
    let a = follow_type_id(a);
    let b = follow_type_id(b);

    if unify_free_types {
      let is_a_free = !get_type_id::<FreeType>(a).is_none();
      let is_b_free = !get_type_id::<FreeType>(b).is_none();

      if is_a_free || is_b_free {
        if self.unify_type_id_type_id_scope_ptr_location(b, a, scope, location) {
          return a;
        }

        return self.error_recovery_type_type_id(self.any_type);
      }
    }

    if a == b {
      return a;
    }

    let types = reduce_union(&[a, b]);
    if types.is_empty() {
      return self.never_type;
    }

    if types.len() == 1 {
      return types[0];
    }

    self.add_type(&UnionType { options: types })
  }
}
