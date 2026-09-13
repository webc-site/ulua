use crate::records::{normalizer::Normalizer, type_ids::TypeIds};

impl Normalizer {
  pub fn intersect_tables(&mut self, heres: &mut TypeIds, theres: &TypeIds) {
    self.consume_fuel();

    let mut tmp = TypeIds::new();
    for &here in &heres.order {
      for &there in &theres.order {
        if let Some(inter) = self.intersection_of_tables(here, there) {
          tmp.insert_type_id(inter);
        }
      }
    }

    heres.retain(&tmp);
    for ty in tmp.order {
      heres.insert_type_id(ty);
    }
  }
}
