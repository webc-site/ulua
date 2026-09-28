use alloc::vec::Vec;

use crate::type_aliases::{type_id::TypeId, type_pack_id::TypePackId};

#[derive(Debug, Clone)]
pub struct TypeInstantiationConstraint {
  pub(crate) function_type: TypeId,
  pub(crate) placeholder_type: TypeId,
  pub(crate) type_arguments: Vec<TypeId>,
  pub(crate) type_pack_arguments: Vec<TypePackId>,
}
