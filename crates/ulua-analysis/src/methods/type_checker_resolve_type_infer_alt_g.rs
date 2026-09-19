use alloc::boxed::Box;

use crate::{
  enums::table_state::TableState,
  functions::{get_type_alt_j::get_type_id, is_undecidable::is_undecidable},
  records::{is_a_predicate::IsAPredicate, table_type::TableType, type_checker::TypeChecker},
  type_aliases::{
    refinement_map::RefinementMap, scope_ptr_type::ScopePtr, type_id::TypeId,
    type_id_predicate::TypeIdPredicate,
  },
};
impl TypeChecker {
  /// C++ `TypeChecker::resolve(const IsAPredicate&, ...)` (TypeInfer.cpp:6459).
  pub fn resolve_is_a_predicate_refinement_map_scope_ptr_bool(
    &mut self,
    isa_p: &IsAPredicate,
    refis: &mut RefinementMap,
    scope: ScopePtr,
    sense: bool,
  ) {
    // The predicate filter calls back into `canUnify`, which mutates the
    // type checker. C++ captures `this` by reference; we mirror that with a
    // raw pointer (the codebase's established reentrancy idiom).
    let self_ptr = self as *mut TypeChecker;
    let isa_ty = isa_p.ty;
    let location = isa_p.location;
    let scope_for_predicate = scope.clone();

    let predicate: TypeIdPredicate = Box::new(move |option: TypeId| -> Option<TypeId> {
      let tc = unsafe { &mut *self_ptr };

      // This by itself is not truly enough to determine that A is stronger than B or vice versa.
      let option_is_subtype = tc
        .can_unify_type_infer(option, isa_ty, &scope_for_predicate, &location)
        .is_empty();
      let target_is_subtype = tc
        .can_unify_type_infer(isa_ty, option, &scope_for_predicate, &location)
        .is_empty();

      // If A is a superset of B, then if sense is true, we promote A to B, otherwise we keep A.
      if !option_is_subtype && target_is_subtype {
        return if sense { Some(isa_ty) } else { Some(option) };
      }

      // If A is a subset of B, then if sense is true we pick A, otherwise we eliminate A.
      if option_is_subtype && !target_is_subtype {
        return if sense { Some(option) } else { None };
      }

      // If neither has any relationship, we only return A if sense is false.
      if !option_is_subtype && !target_is_subtype {
        return if sense { None } else { Some(option) };
      }

      // If both are subtypes, then we're in one of the two situations described in
      // the C++ source. We look at whether the option is undecidable or a free table.
      if option_is_subtype && target_is_subtype {
        let is_free_table =
          get_type_id::<TableType>(option).is_some_and(|ttv| ttv.state == TableState::Free);
        if is_undecidable(option) || is_free_table {
          return if sense { Some(isa_ty) } else { Some(option) };
        }

        if sense {
          return Some(isa_ty);
        }
      }

      None
    });

    self.refine_l_value(&isa_p.lvalue, refis, scope, predicate);
  }
}
