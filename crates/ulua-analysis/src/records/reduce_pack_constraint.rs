use crate::type_aliases::type_pack_id::TypePackId;

#[derive(Debug, Clone)]
pub struct ReducePackConstraint {
  pub(crate) tp: TypePackId,
}
