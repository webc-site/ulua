use crate::{records::type_ids::TypeIds, type_aliases::type_pack_ids::TypePackIds};

#[derive(Debug, Clone)]
pub struct UnblockedTypes {
  pub(crate) types: TypeIds,
  pub(crate) packs: TypePackIds,
}
