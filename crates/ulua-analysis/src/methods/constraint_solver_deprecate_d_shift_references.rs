use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  functions::{follow_type::follow_type_id, is_reference_counted_type::is_reference_counted_type},
  records::{constraint_solver::ConstraintSolver, type_ids::TypeIds},
  type_aliases::type_id::TypeId,
};
impl ConstraintSolver {
  pub fn deprecate_d_shift_references(&mut self, source: TypeId, target: TypeId) {
    LUAU_ASSERT!(!FFlag::LuauConstraintGraph.get());
    let target = follow_type_id(target);

    // if the target isn't a reference counted type, there's nothing to do.
    // this stops us from keeping unnecessary counts for e.g. primitive types.
    if !is_reference_counted_type(target) {
      return;
    }

    if source == target {
      return;
    }

    if let Some(sourcerefs) = self.deprecated_type_to_constraint_set.get(&source).cloned() {
      for constraint in sourcerefs.iter() {
        // For every constraint that the source might be modified by,
        // add that constraint to the set of constraints the target
        // might be modified by.
        let targetrefs = self
          .deprecated_type_to_constraint_set
          .entry(target)
          .or_default();
        targetrefs.insert(*constraint);

        // Additionally, note that said constraint now may modify the target.
        let (it, _) = self
          .deprecated_constraint_to_mutated_types
          .try_insert(*constraint, TypeIds::new());
        it.insert_type_id(target);
      }
    }
  }
}
