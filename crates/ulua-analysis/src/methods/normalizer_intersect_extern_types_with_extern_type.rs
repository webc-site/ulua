use crate::{
  functions::is_subclass_normalize::is_subclass_type_id_type_id,
  records::{
    normalized_extern_type::NormalizedExternType, normalizer::Normalizer, type_ids::TypeIds,
  },
  type_aliases::type_id::TypeId,
};

impl Normalizer {
  pub fn intersect_extern_types_with_extern_type(
    &mut self,
    heres: &mut NormalizedExternType,
    there: TypeId,
  ) {
    self.consume_fuel();

    let mut idx = 0;
    while idx < heres.ordering.len() {
      let here_ty = heres.ordering[idx];
      let here_negations: TypeIds = heres.extern_types.get(&here_ty).cloned().unwrap();

      // If the incoming class is _the_ current class, skip it.
      if here_ty == there {
        idx += 1;
        continue;
      }
      // If the incoming class is a subclass of this type, replace the
      // current class with the incoming class (preserving/dropping negations).
      else if is_subclass_type_id_type_id(there, here_ty) {
        let mut negations = here_negations;
        let mut empty_intersect_with_negation = false;

        let mut n_idx = 0;
        while n_idx < negations.order.len() {
          let n_ty = negations.order[n_idx];
          if is_subclass_type_id_type_id(there, n_ty) {
            empty_intersect_with_negation = true;
            break;
          }

          if !is_subclass_type_id_type_id(n_ty, there) {
            negations.erase_type_id(n_ty);
            // Erasing shifts order; keep index.
            continue;
          }

          n_idx += 1;
        }

        // Remove this type from the ordering and map.
        heres.ordering.remove(idx);
        heres.extern_types.remove(&here_ty);

        if !empty_intersect_with_negation {
          heres.push_pair(there, negations);
        }
        break;
      }
      // If incoming is a superclass of the current class, don't insert it.
      else if is_subclass_type_id_type_id(here_ty, there) {
        return;
      }
      // Completely unrelated: drop current class.
      else {
        heres.ordering.remove(idx);
        heres.extern_types.remove(&here_ty);
        continue;
      }
    }
  }
}
