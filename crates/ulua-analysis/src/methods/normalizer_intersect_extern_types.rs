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
    } else if is_top({ self.builtin_types.get() }, theres) {
      return;
    }

    for &there_ty in &theres.ordering {
      // 成对登记不变式经 `negations` 访问器统一保证。
      let there_negations = theres.negations(there_ty).clone();

      let mut idx = 0;
      while idx < heres.ordering.len() {
        let here_ty = heres.ordering[idx];

        if is_subclass_type_id_type_id(there_ty, here_ty) {
          // 成对移除该 cluster 并取其 negations，修剪合并后以 `there_ty` 重新登记。
          let mut negations = heres.remove_cluster_at(idx);

          for n_ty in negations.order.clone() {
            if !is_subclass_type_id_type_id(n_ty, there_ty) {
              negations.erase_type_id(n_ty);
            }
          }

          self.union_extern_types_type_ids_type_ids(&mut negations, &there_negations);

          heres.push_pair(there_ty, negations);
          break;
        } else if is_subclass_type_id_type_id(here_ty, there_ty) {
          let mut negations = there_negations.clone();

          let mut erased_here = false;
          for n_ty in negations.order.clone() {
            if is_subclass_type_id_type_id(here_ty, n_ty) {
              heres.remove_cluster_at(idx);
              erased_here = true;
              break;
            }

            if !is_subclass_type_id_type_id(n_ty, here_ty) {
              negations.erase_type_id(n_ty);
            }
          }

          if !erased_here {
            if let Some(here_negations) = heres.extern_types.get_mut(&here_ty) {
              self.union_extern_types_type_ids_type_ids(here_negations, &negations);
            }
            idx += 1;
          }
        } else if here_ty == there_ty {
          if let Some(here_negations) = heres.extern_types.get_mut(&here_ty) {
            self.union_extern_types_type_ids_type_ids(here_negations, &there_negations);
          }
          break;
        } else {
          heres.remove_cluster_at(idx);
        }
      }
    }
  }
}
