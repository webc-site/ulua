use crate::{
  functions::{is_subclass_normalize::is_subclass_type_id_type_id, is_top::is_top},
  records::{normalized_extern_type::NormalizedExternType, normalizer::Normalizer},
};

impl Normalizer {
  pub fn intersect_extern_types(
    &mut self,
    heres: &mut NormalizedExternType,
    theres: &NormalizedExternType,
  ) {
    self.consume_fuel();

    if theres.is_never() {
      heres.reset_to_never();
      return;
    } else if is_top(unsafe { &*self.builtin_types }, theres) {
      return;
    }

    for &there_ty in &theres.ordering {
      let there_negations = theres.extern_types.get(&there_ty).unwrap().clone();

      let mut idx = 0;
      while idx < heres.ordering.len() {
        let here_ty = heres.ordering[idx];

        if is_subclass_type_id_type_id(there_ty, here_ty) {
          let mut negations = heres.extern_types.remove(&here_ty).unwrap_or_default();

          for n_ty in negations.order.clone() {
            if !is_subclass_type_id_type_id(n_ty, there_ty) {
              negations.erase_type_id(n_ty);
            }
          }

          self.union_extern_types_type_ids_type_ids(&mut negations, &there_negations);

          heres.ordering.remove(idx);
          heres.push_pair(there_ty, negations);
          break;
        } else if is_subclass_type_id_type_id(here_ty, there_ty) {
          let mut negations = there_negations.clone();

          let mut erased_here = false;
          for n_ty in negations.order.clone() {
            if is_subclass_type_id_type_id(here_ty, n_ty) {
              heres.extern_types.remove(&here_ty);
              heres.ordering.remove(idx);
              erased_here = true;
              break;
            }

            if !is_subclass_type_id_type_id(n_ty, here_ty) {
              negations.erase_type_id(n_ty);
            }
          }

          if !erased_here {
            if let Some(mut here_negations) = heres.extern_types.remove(&here_ty) {
              self.union_extern_types_type_ids_type_ids(&mut here_negations, &negations);
              heres.extern_types.insert(here_ty, here_negations);
            }
            idx += 1;
          }
        } else if here_ty == there_ty {
          if let Some(mut here_negations) = heres.extern_types.remove(&here_ty) {
            self.union_extern_types_type_ids_type_ids(&mut here_negations, &there_negations);
            heres.extern_types.insert(here_ty, here_negations);
          }
          break;
        } else {
          heres.ordering.remove(idx);
          heres.extern_types.remove(&here_ty);
        }
      }
    }
  }
}
