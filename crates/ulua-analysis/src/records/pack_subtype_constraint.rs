use crate::type_aliases::type_pack_id::TypePackId;

#[derive(Debug, Clone)]
pub struct PackSubtypeConstraint {
  pub(crate) sub_pack: TypePackId,
  pub(crate) super_pack: TypePackId,
  pub(crate) returns: bool,
}
