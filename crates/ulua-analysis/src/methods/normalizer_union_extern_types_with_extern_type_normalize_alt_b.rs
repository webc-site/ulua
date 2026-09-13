use crate::{
  functions::is_subclass_normalize::is_subclass_type_id_type_id,
  records::{
    normalized_extern_type::NormalizedExternType, normalizer::Normalizer, type_ids::TypeIds,
  },
  type_aliases::type_id::TypeId,
};

impl Normalizer {
  pub fn union_extern_types_with_extern_type_normalized_extern_type_type_id(
    &mut self,
    heres: &mut NormalizedExternType,
    there: TypeId,
  ) {
    self.consume_fuel();

    let mut idx = 0;
    while idx < heres.ordering.len() {
      let here_ty = heres.ordering[idx];
      if let Some(mut here_negations) = heres.extern_types.get(&here_ty).cloned() {
        // If the incoming class is a subclass of another class in the map, we
        // must ensure that it is negated by one of the negations in the same
        // cluster. If it isn't, we do not need to insert it - the subtyping
        // relationship is already handled by this entry. If it is, we must
        // insert it, to capture the presence of this particular subtype.
        if is_subclass_type_id_type_id(there, here_ty) {
          let mut n_idx = 0;
          while n_idx < here_negations.order.len() {
            let here_negation = here_negations.order[n_idx];

            // If the incoming class is a subclass of one of the negations,
            // we must insert it into the class map.
            if is_subclass_type_id_type_id(there, here_negation) {
              heres.push_pair(there, TypeIds::new());
              return;
            }
            // If the incoming class is a superclass of one of the
            // negations, then the negation no longer applies and must be
            // removed. This is also true if they are equal. Since extern types
            // are, at this time, entirely persistent (we do not clone
            // them), a pointer identity check is sufficient.
            else if is_subclass_type_id_type_id(here_negation, there) {
              // erase shifts order; keep index
              here_negations.erase_type_id(here_negation);
              continue;
            }

            // If the incoming class is unrelated to the negation, we move
            // on to the next item.
            n_idx += 1;
          }

          // If, at the end of the above loop, we haven't returned, that means
          // that the class is not a subclass of one of the negations, and is
          // covered by the existing subtype relationship. We can return now.
          return;
        }
        // If the incoming class is a superclass of another class in the map, we
        // need to replace the existing class with the incoming class,
        // preserving the relevant negations.
        else if is_subclass_type_id_type_id(here_ty, there) {
          let negations = here_negations;
          heres.ordering.remove(idx);
          heres.extern_types.remove(&here_ty);

          heres.push_pair(there, negations);
          return;
        }

        // If the incoming class is unrelated to the class in the map, we move
        // on. If we do not otherwise exit from this method body, we will
        // eventually fall out of this loop and insert the incoming class, which
        // we have proven to be completely unrelated to any class in the map,
        // into the map itself.
      }

      idx += 1;
    }

    heres.push_pair(there, TypeIds::new());
  }
}
