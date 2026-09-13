use crate::records::{normalizer::Normalizer, type_ids::TypeIds};

impl Normalizer {
  pub fn union_tables(&mut self, heres: &mut TypeIds, theres: &TypeIds) {
    self.consume_fuel();

    for there in theres.order.iter() {
      let there = *there;
      let builtin_types = unsafe { &*self.builtin_types };
      if there == builtin_types.table_type {
        heres.clear();
        heres.insert_type_id(there);
        return;
      } else {
        self.union_tables_with_table(heres, there);
      }
    }
  }
}
