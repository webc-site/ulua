use crate::{records::type_ids::TypeIds, type_aliases::type_id::TypeId};

impl TypeIds {
  pub fn type_ids_initializer_list_type_id(&mut self, tys: &[TypeId]) {
    for ty in tys {
      self.insert_type_id(*ty);
    }
  }
}
