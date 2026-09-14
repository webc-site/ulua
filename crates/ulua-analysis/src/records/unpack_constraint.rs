use alloc::vec::Vec;

use crate::type_aliases::{type_id::TypeId, type_pack_id::TypePackId};

#[derive(Debug, Clone)]
pub struct UnpackConstraint {
  pub(crate) result_pack: Vec<TypeId>,
  pub(crate) source_pack: TypePackId,
}
