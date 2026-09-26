use crate::type_aliases::{type_id::TypeId, type_pack_id::TypePackId};
#[derive(Debug, Clone, PartialEq)]
pub struct AmbiguousFunctionCall {
  pub(crate) function: TypeId,
  pub(crate) arguments: TypePackId,
}
