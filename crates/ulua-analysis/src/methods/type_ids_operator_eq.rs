use crate::records::type_ids::TypeIds;

impl TypeIds {
  pub fn operator_eq(&self, there: &TypeIds) -> bool {
    if self.hash != there.hash {
      return false;
    }

    if self.order.len() != there.order.len() {
      return false;
    }

    for &ty in &self.order {
      if there.count(ty) == 0 {
        return false;
      }
    }

    true
  }
}
