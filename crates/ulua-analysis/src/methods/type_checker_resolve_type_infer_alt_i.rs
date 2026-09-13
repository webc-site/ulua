use crate::{
  functions::{
    follow_type::follow_type_id, get_type_alt_j::get_type_id, is_nil::is_nil,
    is_undecidable::is_undecidable, maybe_singleton::maybe_singleton,
  },
  records::{
    eq_predicate::EqPredicate, singleton_type::SingletonType, type_checker::TypeChecker,
    union_type::UnionType,
  },
  type_aliases::{refinement_map::RefinementMap, scope_ptr_type::ScopePtr, type_id::TypeId},
};

impl TypeChecker {
  pub fn resolve_eq_predicate_refinement_map_scope_ptr_bool(
    &mut self,
    eq_p: &EqPredicate,
    refis: &mut RefinementMap,
    scope: ScopePtr,
    sense: bool,
  ) {
    let followed = follow_type_id(eq_p.ty);
    let rhs = match get_type_id::<UnionType>(followed) {
      Some(union) => union.options.clone(),
      None => alloc::vec![followed],
    };

    if sense && rhs.iter().copied().any(is_undecidable) {
      return;
    }

    let checker = self as *mut TypeChecker;
    let eq_ty = eq_p.ty;
    let location = eq_p.location;
    let scope_for_predicate = scope.clone();

    let predicate = Box::new(move |option: TypeId| -> Option<TypeId> {
      if !sense && is_nil(eq_ty) {
        return if is_undecidable(option) || !is_nil(option) {
          Some(option)
        } else {
          None
        };
      }

      if maybe_singleton(eq_ty) {
        let option_is_subtype = unsafe {
          (*checker)
            .can_unify_type_id_type_id_scope_ptr_location(
              option,
              eq_ty,
              &scope_for_predicate,
              &location,
            )
            .is_empty()
        };
        let target_is_subtype = unsafe {
          (*checker)
            .can_unify_type_id_type_id_scope_ptr_location(
              eq_ty,
              option,
              &scope_for_predicate,
              &location,
            )
            .is_empty()
        };

        if sense {
          if option_is_subtype && !target_is_subtype {
            return Some(option);
          } else if !option_is_subtype && target_is_subtype {
            return Some(follow_type_id(eq_ty));
          } else if !option_is_subtype && !target_is_subtype {
            return None;
          } else if option_is_subtype && target_is_subtype {
            return Some(follow_type_id(eq_ty));
          }
        } else {
          let is_option_singleton = get_type_id::<SingletonType>(option).is_some();
          if !is_option_singleton {
            return Some(option);
          } else if option_is_subtype && target_is_subtype {
            return None;
          }
        }
      }

      Some(option)
    });

    self.refine_l_value(&eq_p.lvalue, refis, scope, predicate);
  }
}
