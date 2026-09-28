use crate::type_aliases::type_pack_id::TypePackId;
#[derive(Debug, Clone, PartialEq)]
pub struct UnexpectedTypePackInSubtyping {
  pub(crate) tp: TypePackId,
}
