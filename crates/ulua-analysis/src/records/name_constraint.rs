use alloc::{string::String, vec::Vec};

use crate::type_aliases::{type_id::TypeId, type_pack_id::TypePackId};

#[derive(Debug, Clone)]
pub struct NameConstraint {
  pub(crate) named_type: TypeId,
  pub(crate) name: String,
  pub(crate) synthetic: bool,
  pub(crate) type_parameters: Vec<TypeId>,
  pub(crate) type_pack_parameters: Vec<TypePackId>,
}
