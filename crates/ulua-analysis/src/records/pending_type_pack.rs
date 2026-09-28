use crate::records::type_pack_var::TypePackVar;

#[derive(Debug, Clone)]
pub struct PendingTypePack {
  pub(crate) pending: TypePackVar,
}
