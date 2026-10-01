use crate::type_aliases::{type_id::TypeId, type_pack_id::TypePackId};

#[derive(Debug, Clone)]
pub struct TarjanNode {
  pub(crate) ty: TypeId,
  pub(crate) tp: TypePackId,
  pub(crate) on_stack: bool,
  pub(crate) dirty: bool,
  pub(crate) lowlink: i32,
}
