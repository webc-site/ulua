use crate::{
  records::{subtyping::Subtyping, type_ids::TypeIds},
  type_aliases::type_id::TypeId,
};

impl Subtyping {
  pub fn maybe_update_bounds(
    &mut self,
    here: TypeId,
    there: TypeId,
    bounds_to_update: &mut TypeIds,
    first_bounds_to_check: &TypeIds,
    second_bounds_to_check: &TypeIds,
  ) {
    let mut bounds_changed = false;

    if !first_bounds_to_check.empty() {
      for t in first_bounds_to_check.order.iter() {
        let t = *t;
        if t != here {
          bounds_to_update.insert_type_id(t);
          bounds_changed = true;
        }
      }
    }

    if !bounds_changed && !second_bounds_to_check.empty() {
      for t in second_bounds_to_check.order.iter() {
        let t = *t;
        if t != here {
          bounds_to_update.insert_type_id(t);
          bounds_changed = true;
        }
      }
    }

    if !bounds_changed && here != there {
      bounds_to_update.insert_type_id(there);
    }
  }
}
