use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  records::{normalizer::Normalizer, type_ids::TypeIds},
  type_aliases::{seen_table_prop_pairs::SeenTablePropPairs, type_id::TypeId},
};

impl Normalizer {
  pub fn intersect_tables_with_table(
    &mut self,
    heres: &mut TypeIds,
    there: TypeId,
    _seen_table_prop_pairss: &mut SeenTablePropPairs,
    _seen_set_typeses: &mut DenseHashSet<TypeId>,
  ) {
    self.consume_fuel();

    let mut tmp = TypeIds::new();
    let heres_clone = heres.clone();
    for here in heres_clone.order {
      if let Some(inter) = self.intersection_of_tables(here, there) {
        tmp.insert_type_id(inter);
      }
    }
    heres.retain(&tmp);
    for ty in tmp.order {
      heres.insert_type_id(ty);
    }
  }
}
