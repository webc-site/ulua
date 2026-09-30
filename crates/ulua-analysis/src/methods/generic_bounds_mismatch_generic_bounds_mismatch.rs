use crate::records::{generic_bounds_mismatch::GenericBoundsMismatch, type_ids::TypeIds};

impl GenericBoundsMismatch {
  pub fn new(
    generic_name: &str,
    mut lower_bound_set: TypeIds,
    mut upper_bound_set: TypeIds,
  ) -> Self {
    Self {
      generic_name: generic_name.to_string(),
      lower_bounds: TypeIds::take(&mut lower_bound_set),
      upper_bounds: TypeIds::take(&mut upper_bound_set),
    }
  }
}
