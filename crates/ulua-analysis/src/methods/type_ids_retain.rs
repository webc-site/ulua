use crate::records::type_ids::TypeIds;

impl TypeIds {
  pub fn retain(&mut self, tys: &TypeIds) {
    let mut i = 0;
    while i < self.order.len() {
      let ty = self.order[i];
      if tys.count(ty) > 0 {
        i += 1;
      } else {
        self.erase_type_id(ty);
      }
    }
  }
}
