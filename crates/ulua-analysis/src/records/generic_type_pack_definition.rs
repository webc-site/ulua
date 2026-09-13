use crate::type_aliases::type_pack_id::TypePackId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GenericTypePackDefinition {
  pub(crate) tp: TypePackId,
  pub(crate) default_value: Option<TypePackId>,
}
