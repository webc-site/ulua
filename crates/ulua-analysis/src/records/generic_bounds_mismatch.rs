use alloc::{string::String, vec::Vec};

use crate::type_aliases::type_id::TypeId;
#[derive(Debug, Clone, PartialEq)]
pub struct GenericBoundsMismatch {
  pub(crate) generic_name: String,
  pub(crate) lower_bounds: Vec<TypeId>,
  pub(crate) upper_bounds: Vec<TypeId>,
}
