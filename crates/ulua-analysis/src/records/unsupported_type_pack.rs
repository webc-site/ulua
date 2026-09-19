use crate::type_aliases::type_pack_id::TypePackId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnsupportedTypePack {
  pub(crate) pack: TypePackId,
}
