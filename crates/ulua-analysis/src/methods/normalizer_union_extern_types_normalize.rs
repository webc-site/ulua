use ulua_common::fflag;

use crate::{
  functions::is_subclass_normalize::is_subclass_type_id_type_id,
  records::{
    normalized_extern_type::NormalizedExternType, normalizer::Normalizer, type_ids::TypeIds,
  },
  type_aliases::type_id::TypeId,
};

impl Normalizer {
  pub fn union_extern_types_type_ids_type_ids(&mut self, heres: &mut TypeIds, theres: &TypeIds) {
    self.consume_fuel();

    for there in theres.begin() {
      let there: TypeId = there;
      self.union_extern_types_with_extern_type_type_ids_type_id(heres, there);
    }
  }

  pub fn union_extern_types_normalized_extern_type_normalized_extern_type(
    &mut self,
    heres: &mut NormalizedExternType,
    theres: &NormalizedExternType,
  ) {
    self.consume_fuel();

    for &there_ty in &theres.ordering {
      // Safety: NormalizedExternType 的 ordering 与 extern_types 由 push_pair
      // 成对写入，按 ordering 元素查表恒命中（cpp find->second 同位）。
      let there_negations = theres
        .extern_types
        .get(&there_ty)
        .expect("ordering 与 extern_types 成对登记，按 ordering 元素查表必命中");

      let mut insert = true;
      let mut idx = 0;
      while idx < heres.ordering.len() {
        let here_ty = heres.ordering[idx];
        // Safety: 同上，heres 侧 ordering/extern_types 成对登记恒命中。
        let here_negations = heres
          .extern_types
          .get_mut(&here_ty)
          .expect("ordering 与 extern_types 成对登记，按 ordering 元素查表必命中");

        if is_subclass_type_id_type_id(there_ty, here_ty) {
          let mut inserted = false;
          let mut n_idx = 0;
          while n_idx < here_negations.order.len() {
            let here_negate_ty = here_negations.order[n_idx];

            if is_subclass_type_id_type_id(there_ty, here_negate_ty) {
              inserted = true;
              heres.push_pair(there_ty, there_negations.clone());
              break;
            } else if is_subclass_type_id_type_id(here_negate_ty, there_ty) {
              inserted = true;
              here_negations.erase_type_id(here_negate_ty);
              // 对齐 cpp：erase 后不 break，继续检查其余 negation
              //（`nIt = hereNegations.erase(nIt)`，下标不动）。
            } else {
              n_idx += 1;
            }
          }

          if inserted {
            insert = false;
            break;
          }
        } else if is_subclass_type_id_type_id(here_ty, there_ty) {
          let mut negations = here_negations.clone();
          self.union_extern_types_type_ids_type_ids(&mut negations, there_negations);

          heres.ordering.remove(idx);
          heres.extern_types.remove(&here_ty);
          heres.push_pair(there_ty, negations);
          insert = false;
          break;
        } else if here_ty == there_ty {
          self.union_extern_types_type_ids_type_ids(here_negations, there_negations);
          insert = false;
          break;
        }

        idx += 1;
      }

      if insert {
        heres.push_pair(there_ty, there_negations.clone());

        if fflag::LuauExternTypesNormalizeWithShapes.get() {
          for &shape in &theres.shape_extensions.order {
            heres.shape_extensions.insert_type_id(shape);
          }
        }
      }
    }
  }
}
